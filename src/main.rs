#![feature(scoped)]

mod board;
mod boardset;
mod boards;
mod generator;

fn main() {
    println!("peg-solitaire rust edition");

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

//    for x in descriptions.iter() {
//        println!("{}", generator::get_rust_code(&x));
//    }

    let desc = &descriptions[0];

    //println!("{}", generator::get_rust_code(&desc));

//    let start_fields = boards::possible_start_fields();
//    start_fields.foreach(|x| {
//            println!("Field {}:\n{}\n", x, desc.to_string(x).unwrap());
//        });

    boards::solve(8589869055u64);
}
