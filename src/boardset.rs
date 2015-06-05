use std;
use std::slice::Chunks;
use board::State;
use board::EMPTY_STATE;

/// In the interest of memory-savings, we start with the smallest feasible
/// power-of-two table size that can hold three items without rehashing. If we
/// started with a size of 2, we'd have to expand as soon as the second item
/// was added.
const INITIAL_SIZE: usize = 1 << 5;

#[derive(Clone)]
pub struct BoardSet {
    len: usize,

    data: Vec<State>,
}

impl BoardSet {
    pub fn new() -> BoardSet {
        BoardSet::with_capacity(0)
    }

    pub fn with_capacity(expected_size: usize) -> BoardSet {
        let size = BoardSet::compute_capacity_for_size(expected_size, INITIAL_SIZE);
        let mut data = Vec::with_capacity(size);
        data.extend(std::iter::repeat(EMPTY_STATE).take(size));
        BoardSet {
            len: 0,
            data: data
        }
    }

    /// Reserves capacity for at least additional more elements to be inserted in the HashSet.
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
        use std::mem::replace;

        let expected_size = self.len() + additional;
        if BoardSet::size_fits_into_capacity(expected_size, self.data.len()) {
            return;
        }

        let mut new_obj = BoardSet::with_capacity(expected_size);
        if !self.is_empty() {
            new_obj.fast_insert_all(&self.data);
        }
        replace(&mut self.data, new_obj.data);
    }

    /// Returns the size of the internal array
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// current fill state in percent
    pub fn used(&self) -> f32 {
        if self.len == 0 {
            1.0
        } else {
            (self.len as f32) / (self.data.len() as f32)
        }
    }

    /// Clears the set, removing all values and freeing the memory.
    pub fn clear(&mut self) {
        *self = BoardSet::new();
    }

    /// Clears the set, removing all values and reallocating the internal
    /// memory to fit new_expected_size elements.
    pub fn clear_with_size(&mut self, expected_size: usize) {
        *self = BoardSet::with_capacity(expected_size);
    }

    /// Returns true if the set contains a value.
    pub fn contains(&self, value: State) -> bool {
        self.data[self.find_or_empty(value)] != EMPTY_STATE
    }

    /// Adds a value to the set.
    pub fn insert(&mut self, value: State) {
        self.reserve(1);
        self.fast_insert(value);
    }

    /// Adds a value to the set without checking if it is large enough.
    pub fn fast_insert(&mut self, o: State) {
        let index = self.find_or_empty(o);
        if self.data[index] == EMPTY_STATE {
            self.len += 1;
            self.data[index] = o;
        }
    }

    pub fn merge(&mut self, other: &BoardSet) {
        self.reserve(other.len());
        self.fast_insert_all(&other.data)
    }

    pub fn insert_all(&mut self, other: &[State]) {
        self.reserve(other.len());
        self.fast_insert_all(other);
    }

    pub fn insert_all_abort_on_empty_state(&mut self, other: &[State]) {
        self.reserve(other.len());
        for x in other {
            if *x == EMPTY_STATE {
                break;
            }
            self.fast_insert(*x);
        }
    }

    // add the elements without checking if there is enough space
    pub fn fast_insert_all(&mut self, other: &[State]) {
        for x in other {
            if *x != EMPTY_STATE {
                self.fast_insert(*x);
            }
        }
    }

    pub fn foreach<F>(&self, mut func: F) where F: FnMut(State) {
        for x in self.data.iter().filter(|&x| *x != EMPTY_STATE) {
           func(*x);
       }
    }
    
    pub fn chunks(&self, size: usize) -> Chunks<State> {
        self.data.chunks(size)
    }

    fn size_fits_into_capacity(expected: usize, current: usize) -> bool {
        4 * expected < 3 * current
    }

    fn compute_capacity_for_size(expected: usize, current: usize) -> usize {
        let mut new_capacity = current;
        while !Self::size_fits_into_capacity(expected, new_capacity) {
            new_capacity <<= 1;
        }
        new_capacity
    }

    fn get_index_from_state(&self, value: State) -> usize {
        let mut h = value;
        // modified MurmurHash3
//        h ^= h >> 16;
        h *= 0x85ebca6b;
        h ^= h >> 13;
//        h *= 0xc2b2ae35;
//        h ^= h >> 16;
        (h & (self.data.len()-1) as u64) as usize 
    }

    /// Returns the index in the table at which a particular item resides, or the
    /// index of an empty slot in the table where this item should be inserted if
    /// it is not already in the table.
    /// @return index
    fn find_or_empty(&self, o: State) -> usize {
        let mut index = self.get_index_from_state(o);

        loop {
            let existing = self.data[index];
            if existing == EMPTY_STATE || o == existing {
                return index;
            } else {
                index = (index + 1) & (self.data.len()-1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::board::EMPTY_STATE;

    #[test]
    fn boardset_test() {
        let mut x = BoardSet::new();
        for i in 0u64..512u64 {
            x.insert(i);
        }

        assert_eq!(x.len(), 512);
        assert_eq!(x.data.len(), 1024);
        assert_eq!(x.used(), 0.5);

        x.foreach(|i| assert!(i > 0 && i < 512));

        for i in 1u64..512u64 {
            assert!(x.contains(i));
        }

        assert!(!x.contains(EMPTY_STATE));

        x.clear();
        assert!(x.is_empty());
    }

    #[test]
    fn boardset_insert_all() {
        let mut x = BoardSet::new();
        x.insert_all(&[1,2,3,4,5]);
        x.foreach(|i| assert!(i > 0 && i < 6));
    }

    #[test]
    fn boardset_merge_1() {
        let mut x = BoardSet::new();
        let mut y = BoardSet::new();
        x.insert_all(&[1,2,3,4,5]);
        y.insert_all(&[1,2,3,4,5]);
        x.merge(&y);
        x.foreach(|i| assert!(i > 0 && i <= 5));
        assert_eq!(x.len(), 5);
    }

    #[test]
    fn boardset_merge_2() {
        let mut x = BoardSet::new();
        let mut y = BoardSet::new();
        x.insert_all(&[1,2,3,4,5]);
        y.insert_all(&vec![6,7,8,9,10]);
        x.merge(&y);
        x.foreach(|i| assert!(i > 0 && i <= 10));
        assert_eq!(x.len(), 10);
    }
}
