mod variants {
    pub mod cyoa;
    pub mod quiz;
}
pub mod toolbox;
pub mod types;

use variants::cyoa;
use variants::quiz;

use crate::toolbox::readline;
use crate::variants::cyoa::cyoa;
use crate::variants::quiz::quiz;

fn main() {
    println!("Hello, welcome to this text-based adventure. In this game, you will go through a series of questions, which you will answer by entering text into the console. You have two options: Play this game as a quiz(1), or play this game as a choose your own adventure(2). Which do you choose? Press 1 go play the game as a quiz, or 2 to play it as a choose your own adventure.");
    loop {
        let option = readline();
        if option == "1" {
            quiz();
        } else if option == "2" {
            cyoa();
        } else {
            println!("Please enter a valid option(1 or 2).");
            continue;
        }
        break;
    }
}
