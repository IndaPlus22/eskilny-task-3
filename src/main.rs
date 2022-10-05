mod lib;

fn main() {
    let mut game = lib::Game::new();

    loop {
        use std::io;
        use std::io::prelude::*;

        let input = io::stdin();
        let mut lines = input.lock().lines(); // we've built an iterator over the lines input to stdin

        println!(
            "This is the current board. It is {}'s turn.",
            game.get_active_colour()
        );
        println!("{}", game);
        println!("Please input your move (on the format 'XF XF' where X is a character and F is a number).");

        // read next input
        let input_tmp = lines
            .next() // we iterate over the first line
            .expect("Invalid iostream.")
            .expect("Error."); // expect errors
        let input: Vec<&str> = input_tmp
            .trim() // remove whitespaces
            .split(" ")
            .collect();

        // provide state and colour reading to user
        if input[0] == "state" {
            println!("{:?}", game.get_game_state());
        } else if input[0] == "colour" {
            println!("{:?}", game.get_active_colour());
        } else if input[0] == "gm" {
            println!("{:?}", game.get_possible_moves(lib::Position::parse_str(input[1]).unwrap(), 0));
        } else if input[0] == "piece" {
            println!("{:?}", game.get_board()[lib::Position::parse_str(input[1]).unwrap().idx]);
        } else if input.len() == 2 {
            // try to make the move
            match game.make_move(input[0], input[1]) {
                Err(message) => println!("Error received: \n'{}'\nPlease try again!", message),
                Ok(_) => println!("Succeeded in moving the piece!"),
            };
        } else {
            println!("Invalid input. Please try again!");
        }

        // if the game is waiting on a pawn promotion, make the user fix this!
        while game.get_game_state() == lib::GameState::WaitingOnPromotionChoice {
            println!("What would you like to promote the pawn to?");

            // read next input
            let input_tmp = lines
                .next() // we iterate over the first line
                .expect("Invalid iostream.")
                .expect("Error."); // expect errors
            let input: Vec<&str> = input_tmp
                .trim() // remove whitespaces
                .split(" ")
                .collect();

            match game.set_promotion(String::from(input[0])) {
                Ok(_) => println!("Successfully promoted the piece!"),
                Err(msg) => println!("Error received:\n{}\nPlease try again!", msg),
            }
        }
    }
}
