use tictactoe::{TicTacToe, Symbol};

fn main() {
    println!("Hello, world!");

    let mut ttt = TicTacToe::new(Symbol::X, 6, 2);
    println!("{}", ttt);
    loop {
        ttt.agent_move();
        println!("{}\n\n", ttt);
    }

}
