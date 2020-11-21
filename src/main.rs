mod description;
mod llvm;
mod naive;
mod opencl;
mod utils;

use crate::description::{Description, MoveDirections, Solver};
use std::env::args;

fn main() {
    println!(
        "peg-solitaire rust edition
Copyright (C) 2015-2020 Bernd Amend <berndamend+pegsolitaire@googlemail.com>
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as published by
the Free Software Foundation. This program comes with ABSOLUTELY NO WARRANTY"
    );

    if args().len() <= 2 {
        eprintln!(
            "usage {} <board> <backend>\n    board: english, euro, 15, custom\n    backend: naive, llvm, (opencl)",
            std::env::args().nth(0).unwrap()
        );
        return;
    }

    let (desc, start) = match args().nth(1).unwrap_or(String::new()).as_ref() {
        "english" => (
            Description::new(
                "English",
                "..ooo..\n..ooo..\nooooooo\nooooooo\nooooooo\n..ooo..\n..ooo..",
                &[MoveDirections::Horizontal, MoveDirections::Vertical],
            )
            .unwrap(),
            "  xxx  \n  xxx  \nxxxxxxx\nxxx.xxx\nxxxxxxx\n  xxx  \n  xxx  ",
        ),
        "euro" => (
            Description::new(
                "European",
                "..ooo..\n.ooooo.\nooooooo\nooooooo\nooooooo\n.ooooo.\n..ooo..",
                &[MoveDirections::Horizontal, MoveDirections::Vertical],
            )
            .unwrap(),
            "  xxx  \n xxxxx \nxxxxxxx\nxxx.xxx\nxxxxxxx\n xxxxx \n  xxx  ",
        ),
        "15" => (
            Description::new(
                "Holes15",
                "o....\noo...\nooo..\noooo.\nooooo",
                &[
                    MoveDirections::Horizontal,
                    MoveDirections::Vertical,
                    MoveDirections::LeftDiagonal,
                    MoveDirections::RightDiagonal,
                ],
            )
            .unwrap(),
            "x    \nxx   \nxxx  \nxxxx \n.xxxx",
        ),
        "custom" => unimplemented!(),
        name => {
            panic!(
                "'{}' is an unknown board, supported boards are english, euro, 15, custom",
                name
            );
        }
    };

    let solver: Box<dyn Solver> = match args().nth(2).unwrap_or(String::new()).as_ref() {
        "naive" => Box::new(naive::NaiveSolver::new(&desc)),
        "llvm" => Box::new(llvm::LLVMSolver::new(&desc)),
        "opencl" => Box::new(opencl::OpenCLSolver::new(&desc)),
        name => {
            panic!(
                "'{}' is an unknown backend, supported backends are naive, llvm, (opencl)",
                name
            );
        }
    };

    let solution = solver.solve(desc.from_string(start).unwrap());

    // for (i, x) in solution.iter().enumerate() {
    //     println!("{} len {}", i, x.len());
    // }

    // let start_fields = desc.possible_start_fields();
    // for &x in start_fields.iter() {
    //     println!("Field {}:\n{}\n", x, desc.to_string(x).unwrap());
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn solve_test() {
//         let desc = Description::new(
//             "European",
//             "..ooo..\n.ooooo.\nooooooo\nooooooo\nooooooo\n.ooooo.\n..ooo..",
//             &[MoveDirections::Horizontal, MoveDirections::Vertical],
//         )
//         .unwrap();
//         let start = desc
//             .from_string("  xxx  \n xxxxx \nxxxxxxx\nxxx.xxx\nxxxxxxx\n xxxxx \n  xxx  ")
//             .unwrap();
//         let solver = Solver::new(desc.clone());
//         assert!(solver.solve(start).iter().fold(0, |o, i| o + i.len()) == 23475688);
//     }
// }
