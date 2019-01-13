use std::collections::HashMap;

pub type State = u64;
pub const EMPTY_STATE: u64 = 0u64;

pub type Lut = Vec<Vec<i32>>;
pub type Transformation = HashMap<i32, State>;

#[derive(Debug, Clone)]
pub enum MoveDirections {
    Horizontal,
    Vertical,
    LeftDiagonal,
    RightDiagonal,
}

#[derive(Debug, PartialEq)]
pub enum DescriptionError {
    NoName,
    NotEnoughPegs,
    TooManyPegs,
    LineLengthNotEqual,
    NoMoveDirections,
    NoMovesPossible,
    InvalidLayout,
}

#[derive(Debug)]
pub struct Description {
    pub name: String,
    pub layout: String,
    pub directions: Vec<MoveDirections>,
    pub pegs: usize,

    /// Describes how (x,y)-positions (map-key) inside the boardDescription correspond
    /// to the bit position used to represent the board
    pub lut: Lut,

    /// ...111... required to mask bits effected by a move and execute the move
    pub movemask: Vec<State>,

    /// ...110... required to check if a move is possible
    pub checkmask1: Vec<State>,

    /// ...011... required to check if a move is possible
    pub checkmask2: Vec<State>,

    pub transformations: Vec<Transformation>,
}

impl Description {
    pub fn new(name: &str,
               layout: &str,
               directions: &[MoveDirections])
               -> Result<Description, DescriptionError> {
        if name.is_empty() {
            return Err(DescriptionError::NoName);
        }

        if directions.is_empty() {
            return Err(DescriptionError::NoMoveDirections);
        }

        if !layout.chars().all(|x| x == '.' || x == 'o' || x == '\n') {
            return Err(DescriptionError::InvalidLayout);
        }

        let mut desc = Description {
            name: name.to_string(),
            layout: layout.to_string(),
            directions: directions.to_vec(),
            pegs: layout.chars().filter(|&x| x == 'o').count(),
            lut: vec![],
            movemask: vec![],
            checkmask1: vec![],
            checkmask2: vec![],
            transformations: vec![],
        };

        if desc.pegs < 3 {
            return Err(DescriptionError::NotEnoughPegs);
        }

        if desc.pegs > 64 {
            return Err(DescriptionError::TooManyPegs);
        }

        let mut lines = layout.lines();
        let len = lines.next().unwrap().len();
        if !lines.all(|x| x.len() == len) {
            return Err(DescriptionError::LineLengthNotEqual);
        }

        desc.lut = {
            let mut pos = desc.pegs as i32;
            layout.lines()
                .map(|line| {
                    line.chars()
                        .map(|x| {
                            if x == 'o' {
                                pos -= 1;
                                pos
                            } else {
                                -1
                            }
                        })
                        .collect()
                })
                .collect()
        };

        // calculate the 3 required bit masks, to detect if a move is possible and to execute it
        {
            let lut = &desc.lut;
            let y_max = lut.len();
            let x_max = lut[0].len();
            for y in 0..y_max {
                for x in 0..x_max {
                    if lut[y][x] == -1 {
                        continue;
                    }

                    for dir in &desc.directions {
                        let (valid, x1, y1, x2, y2) = match *dir {
                            MoveDirections::Horizontal => (true, x + 1, y, x + 2, y),
                            MoveDirections::Vertical => (true, x, y + 1, x, y + 2),
                            MoveDirections::LeftDiagonal => (true, x + 1, y + 1, x + 2, y + 2),
                            MoveDirections::RightDiagonal => {
                                if x > 2 {
                                    (true, x - 1, y + 1, x - 2, y + 2)
                                } else {
                                    (false, 0, 0, 0, 0)
                                }
                            }
                        };

                        if valid && x1 < x_max && y1 < y_max && lut[y1][x1] != -1 &&
                           x2 < x_max && y2 < y_max &&
                           lut[y2][x2] != -1 {
                            desc.movemask.push((1u64 << lut[y][x]) | (1u64 << lut[y1][x1]) |
                                                (1u64 << lut[y2][x2]));
                            desc.checkmask1.push((1u64 << lut[y][x]) | (1u64 << lut[y1][x1]));
                            desc.checkmask2.push((1u64 << lut[y1][x1]) | (1u64 << lut[y2][x2]));
                        }
                    }
                }
            }
        }

        if desc.movemask.is_empty() {
            return Err(DescriptionError::NoMovesPossible);
        }

        // calculate transformations
        {
            fn vertical_flip(lut: &Lut) -> Lut {
                let mut r = lut.clone();
                r.reverse();
                r
            }
            fn transpose(lut: &Lut) -> Lut {
                let mut r = lut.clone();
                for y in 0..r.len() {
                    for x in 0..r[0].len() {
                        if x > y {
                            continue;
                        }
                        let tmp = r[y][x];
                        r[y][x] = r[x][y];
                        r[x][y] = tmp;
                    }
                }
                r
            }

            fn horizontal_flip(lut: &Lut) -> Lut {
                lut.iter()
                    .map(|x| {
                        let mut r = x.clone();
                        r.reverse();
                        r
                    })
                    .collect()
            }


            fn have_same_shape(in1: &Lut, in2: &Lut) -> bool {
                if in1.len() != in2.len() || in1[0].len() != in2[0].len() {
                    false
                } else {
                    for y in 0..in1.len() {
                        for x in 0..in2[0].len() {
                            if (in1[y][x] == -1 || in2[y][x] == -1) && in1[y][x] != in2[y][x] {
                                return false;
                            }
                        }
                    }
                    true
                }
            }

            let mut transformations: Vec<Lut> = vec![];
            {
                let mut movemask_as_vec: Vec<Lut> = vec![];
                for x in &desc.movemask {
                    if let Some(v) = desc.to_vec(*x) {
                        movemask_as_vec.push(v);
                    }
                }

                let mut add_transformation = |func: &dyn Fn(&Lut) -> Lut| {
                    let x = func(&desc.lut);
                    if have_same_shape(&desc.lut, &x) &&
                       movemask_as_vec.iter().all(|i| {
                        if let Some(trans) = desc.from_vec(func(i)) {
                            desc.movemask.contains(&trans)
                        } else {
                            false
                        }
                    }) {
                        transformations.push(x);
                    }
                };

                add_transformation(&vertical_flip);
                add_transformation(&horizontal_flip);
                add_transformation(&|lut: &Lut| horizontal_flip(&vertical_flip(&lut)));

                // if transpose is possible
                if desc.lut.len() == desc.lut[0].len() {
                    add_transformation(&transpose);
                    add_transformation(&|lut: &Lut| vertical_flip(&transpose(&lut)));
                    add_transformation(&|lut: &Lut| horizontal_flip(&transpose(&lut)));
                    add_transformation(&|lut: &Lut| {
                        horizontal_flip(&vertical_flip(&transpose(&lut)))
                    });
                }
            }

            for trans in transformations {
                let mut field = vec![];

                for y in trans {
                    for x in y {
                        if x != -1 {
                            field.push(x);
                        }
                    }
                }

                let mut output = Transformation::new();

                for i in (0..field.len()).rev() {
                    let e = field[field.len() - 1 - i];
                    let diff = e - (i as i32);

                    let mut mask = 1u64 << i;
                    if let Some(cur) = output.get(&diff) {
                        mask |= *cur;
                    }

                    output.insert(diff, mask);
                }
                desc.transformations.push(output);
            }
        }

        Ok(desc)
    }

    /// creates a human-readable version of a field, the output as described by the layout
    /// returns None if state was invalid
    pub fn to_string(&self, state: State) -> Option<String> {
        if self.pegs < 64 && state > (1u64 << (self.pegs + 1) - 1) {
            None
        } else {
            let mut pos = self.pegs;
            let mut result = String::with_capacity(self.layout.len());

            for x in self.layout.chars() {
                result.push(match x {
                    '.' => ' ',
                    '\n' => '\n',
                    'o' => {
                        pos -= 1;
                        if state & (1u64 << pos) != 0 { 'x' } else { '.' }
                    }
                    _ => unreachable!(),
                });
            }

            Some(result)
        }
    }

    /// converts a human-readable version into the internal representation
    /// returns None if state was invalid
    pub fn from_string(&self, state: &str) -> Option<State> {
        let mut pos = 0;
        let mut result: State = 0u64;

        if state.len() != self.layout.len() {
            return None;
        }

        if !self.layout.chars().zip(state.chars()).all(|x| {
            match x {
                (left, right) => {
                    match left {
                        'o' => right == 'x' || right == '.',
                        '.' => right == ' ',
                        '\n' => right == '\n',
                        _ => false,
                    }
                }
            }
        }) {
            return None;
        }

        for x in state.chars().rev() {
            if pos > self.pegs {
                return None;
            }
            match x {
                '\n' | ' ' | '\t' => {}
                'x' => {
                    result |= 1u64 << pos;
                    pos += 1;
                }
                '.' => pos += 1,
                _ => return None,
            };
        }

        if pos > self.pegs { None } else { Some(result) }
    }

    /// blocked fields get -1, empty fields get 0, used fields 1
    pub fn to_vec(&self, state: State) -> Option<Lut> {
        if self.pegs < 64 && state > (1u64 << (self.pegs + 1) - 1) {
            None
        } else {
            Some(self.lut
                .iter()
                .map(|o| {
                    o.iter()
                        .map(|&x| {
                            if x == -1 {
                                -1i32
                            } else {
                                if (state & (1u64 << x)) == 0 {
                                    0i32
                                } else {
                                    1i32
                                }
                            }
                        })
                        .collect()
                })
                .collect())
        }
    }

    pub fn from_vec(&self, state: Lut) -> Option<State> {
        let mut r = EMPTY_STATE;
        for y in 0..state.len() {
            for x in 0..state[0].len() {
                match state[y][x] {
                    1 => r |= 1u64 << self.lut[y][x],
                    0 => {
                        if self.lut[y][x] == -1 {
                            return None;
                        }
                    }
                    -1 => {
                        if self.lut[y][x] != -1 {
                            return None;
                        }
                    }
                    _ => return None,
                }
            }
        }
        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_has_no_name() {
        assert_eq!(Description::new("", "ooo", &[MoveDirections::Horizontal]).err(),
                   Some(DescriptionError::NoName));
    }

    #[test]
    fn description_has_not_enough_pegs() {
        assert_eq!(Description::new("test", "o", &[MoveDirections::Horizontal]).err(),
                   Some(DescriptionError::NotEnoughPegs));
    }

    #[test]
    fn description_has_no_move_directions() {
        assert_eq!(Description::new("test", "ooo", &[]).err(),
                   Some(DescriptionError::NoMoveDirections));
    }

    #[test]
    fn description_too_many_pegs() {
        assert_eq!(Description::new("test",
                                    &(0..65).map(|_| "o").collect::<String>(),
                                    &[MoveDirections::Horizontal])
                       .err(),
                   Some(DescriptionError::TooManyPegs));
    }

    #[test]
    fn description_line_length_not_equal() {
        assert_eq!(Description::new("test", "oo\nooo", &[MoveDirections::Horizontal]).err(),
                   Some(DescriptionError::LineLengthNotEqual));
    }

    #[test]
    fn description_no_moves_possible() {
        assert_eq!(Description::new("test", "ooo", &[MoveDirections::Vertical]).err(),
                   Some(DescriptionError::NoMovesPossible));
    }

    #[test]
    fn description_invalid_layout_is_detected() {
        assert_eq!(Description::new("test", " .ooo", &[MoveDirections::Horizontal]).err(),
                   Some(DescriptionError::InvalidLayout));
    }

    #[test]
    fn description_layout_is_valid() {
        assert!(Description::new("test", ".ooo", &[MoveDirections::Horizontal]).is_ok());
    }

    #[test]
    fn description_valid() {
        assert!(Description::new("test", "ooo", &[MoveDirections::Horizontal]).is_ok());
    }

    #[test]
    fn description_peg_count_is_correct() {
        assert_eq!(Description::new("test", "ooooo", &[MoveDirections::Horizontal]).unwrap().pegs,
                   5);
    }

    #[test]
    fn description_to_string_is_ok_1() {
        assert_eq!(Description::new("test", "ooooo", &[MoveDirections::Horizontal])
                       .unwrap()
                       .to_string(0b10100_u64)
                       .unwrap(),
                   "x.x..");
    }

    #[test]
    fn description_to_string_is_ok_2() {
        assert_eq!(Description::new("test",
                                    &(0..64).map(|_| "o").collect::<String>(),
                                    &[MoveDirections::Horizontal])
                       .unwrap()
                       .to_string(!0u64)
                       .unwrap(),
                   (0..64).map(|_| "x").collect::<String>());
    }

    #[test]
    fn description_to_string_is_ok_3() {
        assert_eq!(Description::new("test", ".ooooo.", &[MoveDirections::Horizontal])
                       .unwrap()
                       .to_string(0b10100_u64)
                       .unwrap(),
                   " x.x.. ");
    }

    #[test]
    fn description_to_string_is_ok_4() {
        assert_eq!(Description::new("test",
                                    ".ooooo.\n..ooo..\n...o...",
                                    &[MoveDirections::Horizontal, MoveDirections::Vertical])
                       .unwrap()
                       .to_string(0b101000011_u64)
                       .unwrap(),
                   " x.x.. \n  ..x  \n   x   ");
    }

    #[test]
    fn description_to_string_detects_invalid_state() {
        assert!(Description::new("test", "ooo", &[MoveDirections::Horizontal])
            .unwrap()
            .to_string(0b1111_u64)
            .is_none());
    }

    #[test]
    fn description_from_string_is_ok() {
        assert_eq!(Description::new("test", ".ooooo.", &[MoveDirections::Horizontal])
                       .unwrap()
                       .from_string(" x.x.. ")
                       .unwrap(),
                   0b10100_u64);
    }

    #[test]
    fn description_from_string_detects_invalid_state_1() {
        assert!(Description::new("test", "ooo", &[MoveDirections::Horizontal])
            .unwrap()
            .from_string("xxxx")
            .is_none());
    }

    #[test]
    fn description_from_string_detects_invalid_state_2() {
        assert!(Description::new("test", "ooo", &[MoveDirections::Horizontal])
            .unwrap()
            .from_string("xxxxb")
            .is_none());
    }

    #[test]
    fn description_from_string_detects_invalid_state_3() {
        assert!(Description::new("test", ".ooo.", &[MoveDirections::Horizontal])
            .unwrap()
            .from_string("  xxx")
            .is_none());
    }

    #[test]
    fn description_from_string_detects_invalid_state_4() {
        assert!(Description::new("test", ".ooo.", &[MoveDirections::Horizontal])
            .unwrap()
            .from_string(" xxx  ")
            .is_none());
    }

    #[test]
    fn description_to_string_from_string() {
        let desc = Description::new("test", "..ooooo.", &[MoveDirections::Horizontal]).unwrap();
        let v = 0b11010u64;
        assert_eq!(desc.from_string(&desc.to_string(v).unwrap()).unwrap(), v);
    }

    #[test]
    fn description_from_string_to_string() {
        let desc = Description::new("test", "..ooooo.", &[MoveDirections::Horizontal]).unwrap();
        let v = "  ..x.x ";
        let from = desc.from_string(v).unwrap();
        assert_eq!(from, 0b00101u64);
        assert_eq!(desc.to_string(from).unwrap(), v);
    }

    #[test]
    fn description_to_vec_is_some() {
        assert!(Description::new("test", ".ooo.", &[MoveDirections::Horizontal])
            .unwrap()
            .to_vec(0b100u64)
            .is_some());
    }

    #[test]
    fn description_to_vec_is_none() {
        assert!(Description::new("test", ".ooo.", &[MoveDirections::Horizontal])
            .unwrap()
            .to_vec(0b1101u64)
            .is_none());
    }

    #[test]
    fn description_from_vec_is_some() {
        assert!(Description::new("test", ".ooo.", &[MoveDirections::Horizontal])
            .unwrap()
            .from_vec(vec![vec![-1, 1, 0, 0, -1]])
            .is_some());
    }

    #[test]
    fn description_from_vec_is_none() {
        assert!(Description::new("test", ".ooo.", &[MoveDirections::Horizontal])
            .unwrap()
            .from_vec(vec![vec![-1, 1, 0, -1, -1]])
            .is_none());
    }

    #[test]
    fn description_to_vec_from_vec_works() {
        let desc = Description::new("test", ".ooo.", &[MoveDirections::Horizontal]).unwrap();
        let value = 0b100u64;
        assert_eq!(desc.from_vec(desc.to_vec(value).unwrap()).unwrap(), value);
    }
}
