use reversi::{State, Turn};
use std::io::{self, Write};

fn main() {
    let mut game = Game::new();

    // Start a new game
    println!("Welcome to Reversi!");

    // Play rounds until the game is over
    while !game.state.is_over() {
        game.play_round();
    }

    // Announce the winner
    print!("Game over: ");
    if let Some(player) = game.state.get_winner() {
        println!("{:?} wins!", player);
    } else {
        println!("It's a tie!");
    }
}

/// Store the current game as a State with a collection of turns.
struct Game {
    state: State,
    turns: Vec<Turn>,
}

impl Game {
    /// Create a new Game.
    fn new() -> Game {
        Game {
            state: State::new(),
            turns: Vec::new(),
        }
    }

    /// Play a round of the game.
    fn play_round(&mut self) {
        // Print current state of game
        println!("Turn #{}", self.turns.len() + 1);
        println!();
        println!("{}", self.state);

        // Get all legal turns
        let legal_turns = self.state.get_legal_turns();
        // If none are avaliable...
        if legal_turns.is_empty() {
            println!("No avaliable turns for {:?}", self.state.get_player());
            // ... switch to the next player...
            self.state.switch_player();
            // ... then return
            return;
        }

        // Print avaliable turns
        println!("Avaliable turns for {:?}:", self.state.get_player());
        for turn in self.state.get_legal_turns() {
            println!("{}", turn);
        }

        // Prompt user to take their turn
        print!("Take your turn: ");
        io::stdout().flush().unwrap();

        let mut turn = self.get_turn();

        while (turn == Turn::Invalid) || !self.state.is_legal(&turn) {
            print!("Invalid. Please try again: ");
            io::stdout().flush().unwrap();

            turn = self.get_turn();
        }

        // Play turn
        self.set_turn(turn);

        println!();
        println!();
    }

    /// Prompt the player for their turn.
    fn get_turn(&self) -> Turn {
        loop {
            // Get user input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error: coult not parse input.");

            // Process input
            let input = input.trim().as_bytes();

            if input.len() != 2 {
                print!("Bad input. Please try again: ");
                io::stdout().flush().unwrap();

                continue;
            }

            return Turn::new(
                input[1].checked_sub(b'1').unwrap_or(0) as usize,
                input[0].checked_sub(b'a').unwrap_or(0) as usize,
            );
        }
    }

    /// Play a turn of the game.
    fn set_turn(&mut self, turn: Turn) {
        self.state.play(&turn);

        self.turns.push(turn);
    }
}

impl From<State> for Game {
    /// Create a game from an existing State.
    fn from(state: State) -> Self {
        Game {
            state,
            turns: Vec::new(),
        }
    }
}
