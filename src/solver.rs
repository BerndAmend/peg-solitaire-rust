use board::*;
use boardset::*;

pub struct Solver<T: Board> {
    board: T,
    solution: Vec<BoardSet>,
}

impl<T: Board> Solver<T> {
    pub fn new(board: T) -> Solver<T> {
        Solver {
            board: board,
            solution: vec![]
        }
    }
}

/*
class Solver(val game: Board, val observer: StatusObserver, threadcount: Int) {
    require(threadcount >= 0)
    val thread_count = if (threadcount == 0) Runtime.getRuntime.availableProcessors else threadcount

    ///  solution(0) is unused
    val solution = Array.fill(game.length)(LongHashSet.newInstance)

    /// used to count in the ctor the dead ends
    private val deadends = Array.fill[Long](game.length)(0L)

    def this(game: Board, observer: StatusObserver) = this(game, observer, 0)

    def this(game: Board, startFields: Iterable[Long], end_pegs: Int, endField: Long,
            observer: StatusObserver, threadcount: Int = 0) {
        this(game, observer, threadcount)

        require(startFields != null && !startFields.isEmpty)

        val start_set = game.length - java.lang.Long.bitCount(startFields.head)
        require(start_set > 0)
        for (e <- startFields) {
            require(game.length - java.lang.Long.bitCount(e) == start_set)
            solution(start_set) += game.getNormalform(e)
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
            if(endField != LongHashSet.INVALID_ELEMENT)
                solution(end_set + 1) += game.getNormalform(endField)

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

    /// @return a list of all possible start-fields
    def getStart: LongHashSet = game.getCompleteList(solution(getStartNum))

    /// @return first non empty solution set id
    def getStartNum: Int = {
        var start = 0
        while (solution(start) == null || solution(start).size == 0) {
            start += 1

            if (start == game.length)
                throw new Exception("no set field found")
        }

        start
    }

    ///  @return a list of all possible end-fields
    def getEnd: LongHashSet = game.getCompleteList(solution(getEndNum))

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
    def getFollower(field: Long): LongHashSet = {
        val fieldPos = game.length - java.lang.Long.bitCount(field)
        if (fieldPos + 1 >= game.length)
            return LongHashSet.newInstance
        val next = solution(fieldPos + 1)
        if (next == null)
            return LongHashSet.newInstance

        game.getFollower(field, next)
    }

    /// @return all follower for a provided field
    def getPredecessor(field: Long): LongHashSet = {
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
                    var result = if(thread_count == 1) solution(sol) else LongHashSet.newInstance
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

    @scala.annotation.tailrec
    private def cleanBackward(pos: Int) {
        observer.begin_backward_cleaning_step(pos - 1)

        val previous = solution(pos - 1)
        val current = solution(pos)
        val old_size= previous.size

        @scala.annotation.tailrec
        def loop(iter: HashSetIterator, result: LongHashSet): LongHashSet = {
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

}


lazy val possibleStartFields = {
    val hashSet = LongHashSet.newInstance

    val base = (1L << length) - 1L

    for (i <- 0 until length)
        hashSet += boardHelper.getNormalform(base ^ (1L << i))

    hashSet
}



final def getNormalform(field: Long) = boardHelper.getNormalform(field)

final def addFollower(field: Long, sol: LongHashSet): Unit = applyMoves(field, field){sol += getNormalform(_)}

/// return true if field has a follower/predecessor in the solutions HashSet
final def hasFollower(field: Long, solutions: LongHashSet): Boolean = {
    var i = -1
    while (i < movemask_size-1) {
        i += 1
        applyMove(field, field, i)(n => if(solutions.contains(getNormalform(n))) i=Int.MaxValue)
    }
    i == Int.MaxValue
}

/// Returns a list of all related fields for the given field.
private final def getRelatedFields(checkfield: Long, field: Long, searchSet: LongHashSet): LongHashSet = {
    var result = LongHashSet.newInstance
    // get all related fields
    applyMoves(checkfield, field){n => if(searchSet.contains(getNormalform(n))) result += n}
    result
}

final def getFollower(field: Long, searchSet: LongHashSet): LongHashSet = getRelatedFields(field, field, searchSet)

final def getPredecessor(field: Long, searchSet: LongHashSet): LongHashSet = getRelatedFields(~field, field, searchSet)

/// @return a complete list with all equivalent fields for the fields HashSet
def getCompleteList(fields: LongHashSet): LongHashSet = {
    val output = LongHashSet.newInstance
    fields foreach (output += boardHelper.getEquivalentFields(_))
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
*/
