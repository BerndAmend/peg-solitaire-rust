mod board;
mod boardset;
mod boards;
mod solver;
mod generator;

fn main() {

    println!("peg-solitaire rust edition");

    if false {
        let descriptions = [board::Description::new("Englisch", "..ooo..\n\
                                                ..ooo..\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                ..ooo..\n\
                                                ..ooo..", &[board::MoveDirections::Horizontal, board::MoveDirections::Vertical]).unwrap(),
                            board::Description::new("European", "..ooo..\n\
                                                .ooooo.\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                ooooooo\n\
                                                .ooooo.\n\
                                                ..ooo..", &[board::MoveDirections::Horizontal, board::MoveDirections::Vertical]).unwrap(),
                            board::Description::new("Holes15", "o....\n\
                                               oo...\n\
                                               ooo..\n\
                                               oooo.\n\
                                               ooooo", &[board::MoveDirections::Horizontal, board::MoveDirections::Vertical, board::MoveDirections::LeftDiagonal, board::MoveDirections::RightDiagonal]).unwrap()
                            ];

        for x in descriptions.iter() {
            println!("{}", generator::get_rust_code(&x));
        }
    } else {
        let board = boards::Englisch::new();
        let sol = solver::Solver::new(board);
    }
}
