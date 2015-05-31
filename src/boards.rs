use board::*;

pub struct European {
    desc: Description,
}

impl European {
     pub fn new() -> European {
        European { desc: Description::new("englisch", "..ooo..\n\
                                            ..ooo..\n\
                                            ooooooo\n\
                                            ooooooo\n\
                                            ooooooo\n\
                                            ..ooo..\n\
                                            ..ooo..", &[MoveDirections::Horizontal, MoveDirections::Vertical]).unwrap()
            }
    }
}

impl<'a> Board<'a> for European {
    fn description(&'a self) -> &'a Description {
        &self.desc
    }
    fn normalize(&self, state: State) -> State {
        state
    }
    fn equivalent_fields(&self, state: State) -> Vec<State> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;
    
    #[test]
    fn test_european_board() {
        let board = European::new();
        assert!(board.description().verify_board(&board));
    }
}