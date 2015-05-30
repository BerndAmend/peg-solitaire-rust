mod board;
mod solver;
mod generator;
mod boardset;

fn main() {
	
	println!("peg-solitaire rust edition");
	
    generator::get_rust_code(&board::Description::new("englisch", "..ooo..\n\
                                            ..ooo..\n\
                                            ooooooo\n\
                                            ooooooo\n\
                                            ooooooo\n\
                                            ..ooo..\n\
                                            ..ooo..", vec![board::MoveDirections::Horizontal, board::MoveDirections::Vertical]).unwrap());

    generator::get_rust_code(&board::Description::new("european", "..ooo..\n\
                                            .ooooo.\n\
                                            ooooooo\n\
                                            ooooooo\n\
                                            ooooooo\n\
                                            .ooooo.\n\
                                            ..ooo..", vec![board::MoveDirections::Horizontal, board::MoveDirections::Vertical]).unwrap());
    
    generator::get_rust_code(&board::Description::new("15holes", "o....\n\
                                           oo...\n\
                                           ooo..\n\
                                           oooo.\n\
                                           ooooo", vec![board::MoveDirections::Horizontal, board::MoveDirections::Vertical, board::MoveDirections::LeftDiagonal, board::MoveDirections::RightDiagonal]).unwrap());
}

