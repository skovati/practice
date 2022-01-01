use::std::fs;
use::std::fmt::Debug;
use::std::fmt;

// entry struct to store bingo info
struct Entry {
    num: i32,
    called: bool,
}

impl Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "val: {}, called: {}", self.num, self.called)
    }
}

// typedef so we can organize the whole bingo game
// as a Vec<Board>
type Board = Vec<Vec<Entry>>;

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
        let boards = lines
            .filter(|l| !l.is_empty())      // get rid of empty lines
            .collect::<Vec<&str>>()
            .chunks(5)                      // break into chunks of 5 for each board
            .map(|s| s.into())
            .collect::<Vec<Vec<&str>>>();
        
        for board in boards {
            game.push(
                board.iter().map(|row| {
                    row.split_whitespace().map(|n| Entry {
                        num: n.parse::<i32>().expect(n),
                        called: false,
                    }).collect()
                }).collect()
            );
        }

        // loop, updating all boards on each draw
        for num in drawn {
            for board in game {
                for row in board {
                    for mut entry in row {
                        if entry.num == num {
                            entry.called = true;
                        }
                    }
                }
            }
        }
        
        // if all of a row or col are have called == true, break and calculate score
}
