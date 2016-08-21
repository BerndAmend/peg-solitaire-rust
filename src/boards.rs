use board::*;
use boardset::*;
use generated::*;

use std::sync::{Arc, RwLock};
use std::thread;

const TRANSMIT_SIZE: usize = 1 << 19;

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
        // println!(", found {:?}", current.get_info());
    }

    println!("number of possible fields {}",
            solution.iter().fold(0, |o, i| o + i.len()));

    solution
}

pub fn solve_parallel(start: State) -> Vec<Arc<RwLock<BoardSet>>> {
    let thread_count = 4;
    assert_eq!(start.count_ones() as usize, PEGS - 1);

    let mut solution: Vec<Arc<RwLock<BoardSet>>> = Vec::new();

    let mut current = Arc::new(RwLock::new(BoardSet::new()));
    current.write().unwrap().insert(normalize(start));

    while !current.read().unwrap().is_empty() {
        print!("search fields with {} removed pegs", solution.len() + 2);
        let next = Arc::new(RwLock::new(BoardSet::new()));
        {
            let mut threads = Vec::new();
            for i in 0..thread_count {
                let current = current.clone();
                let next = next.clone();
                threads.push(thread::spawn(move || {
                    let cur = current.read().unwrap();
                    let slice = cur.chunks(cur.data_len() / thread_count + 1).nth(i).unwrap();

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

            for i in threads {
                let _ = i.join();
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
