use board::*;
use boardset::*;

const TRANSMIT_SIZE: usize = 1 << 19;

// Solver
pub fn solve<T: Board>(start: State) -> Vec<BoardSet> {
    assert_eq!(start.count_ones() as usize, T::PEGS - 1);

    let mut solution: Vec<BoardSet> = vec![];

    let mut current = BoardSet::new();
    current.insert(T::normalize(start));

    let mut tmp = Vec::with_capacity(TRANSMIT_SIZE);
    while !current.is_empty() {
        print!("search fields with {} removed pegs", solution.len() + 2);
        let mut next = BoardSet::new();
        tmp.clear();
        for &field in current.data.iter().filter(|&x| *x != EMPTY_STATE) {
            for i in 0..T::SIZE {
                let v = field & T::MOVEMASK[i];
                if v == T::CHECKMASK1[i] || v == T::CHECKMASK2[i] {
                    tmp.push(T::normalize(field ^ T::MOVEMASK[i]));
                }
            }
            if tmp.len() > TRANSMIT_SIZE - T::SIZE {
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

pub fn solve_parallel<T: Board>(start: State) -> Vec<BoardSet> {
    use std::thread;
    use std::sync::{Arc, RwLock};

    let thread_count = 4;
    assert_eq!(start.count_ones() as usize, T::PEGS - 1);

    let mut solution: Vec<Arc<RwLock<BoardSet>>> = Vec::new();

    {
        let mut current = Arc::new(RwLock::new(BoardSet::new()));
        current.write().unwrap().insert(T::normalize(start));

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
                            for i in 0..T::SIZE {
                                let v = field & T::MOVEMASK[i];
                                if v == T::CHECKMASK1[i] || v == T::CHECKMASK2[i] {
                                    tmp.push(T::normalize(field ^ T::MOVEMASK[i]));
                                }
                            }

                            if tmp.len() / 3 > TRANSMIT_SIZE / 4 {
                                match next.try_write() {
                                    Ok(mut t) => {
                                        t.insert_all_abort_on_empty_state(&tmp);
                                        tmp.clear();
                                    }
                                    Err(_) => {
                                        if tmp.len() > TRANSMIT_SIZE - T::SIZE {
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

pub fn possible_start_fields<T: Board>() -> BoardSet {
    let mut set = BoardSet::with_capacity(T::PEGS);

    let base = (1u64 << (T::PEGS + 1)) - 1u64;

    for i in 0..T::PEGS {
        set.fast_insert(T::normalize(base ^ (1u64 << i)));
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;
    use generated::EnglishBoard;

    #[test]
    fn solve_test() {
        assert!(solve::<EnglishBoard>(8589869055u64).iter()
                .fold(0, |o, i| o + i.len()) == 23475688);
    }

    #[test]
    fn solve_parallel_test() {
        assert!(solve_parallel::<EnglishBoard>(8589869055u64).iter()
                .fold(0, |o, i| o + i.len()) == 23475688);
    }
}