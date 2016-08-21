use boardset::*;
use description::{Description, State, EMPTY_STATE};

pub trait Board {
    const PEGS: usize;
    const SIZE: usize;
    const MOVEMASK: &'static [State];
    const CHECKMASK1: &'static [State];
    const CHECKMASK2: &'static [State];

    fn description() -> Description;

    fn normalize(state: State) -> State;
    fn equivalent_fields(state: State) -> [State; 8];

    const TRANSMIT_SIZE: usize = 1 << 19;

    fn solve(start: State) -> Vec<BoardSet> {
        assert_eq!(start.count_ones() as usize, Self::PEGS - 1);

        let mut solution: Vec<BoardSet> = vec![];

        let mut current = BoardSet::new();
        current.insert(Self::normalize(start));

        let mut tmp = Vec::with_capacity(Self::TRANSMIT_SIZE);
        while !current.is_empty() {
            print!("search fields with {} removed pegs", solution.len() + 2);
            let mut next = BoardSet::new();
            tmp.clear();
            for &field in current.data.iter().filter(|&x| *x != EMPTY_STATE) {
                for i in 0..Self::SIZE {
                    let v = field & Self::MOVEMASK[i];
                    if v == Self::CHECKMASK1[i] || v == Self::CHECKMASK2[i] {
                        tmp.push(Self::normalize(field ^ Self::MOVEMASK[i]));
                    }
                }
                if tmp.len() > Self::TRANSMIT_SIZE - Self::SIZE {
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

    fn solve_parallel(start: State) -> Vec<BoardSet> {
        use std::thread;
        use std::sync::{Arc, RwLock};

        let thread_count = 4;
        assert_eq!(start.count_ones() as usize, Self::PEGS - 1);

        let mut solution: Vec<Arc<RwLock<BoardSet>>> = Vec::new();

        {
            let mut current = Arc::new(RwLock::new(BoardSet::new()));
            current.write().unwrap().insert(Self::normalize(start));

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
                            let slice =
                                cur.chunks(cur.data_len() / thread_count + 1).nth(i).unwrap();

                            let mut tmp = Vec::with_capacity(Self::TRANSMIT_SIZE);

                            for &field in slice.iter().filter(|&x| *x != EMPTY_STATE) {
                                for i in 0..Self::SIZE {
                                    let v = field & Self::MOVEMASK[i];
                                    if v == Self::CHECKMASK1[i] || v == Self::CHECKMASK2[i] {
                                        tmp.push(Self::normalize(field ^ Self::MOVEMASK[i]));
                                    }
                                }

                                if tmp.len() / 3 > Self::TRANSMIT_SIZE / 4 {
                                    match next.try_write() {
                                        Ok(mut t) => {
                                            t.insert_all_abort_on_empty_state(&tmp);
                                            tmp.clear();
                                        }
                                        Err(_) => {
                                            if tmp.len() > Self::TRANSMIT_SIZE - Self::SIZE {
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
        }

        let mut result = Vec::new();
        for x in solution {
            match Arc::try_unwrap(x) {
                Ok(d) => result.push(d.into_inner().unwrap()),
                _ => unreachable!(),
            }
        }

        println!("number of possible fields {}",
                 result.iter().fold(0, |o, i| o + i.len()));

        result
    }

    fn possible_start_fields() -> BoardSet {
        let mut set = BoardSet::with_capacity(Self::PEGS);

        let base = (1u64 << (Self::PEGS + 1)) - 1u64;

        for i in 0..Self::PEGS {
            set.fast_insert(Self::normalize(base ^ (1u64 << i)));
        }

        set
    }
}
