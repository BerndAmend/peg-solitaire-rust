/// current state of a board
pub type State = u64;

#[derive(Debug)]
pub enum MoveDirections {
    Horizontal,
    Vertical,
    LeftDiagonal,
    RightDiagonal
}

#[derive(Debug, PartialEq)]
pub enum DescriptionError {
	NoName,
    NotEnoughPegs,
    TooManyPegs,
    LineLengthNotEqual,
    NoMoveDirections,
    NoMovesPossible,
    InvalidLayout
}

#[derive(Debug)]
pub struct Description {
	pub name: String,
	pub layout: String,
	pub directions: Vec<MoveDirections>,
	pub pegs: usize,
	
	/// Describes how (x,y)-positions (map-key) inside the boardDescription correspond
    /// to the bit position used to represent the board
	pub lut: Vec<Vec<i32>>,
	
	/// ...111... required to mask bits effected by a move and execute the move
	pub movemask: Vec<State>,

	/// ...110... required to check if a move is possible
	pub checkmask1: Vec<State>,
	
	/// ...011... required to check if a move is possible
	pub checkmask2: Vec<State>,
}

impl Description {
	pub fn new(name: &str, layout: &str, directions: Vec<MoveDirections>) -> Result<Description, DescriptionError> {
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
			directions: directions,
			pegs: layout.chars().filter(|&x| x == 'o').count(),
			lut: vec![],
			movemask: vec![],
			checkmask1: vec![],
			checkmask2: vec![]};
    
        if desc.pegs < 3 {
            return Err(DescriptionError::NotEnoughPegs);            
        }
        
        if desc.pegs > 64 {
            return Err(DescriptionError::TooManyPegs);
        }
        
        let mut lines = layout.lines_any();
        let len = lines.next().unwrap().len();
        if !lines.all(|x| x.len() == len) {
            return Err(DescriptionError::LineLengthNotEqual);
        }

        desc.lut = {
            let mut pos = desc.pegs as i32;
            layout.lines_any().map(|line|     
                line.chars().map(|x| if x == 'o' { pos -= 1; pos } else { -1 }).collect()
            ).collect()
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
                    
                    for dir in desc.directions.iter() {
                        let (valid, x1, y1, x2, y2) = match *dir {
                            MoveDirections::Horizontal    => (true, x+1, y  , x+2, y  ),
                            MoveDirections::Vertical      => (true, x  , y+1, x  , y+2),
                            MoveDirections::LeftDiagonal  => (true, x+1, y+1, x+2, y+2),
                            MoveDirections::RightDiagonal => if x > 2 { (true, x-1, y+1, x-2, y+2) } else { (false, 0, 0, 0, 0) },
                        };
                        
                        if valid &&
                           x1 < x_max && y1 < y_max && lut[y1][x1] != -1 &&
                           x2 < x_max && y2 < y_max && lut[y2][x2] != -1 {
                            desc.movemask.push(((1u64 << lut[y][x]) | (1u64 << lut[y1][x1]) | (1u64 << lut[y2][x2])));
                            desc.checkmask1.push(((1u64 << lut[y][x]) | (1u64 << lut[y1][x1])));
                            desc.checkmask2.push(((1u64 << lut[y1][x1]) | (1u64 << lut[y2][x2])));
                        }
                    }
                }
            }
        }
        
        if desc.movemask.is_empty() {
        	return Err(DescriptionError::NoMovesPossible);
        }

		Ok(desc)
	}
	
	/// creates a human-readable version of a field, the output as described by the layout
	/// returns None if state was invalid
	pub fn to_string(&self, state: State) -> Option<String> {
		if self.pegs < 64 && state > (1u64 << (self.pegs+1) - 1) {
				None
		} else {
    		let mut mask = 1u64;
    		let mut result = String::with_capacity(self.layout.len());
    		
    		for x in self.layout.chars() {
    			result.push(match x {
                    e @ '.' | e @ '\n' => e,
                    'o' => if state & mask != 0 { 'x' } else { '.' },
                    x => unreachable!(),
                });
    			mask <<= 1;
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
    	
    	if !self.layout.chars().zip(state.chars()).all(
    		|x| match x {
    			(left, right) => match left {
    				'o' => right == 'x' || right == '.',
    				'.' => right == ' ',
    				_ => false,
    			},
    		}) {
			return None;
		}
    	
    	for x in state.chars().rev() {
    		if pos > self.pegs {
    			return None;
    		}
    		match x {
    			'\n' | ' ' | '\t'  => {},
    			'x' => {result |= 1u64 << pos; pos+=1;},
    			'.' => pos+=1,
    			_ => return None,
    		};
    	}
    	
    	if pos > self.pegs {
           None
        } else {
        	Some(result)
    	}
    } 
}

pub trait Board<'a> {
	fn description() -> &'a Description;
    fn normalize(state: State) -> State;
    fn equivalent_fields(state: State) -> Vec<State>;
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn description_has_no_name() {
    	assert_eq!(Description::new("", "ooo", vec![MoveDirections::Horizontal]).err(), Some(DescriptionError::NoName));
    }
    
    #[test]
    fn description_has_not_enough_pegs() {
        assert_eq!(Description::new("test", "o", vec![MoveDirections::Horizontal]).err(), Some(DescriptionError::NotEnoughPegs));
    }
    
    #[test]
    fn description_has_no_move_directions() {
        assert_eq!(Description::new("test", "ooo", vec![]).err(), Some(DescriptionError::NoMoveDirections));
    }
    
    #[test]
    fn description_too_many_pegs() {
        assert_eq!(Description::new("test", &(0..65).map(|_| "o").collect::<String>(), vec![MoveDirections::Horizontal]).err(), Some(DescriptionError::TooManyPegs));
    }
    
    #[test]
    fn description_line_length_not_equal() {
        assert_eq!(Description::new("test", "oo\nooo", vec![MoveDirections::Horizontal]).err(), Some(DescriptionError::LineLengthNotEqual));
    }
    
    #[test]
    fn description_no_moves_possible() {
        assert_eq!(Description::new("test", "ooo", vec![MoveDirections::Vertical]).err(), Some(DescriptionError::NoMovesPossible));
    }
    
    #[test]
    fn description_invalid_layout_is_detected() {
        assert_eq!(Description::new("test", " .ooo", vec![MoveDirections::Horizontal]).err(), Some(DescriptionError::InvalidLayout));
    }
    
    #[test]
    fn description_layout_is_valid() {
        assert!(Description::new("test", ".ooo", vec![MoveDirections::Horizontal]).is_ok());
    }
    
    #[test]
    fn description_valid() {
        assert!(Description::new("test", "ooo", vec![MoveDirections::Horizontal]).is_ok());
    }
    
    #[test]
    fn description_peg_count_is_correct() {
        assert_eq!(Description::new("test", "ooooo", vec![MoveDirections::Horizontal]).unwrap().pegs, 5);
    }
    
    #[test]
    fn description_to_string_is_ok_1() {
        assert_eq!(Description::new("test", "ooooo", vec![MoveDirections::Horizontal]).unwrap().to_string(0b10101_u64).unwrap(), "x.x.x");
    }
    
    #[test]
    fn description_to_string_is_ok_2() {
        assert_eq!(Description::new("test", &(0..64).map(|_| "o").collect::<String>(), vec![MoveDirections::Horizontal]).unwrap().to_string(!0u64).unwrap(), (0..64).map(|_| "x").collect::<String>());
    }
    
    #[test]
    fn description_to_string_detects_invalid_state() {
        assert!(Description::new("test", "ooo", vec![MoveDirections::Horizontal]).unwrap().to_string(0b1111_u64).is_none());
    }
    
    #[test]
    fn description_from_string_is_ok() {
        assert_eq!(Description::new("test", ".ooooo.", vec![MoveDirections::Horizontal]).unwrap().from_string(" x.x.. ").unwrap(), 0b10100_u64);
    }
    
    #[test]
    fn description_from_string_detects_invalid_state_1() {
        assert!(Description::new("test", "ooo", vec![MoveDirections::Horizontal]).unwrap().from_string("xxxx").is_none());
    }
    
    #[test]
    fn description_from_string_detects_invalid_state_2() {
        assert!(Description::new("test", "ooo", vec![MoveDirections::Horizontal]).unwrap().from_string("xxxxb").is_none());
    }
    
    #[test]
    fn description_from_string_detects_invalid_state_3() {
        assert!(Description::new("test", ".ooo.", vec![MoveDirections::Horizontal]).unwrap().from_string("  xxx").is_none());
    }
    
    #[test]
    fn description_from_string_detects_invalid_state_4() {
        assert!(Description::new("test", ".ooo.", vec![MoveDirections::Horizontal]).unwrap().from_string(" xxx  ").is_none());
    }
}
