use board::State;
use board::EMPTY_STATE;

pub struct BoardSet {
    size: usize,
    data: Vec<State>,
}

impl BoardSet {
    pub fn new() -> BoardSet {
        BoardSet { size: 0, data: vec![] }
    }
    
    /// Returns the number of elements the set can hold without reallocating.
    pub fn capacity(&self) -> usize {
    	unimplemented!();
    }
    
    /// Reserves capacity for at least additional more elements to be inserted in the BoardSet.
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve(&mut self, additional: usize) {
    	unimplemented!();
    }
    
    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
    	unimplemented!();
    }
    
    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
    	unimplemented!();
    }
    
    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
    	unimplemented!();
    }
    
    /// Returns true if the set contains a value.
    pub fn contains(&self, value: State) -> bool {
    	unimplemented!();
    }
    
    /// Adds a value to the set.
    pub fn insert(&mut self, value: State) {
    	unimplemented!();
    }
    
    pub fn foreach<F>(&self, func: F) where F: Fn(State) {
    	for x in self.data.iter().filter(|&x| *x != EMPTY_STATE) {
    		func(*x);
		}
    }
    
    fn size_fits_into_capacity(expected: usize, current: usize) -> bool {
    	4 * expected < 3 * current
	}
    
    fn compute_capacity_for_size(expected: usize, current: usize) -> usize {
        let mut newCapacity = current;
        while !Self::size_fits_into_capacity(expected, newCapacity) {
            newCapacity <<= 1;
        }
        newCapacity
    }
}

/*

trait HashSet {
    protected var _size = 0

    def size = _size
    def table_size: Int
    protected var table_length_minus_1 = 0
    
    /// current fill state in percent
    def used = if (table_size == 0) 1.0 else (size.toDouble / table_size.toDouble)
    def isEmpty = size == 0

    /// Removes all elements from the HashSet and frees the required memory.
    def clear()

    /// Removes all elements from the HashSet and allocates the internal
    /// memory to fit new_expected_size elements.
    def clear(new_expected_size: Int)

    final def getIndex(value: Int): Int = {
        var h = value
        // Copied from Apache's AbstractHashedMap; prevents power-of-two collisions.
        h += ~(h << 9)
        h ^= (h >>> 14)
        h += (h << 4)
        h ^= (h >>> 10)
        // Power of two trick.
        h & table_length_minus_1
    }
}

object LongHashSet {
     /// In the interest of memory-savings, we start with the smallest feasible
     /// power-of-two table size that can hold three items without rehashing. If we
     /// started with a size of 2, we'd have to expand as soon as the second item
     /// was added.
    val INITIAL_TABLE_SIZE = 1 << 5

    val INVALID_ELEMENT = 0

    var use_standard_hash_set = false

    def newInstance: LongHashSet =
        if(use_standard_hash_set)
            new StandardLongHashSet
        else
            new MemoryEfficientLongHashSet

    def newInstance(expectedSize: Int): LongHashSet =
        if(use_standard_hash_set)
            new StandardLongHashSet(expectedSize)
        else
            new MemoryEfficientLongHashSet(expectedSize)

    def newInstance(c: LongHashSet): LongHashSet =
        if(use_standard_hash_set)
            new StandardLongHashSet(c)
        else
            new MemoryEfficientLongHashSet(c)
}


trait LongHashSet extends HashSet {

    /// Add all elements from the LongHashSet c to the current instance.
    def +=(c: LongHashSet)

    def +=(c: Array[Long])

    /// Add the element e to the HashSet.
    /// If e == LongHashSet.INVALID_ELEMENT an exception is thrown.
    def +=(o: Long)

    /// Checks if a HashSet contains o
    def contains(o: Long): Boolean

    def iter: HashSetIterator
    def iter(groupID: Int, groupSize: Int): HashSetIterator

    def foreach(func: Long => Unit) {
        val i = iter

        while (i.hasNext)
            func(i.unsafe_next)
    }

    def toList = {
        var r = List[Long]()
        foreach(r ::= _)
        r
    }

    def bitDistribution: Array[Long] = {
        val it = iter
        val result = Array.fill[Long](64)(0L)

        while (it.hasNext) {
            val elem = it.unsafe_next

            var i = 0
            while (i < 64) {
                if ((elem & (1L << i)) != 0)
                    result(i) += 1
                i += 1
            }
        }

        result
    }

    def bitDistributionString: String = {
        val bd = bitDistribution
        val max = bd.max.asInstanceOf[Double]

        val r = new StringBuilder
        r append "bd:"

        for (i <- 0 until bd.length) {
            if (bd(i) != 0) {
                r append " "
                r append i
                r append ":"
                r append "%3f".format(bd(i).asInstanceOf[Double] / max)
            }
        }

        r.result
    }
}

/// A memory-efficient hash set optimized for Longs
/// based on the java HashSet implementation by Google Inc.
class StandardLongHashSet(t: Array[Long], s: Int) extends LongHashSet {

    def allocateTableMemory(size: Int) = new Array[Long](size)

    def table_size = if(table == null) 0 else table.size

    private var table = t
    table_length_minus_1 = if (table == null) 0 else (table.length - 1)

    /// positions can be used to create a HashSetIterator that only work on a subset of the HashSet
    /// e.g. to read multiple elements from a HashSet at a time without synchronization
    class Iterator(val groupID: Int = 0, val groupSize: Int = 1) extends HashSetIterator {
        require(groupID >= 0)
        require(groupSize > 0)
        require(groupID < groupSize)

        private var _index = groupID
        private val table_length = table.length

        advanceToItem()

        override def hasNext = _index < table_length

        /// call this function ONLY if you really know what you are doing
        override def unsafe_next: Long = {
            val toReturn = table(_index)
            _index += groupSize
            advanceToItem()
            toReturn
        }

        private def advanceToItem() {
            while (_index < table_length && (table(_index) == LongHashSet.INVALID_ELEMENT)) {
                _index += groupSize
            }
        }
    }

    def this() = this(StandardLongHashSet.allocateTableMemory(LongHashSet.INITIAL_TABLE_SIZE), 0)

    def this(expectedSize: Int) = this(
                                    StandardLongHashSet.allocateTableMemory(
                                            HashSet.computeCapacityForSize(expectedSize, LongHashSet.INITIAL_TABLE_SIZE)), 0)


    def this(c: LongHashSet) {
        this(c.size)
        this += c
    }

    override def +=(c: LongHashSet) {
        ensureSizeFor(_size + c.size)
        if(c.isInstanceOf[StandardLongHashSet])
            internal_addAll(c.asInstanceOf[StandardLongHashSet].table)
        else {
            c foreach( this += _)
        }
    }

    def +=(c: Array[Long]) {
        ensureSizeFor(_size + c.length)
        internal_addAll(c)
    }

    override def +=(o: Long) {
        require(o != LongHashSet.INVALID_ELEMENT)
        ensureSizeFor(_size + 1)
        internal_add(o)
    }

    // add the elements without checking if there is enough space
    private def internal_addAll(elements: Array[Long]) {
        val length = elements.length
        var i = 0
        while (i < length) {
            if (elements(i) != LongHashSet.INVALID_ELEMENT)
                internal_add(elements(i))
            i += 1
        }
    }

    private def internal_add(o: Long) {
        val index = findOrEmpty(o)
        if (table(index) == LongHashSet.INVALID_ELEMENT) {
            _size += 1
            table(index) = o
        }
    }

    override def clear() {
        table = StandardLongHashSet.allocateTableMemory(LongHashSet.INITIAL_TABLE_SIZE)
        table_length_minus_1 = table.length - 1
        _size = 0
    }

    override def clear(new_expected_size: Int) {
        _size = 0
        table = null
        ensureSizeFor(new_expected_size)
    }

    override def contains(o: Long) = table(findOrEmpty(o)) != LongHashSet.INVALID_ELEMENT

    override def iter: HashSetIterator = new Iterator
    override def iter(groupID: Int, groupSize: Int): HashSetIterator = new Iterator(groupID, groupSize)

    private def ensureSizeFor(expectedSize: Int) {
        if (HashSet.sizeFitsIntoCapacity(expectedSize, table.length))
            return

        val old_table = table
        val old_size = _size
        table = StandardLongHashSet.allocateTableMemory(HashSet.computeCapacityForSize(expectedSize, table.size))
        table_length_minus_1 = table.length - 1
        _size = 0
        if (old_size != 0)
            internal_addAll(old_table)
        require(_size == old_size)
    }

    /// Returns the index in the table at which a particular item resides, or -1 if
    /// the item is not in the table.
    private def find(o: Long): Int = {
        val index = findOrEmpty(o)
        if (table(index) == LongHashSet.INVALID_ELEMENT)
            -1
        else
            index
    }

    /// Returns the index in the table at which a particular item resides, or the
    /// index of an empty slot in the table where this item should be inserted if
    /// it is not already in the table.
    /// @return index
    private def findOrEmpty(o: Long): Int = {
        @scala.annotation.tailrec
        def loop(index: Int): Int = {
            val existing = table(index)
            if (existing == LongHashSet.INVALID_ELEMENT || o == existing)
                index
            else
                loop((index + 1) & table_length_minus_1)
        }

        loop(getIndexFromLong(o))
    }

    private def getIndexFromLong(value: Long): Int = getIndex((value ^ (value >>> 32)).asInstanceOf[Int])
}
*/

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
        
        x.foreach(|i| assert!(i > 0 && i < 512));
        
        for i in 1u64..512u64 {
            assert!(x.contains(i));
        }
        
        assert!(!x.contains(EMPTY_STATE));
    }
}
