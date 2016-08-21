#![feature(associated_consts)]
mod board;
mod boardset;
mod boards;
mod generator;
mod generated;

fn main() {
    println!("peg-solitaire rust edition
Copyright (C) 2015-2016 Bernd Amend <berndamend+pegsolitaire@googlemail.com>
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as published by
the Free Software Foundation. This program comes with ABSOLUTELY NO WARRANTY");

    match std::env::args().nth(1) {
        None => {
            println!("usage {}", std::env::args().nth(0).unwrap());
            println!("  solve_eng");
            println!("  solve_eng_par");
            println!("  all");
        }
        Some(str) => {
            match str.as_ref() {
                "solve_eng" => {
                    boards::solve::<generated::EnglishBoard>(8589869055u64);
                }
                "solve_eng_par" => {
                    boards::solve_parallel::<generated::EnglishBoard>(8589869055u64);
                }
                "all" => {
                    let descriptions =
                        [board::Description::new("English",
                                                 "..ooo..\n..ooo..\nooooooo\nooooooo\nooooooo\n.\
                                                  .ooo..\n..ooo..",
                                                 &[board::MoveDirections::Horizontal,
                                                   board::MoveDirections::Vertical])
                             .unwrap(),
                         board::Description::new("European",
                                                 "..ooo..\n.ooooo.\nooooooo\nooooooo\nooooooo\n.\
                                                  ooooo.\n..ooo..",
                                                 &[board::MoveDirections::Horizontal,
                                                   board::MoveDirections::Vertical])
                             .unwrap(),
                         board::Description::new("Holes15",
                                                 "o....\noo...\nooo..\noooo.\nooooo",
                                                 &[board::MoveDirections::Horizontal,
                                                   board::MoveDirections::Vertical,
                                                   board::MoveDirections::LeftDiagonal,
                                                   board::MoveDirections::RightDiagonal])
                             .unwrap()];

                    for x in &descriptions {
                        println!("//Name: {}\n{}", x.name, generator::get_rust_code(&x));
                    }
                    // let desc = &descriptions[0];

                    // let start_fields = boards::possible_start_fields();
                    // start_fields.foreach(|x| {
                    //     println!("Field {}:\n{}\n", x, desc.to_string(x).unwrap());
                    //  });
                }
                _ => {}
            }
        }
    }
}
