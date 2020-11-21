use ocl::{core, Buffer, ProQue};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::mem;

use crate::description::{BoardSet, Description, Solver, State, EMPTY_STATE};
use crate::utils::Stopwatch;

pub struct OpenCLSolver {
    desc: Description,
}

impl OpenCLSolver {
    pub fn new(desc: &Description) -> OpenCLSolver {
        OpenCLSolver { desc: desc.clone() }
    }
}

impl OpenCLSolver {
    fn generate_cl_code(&self, buckets: usize) -> String {
        let normalize_code = self
            .desc
            .transformations
            .iter()
            .map(|trans| {
                format!(
                    "normalized_state = min(normalized_state,{});",
                    &trans
                        .iter()
                        .map(|(&shift, pos)| {
                            if shift == 0 {
                                format!("(state & {}UL)", &pos.to_string())
                            } else {
                                format!(
                                    "(state & {}UL){}{}",
                                    &pos.to_string(),
                                    if shift > 0 { " << " } else { " >> " },
                                    if shift > 0 { shift } else { i32::abs(shift) }
                                )
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" | ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "
            #pragma OPENCL EXTENSION cl_khr_int64_base_atomics : enable
            #define MASKSIZE {}
            #define OUTPUTSIZE {}

            ptrdiff_t get_index_from_state(ulong v) {{
                v *= 0x85ebca6bUL;
                v ^= v >> 13;
                return (ptrdiff_t)(v & (ulong)(OUTPUTSIZE - 1));
            }}

            __kernel void solve(__global ulong const * const previous,
                            __global ulong* const next,
                            __constant ulong* movemask,
                            __constant ulong* checkmask1,
                            __constant ulong* checkmask2) {{
                ulong const field = previous[get_global_id(0)];

                if(field == 0UL)
                    return;

                for(uint i=0;i<MASKSIZE;++i) {{
                    ulong const mm = movemask[i];
                    ulong const v = field & mm;
                    if(v == checkmask1[i] || v == checkmask2[i]) {{
                        ulong const state = field ^ mm;
                        ulong normalized_state = state;
                        // normalize_code
                        {}

                        ptrdiff_t outpos = get_index_from_state(normalized_state);
                        ulong to_insert = normalized_state;
                        for(int j=0;j<1000;++j) {{
                            to_insert = atomic_xchg(next+outpos, to_insert);
                            if(to_insert == 0UL || to_insert == normalized_state) {{
                                break;
                            }}
                            outpos+=1;
                        }}
                    }}
                }}
            }}",
            self.desc.move_mask.len(),
            buckets,
            normalize_code
        )
    }
}

impl Solver for OpenCLSolver {
    fn solve(&self, start: State) -> Vec<BoardSet> {
        let buckets: usize = 1 << 23;
        let size: usize = buckets + 10000;
        // let devices = ocl::Device::list_all(ocl::Platform::first().unwrap()).unwrap();
        // for dev in devices {
        //     println!("{}", dev.to_string());
        // }
        let mut ocl_pq = ProQue::builder()
            .src(self.generate_cl_code(buckets))
            .build()
            .expect("Build ProQue");
        let t = Stopwatch::default();
        assert_eq!(start.count_ones() as usize, self.desc.pegs - 1);

        let ocl_movemask = unsafe {
            Buffer::builder()
                .queue(ocl_pq.queue().clone())
                .flags(core::MEM_READ_ONLY)
                .len([self.desc.move_mask.len()].clone())
                .use_host_slice(&self.desc.move_mask)
                .build()
                .unwrap()
        };
        let ocl_checkmask1 = unsafe {
            Buffer::builder()
                .queue(ocl_pq.queue().clone())
                .flags(core::MEM_READ_ONLY)
                .len([self.desc.check_mask1.len()].clone())
                .use_host_slice(&self.desc.check_mask1)
                .build()
                .unwrap()
        };
        let ocl_checkmask2 = unsafe {
            Buffer::builder()
                .queue(ocl_pq.queue().clone())
                .flags(core::MEM_READ_ONLY)
                .len([self.desc.check_mask2.len()].clone())
                .use_host_slice(&self.desc.check_mask2)
                .build()
                .unwrap()
        };

        let mut solution: Vec<BoardSet> = Vec::new();

        ocl_pq.set_dims([size]);
        let mut current_buffer: Buffer<State> = ocl_pq.create_buffer().unwrap();
        let mut next_buffer: Buffer<State> = ocl_pq.create_buffer().unwrap();

        {
            let mut current = vec![EMPTY_STATE; size];
            current[0] = start;
            current_buffer.write(&current).enq().unwrap();
            solution.push(
                current
                    .iter()
                    .cloned()
                    .filter(|&x| x != EMPTY_STATE)
                    .collect(),
            );
        }

        ocl_pq.set_dims([size]);
        while solution.len() < self.desc.pegs {
            let t = Stopwatch::default();

            let i = solution.len() + 1;
            print!("search fields with {} removed pegs", i);

            next_buffer.cmd().fill(EMPTY_STATE, None).enq().unwrap();

            unsafe {
                ocl_pq
                    .kernel_builder("solve")
                    .arg(&current_buffer)
                    .arg(&next_buffer)
                    .arg(&ocl_movemask)
                    .arg(&ocl_checkmask1)
                    .arg(&ocl_checkmask2)
                    .build()
                    .unwrap()
                    .enq()
                    .unwrap();
            }

            let mut next = vec![EMPTY_STATE; size];
            next_buffer.read(&mut next).enq().unwrap();
            let new: BoardSet = next.iter().cloned().filter(|&x| x != EMPTY_STATE).collect();
            println!(", found {} fields in {}", new.len(), t);
            solution.push(new);

            mem::swap(&mut current_buffer, &mut next_buffer);
        }

        println!(
            "number of possible fields {} in {}",
            solution.par_iter().map(|i| i.len()).sum::<usize>(),
            t
        );

        solution
    }
}
