use board::*;
//use std::fs::File;
//use std::io::prelude::*;

//let mut f = try!(File::create("foo.txt"));
//try!(f.write_all(b"Hello, world!"));

fn generate_shift_code(transformation: &Transformation) -> String {
    let mut result = String::new();
    let mut line = String::new();

    let mut i = 0;

    for (&shift, pos) in transformation.iter() {

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

fn format_mask(name: &str, vec: &Vec<State>) -> String {
    let mut result = String::new();
    let mut line = String::new();

    let mut pos = 0;
    line.push_str(&format!("static {}: [State; SIZE] = [", &name));
    for i in vec.iter() {
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

pub fn get_rust_code(desc: &Description) -> String {
    let mut r = String::new();

    r.push_str(&format!("const PEGS: usize = {};\n\n", desc.pegs));
    r.push_str(&format!("const SIZE: usize = {};\n", desc.movemask.len()));
    r.push_str(&format_mask("MOVEMASK", &desc.movemask));
    r.push_str(&format_mask("CHECKMASK1", &desc.checkmask1));
    r.push_str(&format_mask("CHECKMASK2", &desc.checkmask2));

    r.push_str(         "\n");
    r.push_str(         "#[warn(dead_code)]");
    r.push_str(         "fn normalize(state: State) -> State {\n");
    r.push_str(         "    let mut n = state;\n");

    let mut pos = 0;
    for trans in desc.transformations.iter() {
        r.push_str(&format!("    let p{} = {};\n", pos, generate_shift_code(&trans)));
        r.push_str(&format!("    if p{} < n {{ n = p{}; }}\n", pos, pos));
        pos += 1;
    }

    r.push_str(         "    n\n");
    r.push_str(         "}\n\n");

    r.push_str(         "\n");
    r.push_str(         "#[warn(dead_code)]");
    r.push_str(         "fn equivalent_fields(state: State) -> [State; 8] {\n");
    r.push_str(         "    let mut n = [EMPTY_STATE; 8];\n");
    r.push_str(         "    n[0] = state;\n");

    let mut pos = 1;
    for trans in desc.transformations.iter() {
        r.push_str(&format!("    n[{}] = {};\n", pos, generate_shift_code(&trans)));
        pos += 1;
    }

    r.push_str(         "    n\n");
    r.push_str(         "}\n");

    r
}