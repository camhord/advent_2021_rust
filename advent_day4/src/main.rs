use std::fs;
use std::collections::HashMap;

fn main() {
    let path = "input/day4";
    let input = fs::read_to_string(path)
        .expect("Something went wrong reading from file");

    let sequence: Vec<&str> = input.split('\n').collect::<Vec<&str>>()[0].split(",").collect();


    let board_str = input.split("\n\n").skip(1).collect::<Vec<&str>>();
    let mut boards: Vec<Board> = board_str.into_iter().map(|b| create_board(b.trim().split('\n').collect())).collect();

    let mut bingo_count = 0;
    let board_count = boards.len();

    'outer: for num in sequence{
        let parsed = num.parse::<i32>().unwrap();

        for i in 0..boards.len(){
            let board = &mut boards[i];

            if !board.bingoed && board.Mark(parsed){
                bingo_count = bingo_count + 1;

                if bingo_count == board_count{
                    println!("WORST BINGO FOUND!: {}", board.Score(parsed));
                    break 'outer;
                }
            }
        }
    }
}

struct Board {
    coords: HashMap<i32, (usize, usize)>,
    board: Vec<Vec<bool>>,
    bingoed: bool
}

impl Board{
    fn LoadBoard(&mut self, board: Vec<&str>){
        let size = board.len();

        for row in 0..size{
            let line: Vec<&str> = board[row].split_whitespace().collect();
            for col in 0..size{
                self.coords.insert(line[col].parse::<i32>().unwrap(), (row, col));
            }
        }
    }

    fn Mark(&mut self, value: i32) -> bool{
        if self.bingoed || !self.coords.contains_key(&value){
            return self.bingoed;
        }

        let coords = self.coords[&value];
        self.board[coords.0][coords.1] = true;

        self.bingoed = self.CheckBingo(value);

        return self.bingoed;
    }

    fn CheckBingo(&self, last_mark: i32) -> bool{
        let coords = self.coords[&last_mark];

        let mut col_bingo = true;

        for row in self.board.iter(){
            col_bingo = col_bingo && row[coords.1];
        }

        return col_bingo || !self.board[coords.0].iter().any(|&x| !x);
    }

    fn Score(&self, last: i32) -> i32{
        let mut sum = 0;
        for key in self.coords.keys(){
            let coords = self.coords[key];
            if !self.board[coords.0][coords.1]{
                sum = sum + key;
            }
        }

        return sum * last;
    }
}

fn create_board(board_str: Vec<&str>) -> Board{
    let size = board_str.len();
    let mut board: Board = Board{
        coords: HashMap::new(),
        board: vec![vec![false; size]; size],
        bingoed: false
    };

    board.LoadBoard(board_str);

    return board;
}
