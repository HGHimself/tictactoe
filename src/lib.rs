#[macro_use] extern crate lazy_static;
use std::cmp;
use std::fmt;
use rand::Rng;
use wasm_bindgen::prelude::*;

mod heuristic_counter;
use heuristic_counter::{heuristic_counter_o, heuristic_counter_x};

#[wasm_bindgen]
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Symbol {
    X,
    O,
    E
}

type Board = Vec<Symbol>;

#[wasm_bindgen]
pub struct TicTacToe {
    length: u32,
    board: Board,
    depth: u32,
    player: Symbol,
    opponent: Symbol,
    turn: Symbol,
    winner: Symbol,
}

fn xy_to_linear(x: u32, y: u32, h: u32, w: u32) -> Option<usize> {
    if x > w - 1 || y > h - 1 {
        None
    } else {
        let i = (y * w) + x;
        Some(i as usize)
    }
}

fn opposite_symbol(sym: &Symbol) -> Symbol {
    match *sym {
        Symbol::X => Symbol::O,
        Symbol::O => Symbol::X,
        Symbol::E => panic!("You can't pick empty!")
    }
}

fn is_empty(sym: &Symbol) -> bool {
    match sym {
        Symbol::E => true,
        _ => false,
    }
}

fn player_heuristic(agent: &Symbol, input: &str) -> i32 {
    match agent {
        Symbol::X => heuristic_counter_x(input),
        Symbol::O => heuristic_counter_o(input),
        _ => panic!("Hey you cant use the empty char")
    }
}

fn heuristic(board: &Board, l: u32, agent: &Symbol) -> i32 {

    let mut heuristic_total = 0;

    for i in 0..l {
        let mut vertical_word = "_".to_string();
        let mut horizontal_word = "_".to_string();
        for j in 0..l {
            let vertical_index = xy_to_linear(i, j, l, l).unwrap();
            let horizontal_index = xy_to_linear(j, i, l, l).unwrap();
            // println!("{} {}", vertical_index, horizontal_index);

            vertical_word = format!("{}{:?}", vertical_word.clone(), board[vertical_index]);
            horizontal_word = format!("{}{:?}", horizontal_word.clone(), board[horizontal_index]);
        }
        vertical_word = format!("{}_", vertical_word.clone());
        horizontal_word = format!("{}_", horizontal_word.clone());
        let v_heuristic = player_heuristic(agent, &vertical_word);
        let h_heuristic = player_heuristic(agent, &horizontal_word);
        heuristic_total += (v_heuristic + h_heuristic);
        // println!("{} {}\n{} {}", vertical_word, v_heuristic, horizontal_word, h_heuristic);
    }
    // println!("---");

    for k in 0..(l * 2) {
        let mut vertical_word = "_".to_string();
        let mut horizontal_word = "_".to_string();
        for j in 0..k+1 {
            let i = k - j;
            if i < l && j < l {
                let vertical_d_index = xy_to_linear(i, j, l, l).unwrap();
                let horizontal_d_index = xy_to_linear(l - i - 1, j, l, l).unwrap();
                vertical_word = format!("{}{:?}", vertical_word.clone(), board[vertical_d_index]);
                horizontal_word = format!("{}{:?}", horizontal_word.clone(), board[horizontal_d_index]);
                // println!("{} {}", vertical_d_index, horizontal_d_index);
            }
        }

        vertical_word = format!("{}_", vertical_word.clone());
        horizontal_word = format!("{}_", horizontal_word.clone());

        let v_heuristic = player_heuristic(agent, &vertical_word);
        let h_heuristic = player_heuristic(agent, &horizontal_word);

        heuristic_total += (v_heuristic + h_heuristic);
        // println!("{} {}\n{} {}", vertical_word, v_heuristic, horizontal_word, h_heuristic);
    }

    heuristic_total
}

fn terminal(h: i32) -> bool {
    if h > 999 || h < -999 {
        true
    } else {
        false
    }
}

fn minimax(node: &Board, length: u32, depth: u32, maximizing: bool, agent: &Symbol) -> i32 {
    if depth == 0 {
        return heuristic(node, length, agent);
    }
    let h = heuristic(node, length, agent);
    if terminal(h) {
        return h
    }

    let boards = get_all_potential_moves(node, agent);
    if depth > 1 {
        println!("Examining {} moves at depth {}", boards.len(), depth);
    }

    if maximizing {
        let max_value = boards.iter().fold((-99999, vec![]), |acc, child| {
            let value = minimax(child, length, depth - 1, false, &opposite_symbol(agent));
            if value > acc.0 {
                (value, child.to_vec())
            } else {
                acc
            }
        });
        // println!("max {} {:?}", max_value.0, max_value.1);
        return max_value.0;
    } else {
        let min_value = boards.iter().fold((99999, vec![]), |acc, child| {
            let value = minimax(child, length, depth - 1, true, &opposite_symbol(agent));
            if value < acc.0 {
                (value, child.to_vec())
            } else {
                acc
            }
        });
        // println!("min {} {:?}", min_value.0, min_value.1);
        return min_value.0;
    }
}

fn get_all_potential_moves(board: &Board, marker: &Symbol) -> Vec<Board> {
    board.iter()
        .enumerate()
        .filter(|s| is_empty(s.1))
        .map(|tupl| {
            let mut newvec = board.clone();
            newvec[tupl.0] = *marker;
            newvec
        })
        .collect()
}

fn gen_random(length: u32) -> (u32, u32) {
    // lazy_static! {
    let mut rng = rand::thread_rng();
    // }

    let n: u32 = rng.gen();
    let x = 1 + (n % (length - 2));
    let h: u32 = rng.gen();
    let y = 1 + (h % (length - 2));

    (x, y)
}

#[wasm_bindgen]
impl TicTacToe {
    pub fn new(player: Symbol, length: u32, depth: u32) -> Self {
        let l = (length * length) as usize;
        let board = vec![Symbol::E; l];
        let opponent = opposite_symbol(&player);

        TicTacToe {
            length,
            board,
            depth,
            player,
            opponent,
            turn: Symbol::X,
            winner: Symbol::E,
        }
    }

    pub fn make_move(&mut self, x: u32, y: u32) -> () {
        if let Symbol::E = self.turn {
            return;
        }

        let index = xy_to_linear(x, y, self.length, self.length);

        if let Some(i) = index {
            self.board[i as usize] = self.turn;
            self.turn = opposite_symbol(&self.turn);
        } else {
            panic!("This is not a valid move!");
        }
    }

    pub fn agent_move(&mut self) -> i32 {
        if let Symbol::E = self.turn {
            return 0;
        }

        if terminal(heuristic(&self.board, self.length, &self.turn)) {
            self.turn = Symbol::E;
            return 0;
        }

        let mut sum = 0;
        let max_value = get_all_potential_moves(&self.board, &self.turn).iter().fold((-99999, vec![]), |acc, child| {
            let value = minimax(child, self.length, self.depth, false, &self.turn);
            print!("{} ", value);
            sum += value;
            if value > acc.0 {
                (value, child.to_vec())
            } else {
                acc
            }
        });
        println!("chosen {} {:?}", max_value.0, self.turn);

        if sum == 0 {
            let (x, y) = gen_random(self.length);
            while {
                !is_empty(&self.board[xy_to_linear(x, y, self.length, self.length).unwrap()])
            } {
                let (x, y) = gen_random(self.length);
            }

            self.make_move(x, y);
        } else {
            self.board = max_value.1;
            self.turn = opposite_symbol(&self.turn);
        }

        max_value.0
    }

    pub fn agent_move_2(&mut self) -> i32 {
        if let Symbol::E = self.turn {
            return 0;
        }

        if terminal(heuristic(&self.board, self.length, &self.turn)) {
            self.turn = Symbol::E;
            return 0;
        }
        // self.turn = opposite_symbol(&self.turn);

        let max_value = get_all_potential_moves(&self.board, &self.turn).iter().fold((-99999, vec![]), |acc, child| {
            let value = minimax(child, self.length, self.depth, false, &self.turn);
            // print!("{} ", value);
            // sum += value;
            if value > acc.0 {
                (value, child.to_vec())
            } else {
                acc
            }
        });

        // return max_value.0;

        // let mut sum = 0;

        // println!("chosen {} {:?}", max_value.0, self.turn);
        //
        // if sum == 0 {
        //     let (x, y) = gen_random(self.length);
        //     while {
        //         !is_empty(&self.board[xy_to_linear(x, y, self.length, self.length).unwrap()])
        //     } {
        //         let (x, y) = gen_random(self.length);
        //     }
        //
        //     self.make_move(x, y);
        // } else {
            self.board = max_value.1;
            self.turn = opposite_symbol(&self.turn);
        // }
        //
        max_value.0
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn is_playing(&self) -> bool {
        match self.turn {
            Symbol::E => false,
            _ => true,
        }
    }

    pub fn turn(&self) -> Symbol {
        self.turn
    }

    pub fn board(&self) -> *const Symbol {
        self.board.as_ptr()
    }
}

impl TicTacToe {
    fn get_board(&self) -> &Board {
        &self.board
    }
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for TicTacToe {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_string();

        for tupl in self.board.iter().enumerate() {
            let i = tupl.0;
            let square = tupl.1;
            let newline = "\n";
            if i % (self.length as usize) == 0 {
                string = format!("{}{}", string, newline);
            }
            let symbol = match square {
                Symbol::X => "🦐",
                Symbol::O => "🪐",
                Symbol::E => "⬜️",
            };

            string = format!("{}{}", string, symbol);
        }
        write!(f, "{}", &string)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xy_to_linear() {
        let w = 5;
        let h = 4;
        assert_eq!(xy_to_linear( 1, 1, h, w), Some(6));
        assert_eq!(xy_to_linear( 0, 0, h, w), Some(0));
        assert_eq!(xy_to_linear( 4, 0, h, w), Some(4));
        assert_eq!(xy_to_linear( 4, 2, h, w), Some(14));
        assert_eq!(xy_to_linear( 4, 4, h, w), None);
        assert_eq!(xy_to_linear( 5, 3, h, w), None);
        assert_eq!(xy_to_linear( 1, 2, h, w), Some(11));
    }

    #[test]
    fn test_move() {
        let l = 3;
        let mut ttt = TicTacToe::new(Symbol::X, l, 2);

        ttt.make_move(0, 0);
        ttt.make_move(1, 0);
        let board = ttt.get_board();

        assert_eq!(board[0], Symbol::X);
        assert_eq!(board[1], Symbol::O);
    }

    #[test]
    fn test_get_all_potential_moves() {
        let l = 3;
        let mut ttt = TicTacToe::new(Symbol::X, l, 2);

        ttt.make_move(0, 0);
        ttt.make_move(1, 0);

        let moves = get_all_potential_moves(ttt.get_board(), &Symbol::X);
        assert_eq!(moves.len(), 7);
        // println!("{:?}", moves);
        for (i, mov) in moves.iter().enumerate() {
            assert_eq!(mov[i+2], Symbol::X);
        }
    }

    #[test]
    fn test_heuristic() {
        let l = 6;
        let mut ttt = TicTacToe::new(Symbol::X, l, 2);

        ttt.make_move(2, 2); //x
        ttt.make_move(2, 3);
        ttt.make_move(3, 3); // x
        ttt.make_move(1, 2);
        ttt.make_move(1, 4); // x
        ttt.make_move(1, 3);
        ttt.make_move(2, 4); // x
        ttt.make_move(0, 4);
        ttt.make_move(1, 5); // x
        ttt.make_move(3, 4);

        assert_eq!(heuristic(ttt.get_board(), l, &Symbol::X), -8);
    }

    #[test]
    fn test_terminal() {
        let mut ttt = TicTacToe::new(Symbol::X, 6, 2);
        ttt.make_move(1, 2); //x
        ttt.make_move(0, 3);
        ttt.make_move(1, 3);
        ttt.make_move(3, 0);
        ttt.make_move(1, 4);
        ttt.make_move(0, 0);
        ttt.make_move(1, 5);

        assert_eq!(terminal(heuristic(ttt.get_board(), 6, &Symbol::X)), true);
    }

    #[test]
    fn test_agent_move() {
        let l = 6;
        let mut ttt = TicTacToe::new(Symbol::X, l, 2);

        ttt.make_move(2, 2); //x
        ttt.make_move(2, 3);
        ttt.make_move(3, 3); // x
        ttt.make_move(1, 2);
        ttt.make_move(1, 4); // x
        ttt.make_move(1, 3);
        ttt.make_move(2, 4); // x
        ttt.make_move(0, 4);
        ttt.make_move(1, 5); // x
        ttt.make_move(3, 4);

        // println!("{:?}", ttt.get_board());
        ttt.agent_move();
        // ttt.agent_move();
        // println!("{:?}", ttt.get_board());
    }
}
