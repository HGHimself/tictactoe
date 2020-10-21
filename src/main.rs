use tictactoe::{TicTacToe, Symbol};

fn main() {
    println!("Hello, world!");

    let mut ttt = TicTacToe::new(Symbol::X, 6, 2);
    // println!("{}", ttt);
    // ttt.make_move(1,1);
    // ttt.make_move(1,2);
    // ttt.make_move(2,1);
    // ttt.make_move(2,3);
    println!("{}\n\n", ttt);
    while ttt.is_playing() {
        ttt.agent_move();
        println!("{}\n\n", ttt);
    }

}
