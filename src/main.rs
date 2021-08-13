#![feature(test)]


fn main() {
    let mut s = vec![ vec!['.'; 9] ; 9];
    Solution::solve_sudoku(& mut s);
    for row in & s {
        println!("{:?}",  row);
    }
    
}


struct Sudoku {
    board : Vec<Vec<Option<u8>>>,
}

impl Sudoku {

    pub fn new(input : & Vec<Vec<char>>) -> Self {
        let mut board : Vec<Vec<Option<u8>>> = Vec::with_capacity(9);
        for v in input {
            let mut row : Vec<Option<u8>> = Vec::with_capacity(9);
            for c in v {
                row.push( match c {
                    '.' => None,
                    c if c.is_ascii_digit() => Some(*c as u8 - '0' as u8),
                    _ =>  { panic!("Invalid char in input."); }
                });
            }
            board.push(row);
        }
        Sudoku { board, }
    }

    pub fn output(&self) -> Vec<Vec<char>> {
        self.board.iter().map(|r| {
            r.iter().map(|v| {
                match v {
                    Some(v) => ('0' as u8 + v) as char,
                    None => '.',
                }
            }).collect::<Vec<char>>()
        }).collect::<Vec<Vec<char>>>()
    }

    pub fn output_mut(&self, out : &mut Vec<Vec<char>>) {
        for r in 0..9 {
            for c in 0..9 {
                out[r][c] = match self.value_at(r as u8, c as u8) {
                    Some(v) => ('0' as u8 + v) as char,
                    None => '.', 
                }
            }
        }
    }

    #[inline]
    pub fn value_at(&self, i : u8, j : u8) -> Option<u8> {
        self.board[i as usize][j as usize]    
    }

    pub fn set_value_at(& mut self, val : Option<u8>, i : u8, j : u8) {
        self.board[i as usize][j as usize] = val;
    }

    #[inline]
    fn cell_to_coords(cell : u8) -> (u8, u8) {
        (cell % 9, cell / 9)
    }

    pub fn depth_first(& mut self, cell : u8) -> bool {
        if cell == 81 { return true; } // All the cells are filled
        let (i, j) = Self::cell_to_coords(cell);
        if self.value_at(i, j).is_some() { // if the current cell is already filled, skip it
            return self.depth_first(cell+1);
        }
        // Get a list of all the numbers that could be at this location
        // and try to solve with each of them in this position
        let p = self.get_possibles(i, j);
        for v in p {
            self.set_value_at(Some(v), i, j);
            if self.depth_first(cell + 1) { return true; }    
        }
        // We didn't solve it with the current board state, so restore
        // the previous state and return to allow the parent function
        // to resolve it;
        self.set_value_at(None, i, j);
        false
    }

    fn get_possibles(&self, i : u8, j : u8) -> Vec<u8> {
        let mut flags : [bool; 10] = [true; 10];
        for k in 0..9 {
            if self.value_at(i, k).is_some() { flags[self.value_at(i, k).unwrap() as usize] = false; }
            if self.value_at(k,j).is_some() { flags[self.value_at(k,j).unwrap() as usize] = false; }
            
        }
        // Determine which 3x3 cell i,j is in
        let icell = i / 3; // Note integer division
        let jcell = j / 3; 
        for m in 0..3 {
            for n in 0..3 {
                let x = m + (3 * icell);
                let y = n + (3 * jcell);
                if self.value_at(x,y).is_some() { 
                    flags[self.value_at(x,y).unwrap() as usize] = false; }
            }
        }
        flags.iter()
            .enumerate()
            .filter(|(n, b)| (*n > 0) && **b)
            .map(|(n, _)| n as u8)
            .collect()
    }




}
struct Solution;

impl Solution {
    pub fn solve_sudoku(board: & mut Vec<Vec<char>>)  {
        let mut s = Sudoku::new(board);
        s.depth_first(0);
        s.output_mut(board);
    }  
}

#[cfg(test)]

mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    fn get_example() -> Vec<Vec<char>> {  
        vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ]
    }
    
    fn get_example_solution() -> Vec<Vec<char>> {
        vec![
            vec!['5','3','4','6','7','8','9','1','2'],
            vec!['6','7','2','1','9','5','3','4','8'],
            vec!['1','9','8','3','4','2','5','6','7'],
            vec!['8','5','9','7','6','1','4','2','3'],
            vec!['4','2','6','8','5','3','7','9','1'],
            vec!['7','1','3','9','2','4','8','5','6'],
            vec!['9','6','1','5','3','7','2','8','4'],
            vec!['2','8','7','4','1','9','6','3','5'],
            vec!['3','4','5','2','8','6','1','7','9']
        ]
    }

    fn get_empty_board() -> Vec<Vec<char>> {
        vec![ vec!['.'; 9] ; 9]
    }

    #[test]
    fn test_cell_to_coords() {
        assert_eq!((0, 0), Sudoku::cell_to_coords(0));
        assert_eq!((0, 1), Sudoku::cell_to_coords(9));
        assert_eq!((1, 0), Sudoku::cell_to_coords(1));
        assert_eq!((8, 8), Sudoku::cell_to_coords(80));
    }

    #[test]
    fn test_new_sudoku() {
        let s = Sudoku::new(& get_example());
        assert_eq!(Some(5), s.value_at(0,0));
        assert_eq!(Some(6), s.value_at(1,0));
        assert_eq!(None, s.value_at(3,2));
        assert_eq!(Some(9), s.value_at(8,8));
    }
    #[test]
    fn test_get_possibles() {
        let s = Sudoku::new(& get_example());
        assert_eq!(vec![2,4,7], s.get_possibles(1, 1));
        assert_eq!(vec![2,4,7], s.get_possibles(1, 1));
    }

    #[test]
    fn test_get_output() {
        let s = Sudoku::new(& get_example());
        assert_eq!(get_example(), s.output());
    }

    #[test]
    fn test_solve() {
        let mut e = get_example();
        Solution::solve_sudoku(& mut e );
        assert_eq!(e, get_example_solution());
    }

    #[bench]
    fn bench_solve(bencher : & mut Bencher) {
        bencher.iter(|| {
            let mut e = get_example();
            Solution::solve_sudoku(& mut e );
        });
    }

    #[bench]
    fn bench_solve_empty(bencher : & mut Bencher) {
        bencher.iter(|| {
            let mut b = get_empty_board();
            Solution::solve_sudoku(& mut b );
        });
    }
}

