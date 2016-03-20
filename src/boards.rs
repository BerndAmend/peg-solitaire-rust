extern crate thread_scoped;
use board::*;
use boardset::*;

use std::sync::{Arc, RwLock};

const TRANSMIT_SIZE: usize = 1 << 19;

// generated
const PEGS: usize = 33;

const SIZE: usize = 38;
const MOVEMASK: [State; SIZE] = [7516192768u64,
                                 4848615424u64,
                                 2424307712u64,
                                 1212153856u64,
                                 939524096u64,
                                 553779200u64,
                                 276889600u64,
                                 138444800u64,
                                 117440512u64,
                                 67637248u64,
                                 58720256u64,
                                 33818624u64,
                                 29360128u64,
                                 16909312u64,
                                 14680064u64,
                                 8454656u64,
                                 7340032u64,
                                 4227328u64,
                                 2113664u64,
                                 1056832u64,
                                 917504u64,
                                 458752u64,
                                 229376u64,
                                 132128u64,
                                 114688u64,
                                 66064u64,
                                 57344u64,
                                 33032u64,
                                 7168u64,
                                 3584u64,
                                 1792u64,
                                 1060u64,
                                 896u64,
                                 530u64,
                                 448u64,
                                 265u64,
                                 56u64,
                                 7u64];
const CHECKMASK1: [State; SIZE] = [6442450944u64,
                                   4831838208u64,
                                   2415919104u64,
                                   1207959552u64,
                                   805306368u64,
                                   553648128u64,
                                   276824064u64,
                                   138412032u64,
                                   100663296u64,
                                   67633152u64,
                                   50331648u64,
                                   33816576u64,
                                   25165824u64,
                                   16908288u64,
                                   12582912u64,
                                   8454144u64,
                                   6291456u64,
                                   4227072u64,
                                   2113536u64,
                                   1056768u64,
                                   786432u64,
                                   393216u64,
                                   196608u64,
                                   132096u64,
                                   98304u64,
                                   66048u64,
                                   49152u64,
                                   33024u64,
                                   6144u64,
                                   3072u64,
                                   1536u64,
                                   1056u64,
                                   768u64,
                                   528u64,
                                   384u64,
                                   264u64,
                                   48u64,
                                   6u64];
const CHECKMASK2: [State; SIZE] = [3221225472u64,
                                   553648128u64,
                                   276824064u64,
                                   138412032u64,
                                   402653184u64,
                                   16908288u64,
                                   8454144u64,
                                   4227072u64,
                                   50331648u64,
                                   528384u64,
                                   25165824u64,
                                   264192u64,
                                   12582912u64,
                                   132096u64,
                                   6291456u64,
                                   66048u64,
                                   3145728u64,
                                   33024u64,
                                   16512u64,
                                   8256u64,
                                   393216u64,
                                   196608u64,
                                   98304u64,
                                   1056u64,
                                   49152u64,
                                   528u64,
                                   24576u64,
                                   264u64,
                                   3072u64,
                                   1536u64,
                                   768u64,
                                   36u64,
                                   384u64,
                                   18u64,
                                   192u64,
                                   9u64,
                                   24u64,
                                   3u64];

#[inline(always)]
fn normalize(state: State) -> State {
    use std::cmp::min;
    let p0 = (state & 8128u64) << 14 | (state & 1040384u64) | (state & 133169152u64) >> 14 |
             (state & 7516192768u64) >> 30 | (state & 939524096u64) >> 24 |
             (state & 56u64) << 24 | (state & 7u64) << 30;
    let p1 = (state & 67637248u64) >> 6 | (state & 1056832u64) << 6 | (state & 2424373778u64) |
             (state & 1212186889u64) << 2 |
             (state & 4848747556u64) >> 2 | (state & 2113664u64) << 4 |
             (state & 33818624u64) >> 4;
    let p2 = (state & 1024u64) << 12 | (state & 33554432u64) >> 18 | (state & 16u64) << 24 |
             (state & 1073741824u64) >> 28 | (state & 16384u64) << 4 |
             (state & 2u64) << 30 |
             (state & 2147483648u64) >> 30 | (state & 65536u64) |
             (state & 524288u64) >> 6 | (state & 1048576u64) >> 8 |
             (state & 2097152u64) >> 10 | (state & 8192u64) << 6 |
             (state & 32u64) << 22 | (state & 262144u64) >> 4 |
             (state & 4096u64) << 8 | (state & 512u64) << 14 |
             (state & 32768u64) << 2 | (state & 16777216u64) >> 16 |
             (state & 131072u64) >> 2 |
             (state & 67108864u64) >> 20 |
             (state & 8u64) << 26 |
             (state & 1u64) << 32 | (state & 4194304u64) >> 12 |
             (state & 4294967296u64) >> 32 |
             (state & 64u64) << 20 | (state & 256u64) << 16 | (state & 128u64) << 18 |
             (state & 2048u64) << 10 |
             (state & 536870912u64) >> 26 |
             (state & 4u64) << 28 | (state & 268435456u64) >> 24 |
             (state & 134217728u64) >> 22 | (state & 8388608u64) >> 14;
    let p3 = (state & 1074790400u64) >> 18 | (state & 525314u64) << 12 | (state & 4100u64) << 18 |
             (state & 16843008u64) |
             (state & 262160u64) << 10 | (state & 268451840u64) >> 10 |
             (state & 2080u64) << 16 | (state & 33554440u64) << 4 |
             (state & 536871040u64) >> 4 | (state & 4303388736u64) >> 6 |
             (state & 136314880u64) >> 16 | (state & 67240449u64) << 6 |
             (state & 2151686144u64) >> 12;
    let p4 = (state & 33554688u64) << 2 | (state & 545259520u64) >> 8 | (state & 131072u64) << 6 |
             (state & 16777344u64) >> 2 |
             (state & 524289u64) << 12 | (state & 2050u64) << 18 |
             (state & 520u64) << 8 | (state & 262144u64) << 10 |
             (state & 16384u64) >> 10 | (state & 64u64) >> 4 |
             (state & 2149580800u64) >> 18 |
             (state & 4u64) << 24 | (state & 32768u64) >> 6 | (state & 4128u64) << 20 |
             (state & 1040u64) << 14 |
             (state & 135266304u64) >> 20 | (state & 65536u64) |
             (state & 4294975488u64) >> 12 |
             (state & 67108864u64) << 4 | (state & 1073741824u64) >> 24 |
             (state & 272629760u64) >> 14;
    let p5 = (state & 133120u64) >> 8 | (state & 4u64) << 4 | (state & 17039360u64) >> 14 |
             (state & 4328521728u64) >> 20 | (state & 537395200u64) >> 18 |
             (state & 65536u64) |
             (state & 64u64) << 24 | (state & 67108864u64) >> 24 |
             (state & 268435456u64) >> 10 | (state & 1073741824u64) >> 4 |
             (state & 8388608u64) >> 6 | (state & 16640u64) << 14 |
             (state & 4194336u64) << 2 | (state & 8200u64) << 18 |
             (state & 2147487744u64) >> 12 |
             (state & 2129920u64) << 8 | (state & 1048578u64) << 12 |
             (state & 16u64) << 10 |
             (state & 134218752u64) >> 2 | (state & 129u64) << 20 |
             (state & 512u64) << 6;
    let p6 = (state & 570425344u64) >> 22 | (state & 134219776u64) >> 6 | (state & 256u64) << 16 |
             (state & 2148007936u64) >> 18 |
             (state & 33280u64) << 8 | (state & 8519680u64) >> 8 |
             (state & 1073745920u64) >> 10 | (state & 136u64) << 22 |
             (state & 2097184u64) << 6 | (state & 1048580u64) << 10 |
             (state & 16400u64) << 14 | (state & 8194u64) << 18 |
             (state & 4362076160u64) >> 26 | (state & 268697600u64) >> 14 |
             (state & 16777216u64) >> 16 | (state & 4260864u64) |
             (state & 65u64) << 26;
    min(min(min(state, p0), min(p1, p2)),
        min(min(p3, p4), min(p5, p6)))
}

fn equivalent_fields(state: State) -> [State; 8] {
    let mut n = [EMPTY_STATE; 8];
    n[0] = state;
    n[1] = (state & 8128u64) << 14 | (state & 1040384u64) | (state & 133169152u64) >> 14 |
           (state & 7516192768u64) >> 30 | (state & 939524096u64) >> 24 |
           (state & 56u64) << 24 | (state & 7u64) << 30;
    n[2] = (state & 67637248u64) >> 6 | (state & 1056832u64) << 6 | (state & 2424373778u64) |
           (state & 1212186889u64) << 2 | (state & 4848747556u64) >> 2 |
           (state & 2113664u64) << 4 | (state & 33818624u64) >> 4;
    n[3] = (state & 1024u64) << 12 | (state & 33554432u64) >> 18 | (state & 16u64) << 24 |
           (state & 1073741824u64) >> 28 | (state & 16384u64) << 4 |
           (state & 2u64) << 30 | (state & 2147483648u64) >> 30 | (state & 65536u64) |
           (state & 524288u64) >> 6 | (state & 1048576u64) >> 8 |
           (state & 2097152u64) >> 10 | (state & 8192u64) << 6 |
           (state & 32u64) << 22 | (state & 262144u64) >> 4 |
           (state & 4096u64) << 8 | (state & 512u64) << 14 | (state & 32768u64) << 2 |
           (state & 16777216u64) >> 16 |
           (state & 131072u64) >> 2 | (state & 67108864u64) >> 20 |
           (state & 8u64) << 26 | (state & 1u64) << 32 | (state & 4194304u64) >> 12 |
           (state & 4294967296u64) >> 32 | (state & 64u64) << 20 |
           (state & 256u64) << 16 | (state & 128u64) << 18 |
           (state & 2048u64) << 10 | (state & 536870912u64) >> 26 |
           (state & 4u64) << 28 |
           (state & 268435456u64) >> 24 |
           (state & 134217728u64) >> 22 | (state & 8388608u64) >> 14;
    n[4] = (state & 1074790400u64) >> 18 | (state & 525314u64) << 12 | (state & 4100u64) << 18 |
           (state & 16843008u64) |
           (state & 262160u64) << 10 | (state & 268451840u64) >> 10 |
           (state & 2080u64) << 16 | (state & 33554440u64) << 4 |
           (state & 536871040u64) >> 4 | (state & 4303388736u64) >> 6 |
           (state & 136314880u64) >> 16 | (state & 67240449u64) << 6 |
           (state & 2151686144u64) >> 12;
    n[5] = (state & 33554688u64) << 2 | (state & 545259520u64) >> 8 | (state & 131072u64) << 6 |
           (state & 16777344u64) >> 2 |
           (state & 524289u64) << 12 | (state & 2050u64) << 18 |
           (state & 520u64) << 8 | (state & 262144u64) << 10 |
           (state & 16384u64) >> 10 | (state & 64u64) >> 4 |
           (state & 2149580800u64) >> 18 | (state & 4u64) << 24 |
           (state & 32768u64) >> 6 | (state & 4128u64) << 20 |
           (state & 1040u64) << 14 | (state & 135266304u64) >> 20 | (state & 65536u64) |
           (state & 4294975488u64) >> 12 | (state & 67108864u64) << 4 |
           (state & 1073741824u64) >> 24 | (state & 272629760u64) >> 14;
    n[6] = (state & 133120u64) >> 8 | (state & 4u64) << 4 | (state & 17039360u64) >> 14 |
           (state & 4328521728u64) >> 20 | (state & 537395200u64) >> 18 |
           (state & 65536u64) | (state & 64u64) << 24 | (state & 67108864u64) >> 24 |
           (state & 268435456u64) >> 10 | (state & 1073741824u64) >> 4 |
           (state & 8388608u64) >> 6 | (state & 16640u64) << 14 |
           (state & 4194336u64) << 2 | (state & 8200u64) << 18 |
           (state & 2147487744u64) >> 12 | (state & 2129920u64) << 8 |
           (state & 1048578u64) << 12 |
           (state & 16u64) << 10 | (state & 134218752u64) >> 2 |
           (state & 129u64) << 20 | (state & 512u64) << 6;
    n[7] = (state & 570425344u64) >> 22 | (state & 134219776u64) >> 6 | (state & 256u64) << 16 |
           (state & 2148007936u64) >> 18 |
           (state & 33280u64) << 8 | (state & 8519680u64) >> 8 |
           (state & 1073745920u64) >> 10 | (state & 136u64) << 22 |
           (state & 2097184u64) << 6 | (state & 1048580u64) << 10 |
           (state & 16400u64) << 14 | (state & 8194u64) << 18 |
           (state & 4362076160u64) >> 26 | (state & 268697600u64) >> 14 |
           (state & 16777216u64) >> 16 | (state & 4260864u64) | (state & 65u64) << 26;
    n
}

// Solver
pub fn solve(start: State) -> Vec<BoardSet> {
    assert_eq!(start.count_ones() as usize, PEGS - 1);

    let mut solution: Vec<BoardSet> = vec![];

    let mut current = BoardSet::new();
    current.insert(normalize(start));

    let mut tmp = Vec::with_capacity(TRANSMIT_SIZE);
    while !current.is_empty() {
        print!("search fields with {} removed pegs", solution.len() + 2);
        let mut next = BoardSet::new();
        tmp.clear();
        for &field in current.data.iter().filter(|&x| *x != EMPTY_STATE) {
            next.reserve(SIZE);
            for i in 0..SIZE {
                let v = field & MOVEMASK[i];
                if v == CHECKMASK1[i] || v == CHECKMASK2[i] {
                    tmp.push(normalize(field ^ MOVEMASK[i]));
                }
            }
            if tmp.len() > TRANSMIT_SIZE - SIZE {
                next.insert_all_abort_on_empty_state(&tmp);
                tmp.clear();
            }
        }

        if tmp.len() > 0 {
            next.insert_all_abort_on_empty_state(&tmp);
        }

        solution.push(current);
        current = next;
        println!(", found {} fields", current.len());
        // println!(", found {:?}", current.read().unwrap().get_info());
    }

    solution
}

pub fn solve_parallel(start: State) -> Vec<Arc<RwLock<BoardSet>>> {
    let thread_count = 8;
    assert_eq!(start.count_ones() as usize, PEGS - 1);

    let mut solution: Vec<Arc<RwLock<BoardSet>>> = vec![];

    let mut current = Arc::new(RwLock::new(BoardSet::new()));
    current.write().unwrap().insert(normalize(start));

    while !current.read().unwrap().is_empty() {
        print!("search fields with {} removed pegs", solution.len() + 2);
        let next = Arc::new(RwLock::new(BoardSet::new()));
        {
            let cur = current.read().unwrap();
            {
                let mut threads = Vec::new();
                let chunks = cur.chunks(cur.data_len() / thread_count);
                for slice in chunks {
                    let next = next.clone();
                    unsafe {
                        threads.push(thread_scoped::scoped(move || {
                            let mut tmp = Vec::with_capacity(TRANSMIT_SIZE);

                            for &field in slice.iter().filter(|&x| *x != EMPTY_STATE) {
                                for i in 0..SIZE {
                                    let v = field & MOVEMASK[i];
                                    if v == CHECKMASK1[i] || v == CHECKMASK2[i] {
                                        tmp.push(normalize(field ^ MOVEMASK[i]));
                                    }
                                }

                                if tmp.len() / 3 > TRANSMIT_SIZE / 4 {
                                    match next.try_write() {
                                        Ok(mut t) => {
                                            t.insert_all_abort_on_empty_state(&tmp);
                                            tmp.clear();
                                        }
                                        Err(_) => {
                                            if tmp.len() > TRANSMIT_SIZE - SIZE {
                                                let mut t = next.write().unwrap();
                                                t.insert_all_abort_on_empty_state(&tmp);
                                                tmp.clear();
                                            }
                                        }
                                    };
                                }
                            }

                            let mut t = next.write().unwrap();
                            t.insert_all_abort_on_empty_state(&tmp);
                        }));
                    }
                }

                for i in threads {
                    let _ = i.join();
                }
            }
        }

        solution.push(current);
        current = next;
        println!(", found {} fields", current.read().unwrap().len());
        // println!(", found {:?}", current.read().unwrap().get_info());
    }

    println!("number of possible fields {}",
             solution.iter().fold(0, |o, i| o + i.read().unwrap().len()));

    solution
}

pub fn possible_start_fields() -> BoardSet {
    let mut set = BoardSet::with_capacity(PEGS);

    let base = (1u64 << (PEGS + 1)) - 1u64;

    for i in 0..PEGS {
        set.fast_insert(normalize(base ^ (1u64 << i)));
    }

    set
}
