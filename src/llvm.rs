use crate::description::{BoardSet, Description, Solver, State, Transformation};
use crate::utils::Stopwatch;
use rayon::iter::*;
use std::mem::transmute_copy;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::IntPredicate;
use inkwell::OptimizationLevel;

type NormalizeFunc = extern "C" fn(u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_normalize(&self, transformations: &[Transformation]) -> Option<NormalizeFunc> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into()], false);
        let function = self.module.add_function("norm", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let b = &self.builder;

        let state = function.get_nth_param(0)?.into_int_value();

        let mut norm = state;

        for transformation in transformations.iter() {
            let mut temp_state = i64_type.const_int(0, false);
            for (&shift, &pos) in transformation {
                let mut masked_state = b.build_and(state, i64_type.const_int(pos, false), "and");

                if shift > 0 {
                    masked_state = b.build_left_shift(
                        masked_state,
                        i64_type.const_int(shift as u64, false),
                        "shift",
                    );
                } else if shift < 0 {
                    masked_state = b.build_right_shift(
                        masked_state,
                        i64_type.const_int(i32::abs(shift) as u64, false),
                        false,
                        "shift",
                    );
                }

                temp_state = b.build_or(temp_state, masked_state, "or");
            }

            let compare = b.build_int_compare(IntPredicate::ULT, norm, temp_state, "compare");
            norm = b
                .build_select(compare, norm, temp_state, "select")
                .into_int_value();
        }

        self.builder.build_return(Some(&norm));

        unsafe {
            let address = self.execution_engine.get_function_address("norm").unwrap();
            transmute_copy(&address)
        }
    }
}

pub struct LLVMSolver {
    desc: Description,
}

impl LLVMSolver {
    pub fn new(desc: &Description) -> LLVMSolver {
        LLVMSolver { desc: desc.clone() }
    }
}

impl Solver for LLVMSolver {
    fn solve(&self, start: State) -> Vec<BoardSet> {
        let t = Stopwatch::default();
        assert_eq!(start.count_ones() as usize, self.desc.pegs - 1);

        let context = Context::create();
        let module = context.create_module("normalize");
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();
        let codegen = CodeGen {
            context: &context,
            module,
            builder: context.create_builder(),
            execution_engine,
        };

        let normalize = codegen
            .jit_compile_normalize(&self.desc.transformations)
            .unwrap();

        let move_mask: &[State] = &self.desc.move_mask;
        let check_mask1: &[State] = &self.desc.check_mask1;
        let check_mask2: &[State] = &self.desc.check_mask2;
        let mask_size = move_mask.len();

        let mut solution: Vec<BoardSet> = vec![];
        let mut current = BoardSet::default();
        current.insert(normalize(start));

        while !current.is_empty() {
            print!("search fields with {} removed pegs", solution.len() + 2);
            let t = Stopwatch::default();
            let next = current
                .par_iter()
                .map(|field| {
                    let mut tmp = BoardSet::default();
                    for i in 0..mask_size {
                        let v = field & move_mask[i];
                        if v == check_mask1[i] || v == check_mask2[i] {
                            tmp.insert(normalize(field ^ move_mask[i]));
                        }
                    }
                    tmp
                })
                .reduce(
                    || BoardSet::default(),
                    |mut n, tmp| {
                        for x in tmp {
                            n.insert(x);
                        }
                        n
                    },
                );

            solution.push(current);
            current = next;
            println!(", found {} fields in {}", current.len(), t);
        }

        println!(
            "number of possible fields {} in {}",
            solution.par_iter().map(|i| i.len()).sum::<usize>(),
            t
        );

        solution
    }
}
