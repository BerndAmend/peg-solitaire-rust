mod board;
mod boardset;
mod boards;
mod solver;
mod generator;

use std::rc::Rc;
use board::Board;

fn main() {
    println!("peg-solitaire rust edition");

    if false {
        let descriptions = [board::Description::new("English", "..ooo..\n\
                                                ..ooo..\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                ..ooo..\n\
                                                ..ooo..",
                                                &[board::MoveDirections::Horizontal, board::MoveDirections::Vertical]).unwrap(),
                            board::Description::new("European", "..ooo..\n\
                                                .ooooo.\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                .ooooo.\n\
                                                ..ooo..",
                                                &[board::MoveDirections::Horizontal, board::MoveDirections::Vertical]).unwrap(),
                            board::Description::new("Holes15", "o....\n\
                                               oo...\n\
                                               ooo..\n\
                                               oooo.\n\
                                               ooooo",
                                               &[board::MoveDirections::Horizontal, board::MoveDirections::Vertical, board::MoveDirections::LeftDiagonal, board::MoveDirections::RightDiagonal]).unwrap()
                            ];

        for x in descriptions.iter() {
            println!("{}", generator::get_rust_code(&x));
        }
    } else {
        let board = Rc::new(boards::English::new());
        let mut sol = solver::Solver::new(board.clone());

        let mut i = 0;
        let mut start_field = board::EMPTY_STATE;
        let start_fields = sol.possible_start_fields();
        start_fields.foreach(|x| {
                println!("Field {}:\n{}\n", i, (*board).description().to_string(x).unwrap());
                if i == 0 {
                    start_field = x;
                }
                i += 1;
            });

        println!("solve");
        let solution = sol.solve(&[start_field]);

    }
}
