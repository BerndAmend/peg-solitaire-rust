use crate::description::{BoardSet, Description, Solver, State};
use crate::utils::Stopwatch;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct NaiveSolver {
    desc: Description,
}

impl NaiveSolver {
    pub fn new(desc: &Description) -> NaiveSolver {
        NaiveSolver { desc: desc.clone() }
    }
}

impl Solver for NaiveSolver {
    fn solve(&self, start: State) -> Vec<BoardSet> {
        let t = Stopwatch::default();
        assert_eq!(start.count_ones() as usize, self.desc.pegs - 1);

        let move_mask: &[State] = &self.desc.move_mask;
        let check_mask1: &[State] = &self.desc.check_mask1;
        let check_mask2: &[State] = &self.desc.check_mask2;
        let mask_size = move_mask.len();

        let mut solution: Vec<BoardSet> = vec![];
        let mut current = BoardSet::default();
        current.insert(self.desc.normalize(start));

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
                            tmp.insert(self.desc.normalize(field ^ move_mask[i]));
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
