use board::*;

fn generate_shift_code(transformation: &Transformation) -> String {
    let mut result = String::new();

    result.push_str("(\n               ");
    let mut i = 0;
    for (&shift, pos) in transformation.iter() {
        result.push_str("((state & ");
        result.push_str(&pos.to_string());
        result.push_str("u64)");

        if shift > 0 {
            result.push_str(" << ");
            result.push_str(&shift.to_string());
        } else if shift < 0 {
            result.push_str(" >> ");
            result.push_str(&i32::abs(shift).to_string());
        }
        result.push_str(")");

        if i % 4 == 3 {
            result.push_str("\n            ");
        }

        if i != transformation.len() - 1 {
            result.push_str(" | ");
        }

        i += 1;
    }

    result.push_str(")");

    result
}

pub fn get_rust_code(desc: &Description) -> String {
    let mut r = String::new();

    r.push_str(&format!("pub struct {} {{\n", &desc.name));
    r.push_str(         "    desc: Description,\n");
    r.push_str(         "}\n\n");

    r.push_str(&format!("impl {} {{\n", &desc.name));
    r.push_str(&format!("    pub fn new() -> {} {{\n", &desc.name));
    r.push_str(         "        use board::MoveDirections::*;\n");
    r.push_str(&format!("        {} {{\n", &desc.name));
    r.push_str(&format!("            desc: Description::new(\"{}\", \"{}\", &{:?}).unwrap()\n", &desc.name, &desc.layout.replace("\n", "\\n"), &desc.directions));
    r.push_str(         "        }\n");
    r.push_str(         "    }\n");
    r.push_str(         "}\n\n");

    r.push_str(&format!("impl Board for {} {{\n", &desc.name));

    r.push_str(         "    fn description(&self) -> &Description {\n");
    r.push_str(         "        &self.desc\n");
    r.push_str(         "    }\n\n");

    r.push_str(         "    fn normalize(&self, state: State) -> State {\n");
    r.push_str(         "        let mut n = state;\n");

    let mut pos = 0;
    for trans in desc.transformations.iter() {
    r.push_str(&format!("        let p{} = {};\n", pos, generate_shift_code(&trans)));
    r.push_str(&format!("        if p{} < n {{ n = p{}; }}\n", pos, pos));
    pos += 1;
    }

    r.push_str(         "        n\n");
    r.push_str(         "    }\n\n");


    r.push_str(         "    fn equivalent_fields(&self, state: State) -> [State; 8] {\n");
    r.push_str(         "        let mut n = [EMPTY_STATE; 8];\n");
    r.push_str(         "        n[0] = state;\n");

    let mut pos = 1;
    for trans in desc.transformations.iter() {
    r.push_str(&format!("        n[{}] = {};\n", pos, generate_shift_code(&trans)));
    pos += 1;
    }

    r.push_str(         "        n\n");
    r.push_str(         "    }\n");

    r.push_str(         "}\n");

    r
}