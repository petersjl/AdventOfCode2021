use std::fs;

fn main() {
    let contents = fs::read_to_string("bingo.txt").expect("Problem reading file");
    let mut list = contents.split("\n");
    let numbers = list.next().unwrap().split(",");
    list.next();
    let mut boards = collect_boards(&mut list);

    let mut winning_number = 0;
    let mut winning_index = 1000;
    'items: for item in numbers{
        let number = item.parse::<u8>().unwrap();
		let mut n = 0;
        loop {
			if n == boards.len() {break;}
            let board = &mut boards[n];
            board.mark_num(number);
            //println!("Board {} marked {}", n, number);
            //board.print_marks();
            if board.check_win() {
				if boards.len() == 1 {
					winning_number = number;
                	winning_index = n;
					break 'items;
				}else{
					boards.swap_remove(n);
				}
            }else{
				n += 1;
			}
        }
    }
    println!("Board {} is the last to win on number {}", winning_index, winning_number);
    let board = &boards[winning_index];
    let mut sum : usize = 0;

    for r in 0..5 {
        for c in 0..5 {
            if !board.check(r,c) {sum += usize::from(board.get_num(r,c));}
        }
    }

    println!("Unmarked spaces add to {}\nMultiplied by winning number is {}", sum, sum * usize::from(winning_number));
}

fn collect_boards(list: &mut dyn Iterator<Item = &str>) -> Vec<Board>{
    let mut boards : Vec<Board> = Vec::new();
    loop{
        let test = list.next();
        if test.is_none() {break;}
        let mut board = Board {board : Vec::new()};
        board.push(//push the following onto the board
            vec_to_board_line(//change the following to a board line
                test.unwrap()//unwrap string from optional
                .split_whitespace()//split items from white space
                .flat_map(|x| x.parse::<u8>())//parse all items to u8s
                .collect()));//collect to a vec
        for _n in 1..5{
            board.push(vec_to_board_line(list.next().unwrap().split_whitespace().flat_map(|x| x.parse::<u8>()).collect()))
        }
        boards.push(board);
        list.next();        
    }
    return boards
}

fn vec_to_board_line(vec: Vec<u8>) -> Vec<(bool, u8)>{
    let mut line : Vec<(bool, u8)> = Vec::new();
    for n in vec {
        line.push((false, n));
    }
    line
}

struct Board {
    board: Vec<Vec<(bool, u8)>>
}

impl Board {
    fn get_num(&self, r: usize, c: usize) -> u8 {
        self.board[r][c].1
    }
    fn check(&self, r: usize, c: usize) -> bool{
        self.board[r][c].0
    }
    // fn mark(&mut self, r: usize, c: usize){
    //     self.board[r][c].0 = true;
    // }
    fn mark_num(&mut self, val: u8){
        'rows: for line in &mut self.board {
            for mut num in line {
                if num.1 == val {num.0 = true; break 'rows;}
            }
        }
    }
    fn push(&mut self, line: Vec<(bool, u8)>){
        self.board.push(line);
    }
    fn check_win(&self) -> bool {
        //check rows
        'row: for row in 0..5 {
            for col in 0..5 {
                if !self.check(row, col) {continue 'row;}
            }
            // self.print();
            // self.print_marks();
            // println!("Won with row {}", row);
            return true
        }
        'col: for col in 0..5 {
            for row in 0..5 {
                if !self.check(row, col) {continue 'col;}
            }
            // self.print();
            // self.print_marks();
            // println!("Won with col {}", col);
            return true
        }

        // let mut count1 = 0;
        // let mut count2 = 0;
        // for num in 0..5 {
        //     if self.check(num, num) {count1 += 1};
        //     if self.check(num, 4 - num) {count2 += 1;}
        // }

        // if (count1 == 5) || (count2 == 5) {
        //     self.print();
        //     self.print_marks();
        //     println!("Won with a diag");
        //     return true
        // }

        return false
    }
    // fn print(&self) {
    //     for line in &self.board{
    //         for num in line {
    //             print!("{} ", num.1);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
    // fn print_marks(&self){
    //     for line in &self.board{
    //         for num in line {
    //             print!("{} ", if num.0 {'x'} else {'-'});
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
}