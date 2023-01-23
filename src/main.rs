use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

struct Game {
    board: [[u32; 4]; 4],
    score: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[0; 4]; 4],
            score: 0,
        }
    }

    fn print(&self) {
        // print the game board
    }

    fn move_up(&mut self) {
        // move the tiles up and merge them
    }

    fn move_down(&mut self) {
        // move the tiles down and merge them
    }

    fn move_left(&mut self) {
        // move the tiles left and merge them
    }

    fn move_right(&mut self) {
        // move the tiles right and merge them
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        loop {
            self.print();
            write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
            stdout.flush().unwrap();

            let key = stdin.keys().next().unwrap().unwrap();

            match key {
                Key::Up => self.move_up(),
                Key::Down => self.move_down(),
                Key::Left => self.move_left(),
                Key::Right => self.move_right(),
                _ => {}
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}

