#![allow(dead_code)]
use std::fs::File;
use std::io::Write;

include!("src/description.rs");

fn generate_shift_code(transformation: &Transformation) -> String {
    let mut result = String::new();
    let mut line = String::new();

    let mut i = 0;

    for (&shift, pos) in transformation {

        line.push_str("(state & ");
        line.push_str(&pos.to_string());
        line.push_str("u64)");

        if shift > 0 {
            line.push_str(" << ");
            line.push_str(&shift.to_string());
        } else if shift < 0 {
            line.push_str(" >> ");
            line.push_str(&i32::abs(shift).to_string());
        }

        if line.len() > 100 {
            result.push_str(&line);
            line.clear();

            if i != transformation.len() - 1 {
                line.push_str("\n            ");
            }
        }

        if i != transformation.len() - 1 {
            line.push_str(" | ");
        }

        i += 1;
    }

    if !line.is_empty() {
        result.push_str(&line);
    }

    result
}

fn format_mask(name: &str, vec: &[State]) -> String {
    let mut result = String::new();
    let mut line = String::new();

    let mut pos = 0;
    line.push_str(&format!("const {}: &'static [State] = &[", &name));
    for i in vec {
        line.push_str(&format!("{}u64", i));

        if pos != vec.len() - 1 {
            line.push_str(", ");
        }

        if line.len() > 100 {
            result.push_str(&line);
            line.clear();
            line.push_str("\n                            ");
        }
        pos += 1;
    }

    if !line.is_empty() {
        result.push_str(&line);
    }

    result.push_str("];\n");
    result
}

fn get_rust_code(desc: &Description) -> String {
    let mut r = String::new();

    r.push_str(&format!("pub struct {}Board;\n", desc.name));
    r.push_str(&format!("impl Board for {}Board {{\n", desc.name));

    r.push_str(&format!("const PEGS: usize = {};\n", desc.pegs));
    r.push_str(&format!("const SIZE: usize = {};\n", desc.movemask.len()));
    r.push_str(&format_mask("MOVEMASK", &desc.movemask));
    r.push_str(&format_mask("CHECKMASK1", &desc.checkmask1));
    r.push_str(&format_mask("CHECKMASK2", &desc.checkmask2));

    r.push_str("\n");
    r.push_str("fn description() -> Description {\n");
    r.push_str(&format!(" Description::new({:?}, {:?}, &{:?}).unwrap()",
                        &desc.name,
                        &desc.layout,
                        &desc.directions));
    r.push_str("\n}\n");

    r.push_str("\n");
    r.push_str("fn normalize(state: State) -> State {\n");

    let states = desc.transformations
        .iter()
        .map(|trans| generate_shift_code(&trans))
        .collect::<Vec<String>>();
    let joined_states = states.join(",");
    r.push_str(&format!("   *[state,{}].iter().min().unwrap()\n", &joined_states));

    r.push_str("}\n\n");

    r.push_str("\n");
    r.push_str("fn equivalent_fields(state: State) -> [State; 8] {\n");
    if states.len() == 7 {
        r.push_str(&format!("    [state,{}]\n", &joined_states));
    } else {
        let rest = (states.len()..7)
            .map(|_| "EMPTY_STATE".to_owned())
            .collect::<Vec<String>>()
            .join(",");
        r.push_str(&format!("    [state,{},\n   {}]\n", &joined_states, rest));
    }
    r.push_str("}\n");

    r.push_str("}\n");

    r
}

fn main() {
    let mut f = File::create("src/boards.rs").unwrap();

    let descriptions =
        [Description::new("English",
                          "..ooo..\n..ooo..\nooooooo\nooooooo\nooooooo\n..ooo..\n..ooo..",
                          &[MoveDirections::Horizontal, MoveDirections::Vertical])
             .unwrap(),
         Description::new("European",
                          "..ooo..\n.ooooo.\nooooooo\nooooooo\nooooooo\n.ooooo.\n..ooo..",
                          &[MoveDirections::Horizontal, MoveDirections::Vertical])
             .unwrap(),
         Description::new("Holes15",
                          "o....\noo...\nooo..\noooo.\nooooo",
                          &[MoveDirections::Horizontal,
                            MoveDirections::Vertical,
                            MoveDirections::LeftDiagonal,
                            MoveDirections::RightDiagonal])
             .unwrap()];

    f.write_all(b"use board::Board;\n").unwrap();
    f.write_all(b"use description::MoveDirections::*;\n").unwrap();
    f.write_all(b"use description::{Description, State, EMPTY_STATE};\n").unwrap();

    for x in &descriptions {
        f.write_all(&get_rust_code(&x).as_bytes()).unwrap();
    }
    // let desc = &descriptions[0];

    // let start_fields = boards::possible_start_fields();
    // start_fields.foreach(|x| {
    //     println!("Field {}:\n{}\n", x, desc.to_string(x).unwrap());
    //  });
}
