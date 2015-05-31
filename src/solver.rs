use board::*;
use boardset::*;
use std::rc::Rc;

pub struct Solution<T: Board> {
    board: Rc<T>,
    solution: Vec<BoardSet>,

    /// number of dead ends
    dead_ends: Vec<u64>,
}

pub struct Solver<T: Board> {
    board: Rc<T>,
}

impl<T: Board> Solver<T> {
    pub fn new(board: Rc<T>) -> Solver<T> {
        Solver {
            board: board,
        }
    }

    pub fn possible_start_fields(&self) -> BoardSet {
        let mut set = BoardSet::with_capacity(self.board.description().pegs);

        let base = (1u64 << (self.board.description().pegs + 1)) - 1u64;

        for i in 0..self.board.description().pegs {
            set.fast_insert(self.board.normalize(base ^ (1u64 << i)));
        }

        set
    }

    pub fn solve(&mut self, start_fields: &[State]) -> Solution<T> {

        assert!(!start_fields.is_empty());

        let mut solution = vec![BoardSet::new(); self.board.description().pegs];
        let mut dead_ends = vec![0u64; self.board.description().pegs];

        Solution {
            board: self.board.clone(),
            solution: solution,
            dead_ends: dead_ends
            }
    }

// TODO
/*
    fn add_follower(field: State, sol: BoardSet) {
        applyMoves(field, field){sol += normalize(_)};
    }

    /// return true if field has a follower/predecessor in the solutions HashSet
    fn has_follower(field: State, solutions: BoardSet) -> bool {
        var i = -1
        while (i < movemask_size-1) {
            i += 1
            applyMove(field, field, i)(
                n => if(solutions.contains(normalize(n)))
                    i=Int.MaxValue
            )
        }
        i == Int.MaxValue
    }
*/
}

impl<T: Board> Solution<T> {

}

/*
def this(game: Board, startFields: Iterable[Long], end_pegs: Int, endField: Long,
        observer: StatusObserver, threadcount: Int = 0) {

    val start_set = game.length - java.lang.Long.bitCount(startFields.head)
    require(start_set > 0)
    for (e <- startFields) {
        require(game.length - java.lang.Long.bitCount(e) == start_set)
        solution(start_set) += game.normalize(e)
    }

    val clean = end_pegs >= 0

    var end_set = game.length - (if(clean) end_pegs else 0)
    if(endField != EMPTY_STATE) {
        end_set = game.length - java.lang.Long.bitCount(endField) - 1
    }
    require(end_set > start_set)

    if(end_set >= game.length)
        end_set -= 1

    observer.begin_forward_calculation()
    Time(observer.end_forward_calculation _) {
        for (sol <- (getStartNum + 1) to end_set) {
            observer.begin_forward_calculation_step(sol)
            calculateForward(sol)
            observer.end_forward_calculation_step(sol, solution(sol))
        }
    }

    if(clean) {
        if(endField != EMPTY_STATE)
            solution(end_set + 1) += game.normalize(endField)

        // cleaning the gamefield after every step is useless
        observer.begin_backward_cleaning()
        Time(observer.end_backward_cleaning _) {
            cleanBackward(getEndNum)
        }

        if(solution(getEndNum-1).isEmpty)
            throw new Exception("Entered end field is not reachable.")

        observer.dead_ends(deadends.sum)
    }
}

/// @return first non empty solution set id
def getStartNum: Option<Int> = {
    var start = 0
    while (solution(start) == null || solution(start).size == 0) {
        start += 1

        if (start == game.length)
            throw new Exception("no set field found")
    }

    start
}


/// @return last non empty solution set id
def getEndNum: Int = {
    var end = game.length - 1
    while (solution(end) == null || solution(end).size == 0) {
        end -= 1

        if (end == -1)
            throw new Exception("solution is empty")
    }

    end
}

/// @return all follower for a provided field
def getFollower(field: Long): BoardSet = {
    val fieldPos = game.length - java.lang.Long.bitCount(field)
    if (fieldPos + 1 >= game.length)
        return BoardSet::new()
    val next = solution(fieldPos + 1)
    if (next == null)
        return BoardSet::new()

    game.getFollower(field, next)
}

/// @return all follower for a provided field
def getPredecessor(field: Long): BoardSet = {
    val fieldPos = game.length - java.lang.Long.bitCount(field)
    if (fieldPos - 1 <= 0)
        return LongHashSet.newInstance
    game.getPredecessor(field, solution(fieldPos - 1))
}

private def calculateForward(sol: Int) {
    val current = solution(sol)
    (0 until thread_count).map {
        threadID =>
            future {
                val iter = solution(sol - 1).iter(threadID, thread_count)
                var result = if(thread_count == 1) solution(sol) else BoardSet::new()
                while (iter.hasNext) {
                    game.addFollower(iter.unsafe_next, result)
                }
                if(thread_count > 1) {
                    current.synchronized {
                        current += result
                    }
                }
                Unit
            }
    }.foreach(Await.result(_, Duration.Inf))
}

private def cleanBackward(pos: Int) {
    observer.begin_backward_cleaning_step(pos - 1)

    val previous = solution(pos - 1)
    val current = solution(pos)
    val old_size= previous.size

    @scala.annotation.tailrec
    def loop(iter: HashSetIterator, result: BoardSet): BoardSet = {
        if(iter.hasNext) {
            val elem = iter.unsafe_next
            if (game.hasFollower(elem, current))
                result += elem
            loop(iter, result)
        } else
            result
    }

    if(thread_count > 1) {
        val r = (0 until thread_count).map(
                id => future(loop(previous.iter(id, thread_count), LongHashSet.newInstance))
        )
        previous.clear( r.foldLeft(0)(_ + Await.result(_, Duration.Inf).size) )
        r foreach (previous += Await.result(_, Duration.Inf))
    } else {
        val new_previous = LongHashSet.newInstance
        loop(previous.iter, new_previous)
        solution(pos - 1) = new_previous
    }

    val deadEndFields = old_size - solution(pos - 1).size
    deadends(pos - 1) = deadEndFields

    observer.end_backward_cleaning_step(pos - 1, deadEndFields)

    if (deadEndFields != 0L) cleanBackward(pos-1) else return
}




/// Returns a list of all related fields for the given field.
private final def getRelatedFields(checkfield: Long, field: Long, searchSet: BoardSet): BoardSet = {
    var result = BoardSet::new()
    // get all related fields
    applyMoves(checkfield, field){n => if(searchSet.contains(normalize(n))) result += n}
    result
}

final def getFollower(field: Long, searchSet: BoardSet): BoardSet =
        getRelatedFields(field, field, searchSet)

final def getPredecessor(field: Long, searchSet: BoardSet): BoardSet =
        getRelatedFields(~field, field, searchSet)

/// @return a complete list with all equivalent fields for the fields HashSet
def getCompleteList(fields: BoardSet): BoardSet = {
    val output = LongHashSet.newInstance
    fields foreach (output += boardHelper.equivalent_fields(_))
    output
}

private final def applyMove(checkfield: Long, field: Long, i: Int)(cmd: Long => Unit) {
    val mask = movemask(i)
    val tmp = checkfield & mask
    if (tmp == checkmask1(i) || tmp == checkmask2(i))
        cmd(field ^ mask)
}

private final def applyMoves(checkfield: Long, field: Long)(cmd: Long => Unit) {
    var i = 0
    while (i < movemask_size) {
        applyMove(checkfield, field, i)(cmd)
        i += 1
    }
}

/// @return a list of all possible start-fields
def getStart: BoardSet = game.getCompleteList(solution(getStartNum))
///  @return a list of all possible end-fields
def getEnd: BoardSet = game.getCompleteList(solution(getEndNum))
*/