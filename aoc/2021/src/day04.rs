use::std::fs;
use::std::collections::HashSet;

// typedef so we can organize the whole bingo game as a Vec<Board>
#[derive(Clone)]
struct Board {
    rows: Vec<HashSet<i32>>,
    cols: Vec<HashSet<i32>>,
}

impl Board {
    fn parse(lines: &mut core::str::Lines) -> Option<Board> {
        let nums = lines
            .take(6)                        // break into chunks of 5 for each board
            .filter(|l| !l.is_empty())      // get rid of empty lines
            .map(|l| {
                l.split_whitespace()        // for each line, split into nums
                    .map(|n| 
                         n.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()      // parse and collect into vector
            })
            .collect::<Vec<Vec<i32>>>();    // and collect each set of 5 vectors into another vector (board)
        if nums.len() == 0 { return None; }
        let mut rows: Vec<HashSet<i32>> = Vec::new();
        let mut cols: Vec<HashSet<i32>> = Vec::new();

        // row wise
        for row in &nums {
            let mut set = HashSet::new();
            for col in row {
                set.insert(*col);
            }
            rows.push(set);
        }

        // col wise
        for col in 0..nums[0].len() {
            let mut set = HashSet::new();
            for row in 0..nums[col].len() {
                set.insert(nums[row][col]);
            }
            cols.push(set);
        }

        Some(Board {
            rows,
            cols,
        })
    }

    fn draw(&mut self, n: i32) {
        self.rows.iter_mut().for_each(|row| {
            row.remove(&n);
        });
        self.cols.iter_mut().for_each(|col| {
            col.remove(&n);
        });
    }

    fn has_complete(&self) -> bool {
        let mut complete = false;
        for row in self.rows.iter() {
            if row.is_empty() {
                complete = true;
            }
        }
        for col in self.cols.iter() {
            if col.is_empty() {
                complete = true;
            }
        }

        complete
    }

    fn sum_remaining(&self) -> i32 {
        self.rows.iter().flatten().sum()
    }
}

fn main() {
    // get iter of lines
    let input = fs::read_to_string("input/day04.txt")
        .expect("File not found!");
    let mut lines = input.lines();
    // parse first line into vec of rand numbers to "call"
    let drawn: Vec<i32> = lines
        .nth(0)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>() .unwrap())
        .collect();

    let mut game: Vec<Board> = Vec::new();
    // parse rest into Vec<Board>
    while let Some(board) = Board::parse(&mut lines) {
        game.push(board);
    }

    // part 1
    {
        // loop over all drawn numbers
        let mut game_pt1 = game.clone();
        'pt1: for n in drawn.iter() {
            for board in game_pt1.iter_mut() {
                board.draw(*n);
                if board.has_complete() {
                    println!("part 1: {}", *n * board.sum_remaining());
                    break 'pt1;
                }
            }
        }
    }

    // part 2
    {
        // loop over all drawn numbers
        let mut game_pt2 = game.clone();
        let mut result = 0;
        for n in drawn.iter() {
            let mut to_rem = Vec::new();
            for (i, board) in game_pt2.iter_mut().enumerate() {
                board.draw(*n);
                if board.has_complete() {
                    result = *n * board.sum_remaining();
                    to_rem.push(i);
                }
            }

            // after looping through all boards, remove so we update game_pt2
            while let Some(i) = to_rem.pop() {
                game_pt2.remove(i);
            }
        }
        println!("part 2: {}", result);
    }
}
