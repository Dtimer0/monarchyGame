mod variants {
    pub mod cyoa;
    pub mod quiz;
}
pub mod toolbox;
pub mod types;

use variants::quiz as quiz;
use variants::cyoa as cyoa;

use crate::toolbox::readline;
use crate::variants::quiz::quiz;
use crate::variants::cyoa::cyoa;
fn main() { 
    println!("Hello, welcome to this text-based adventure. In this game, you will go through a series of questions, which you will answer by entering text into the console. You have two options: Play this game as a quiz(1), or play this game as a choose your own adventure(2). Which do you choose? Press 1 go play the game as a quiz, or 2 to play it as a choose your own adventure.");
    if readline() == "1" {
        quiz();
    } else {
        println!("Under Construction");
    }
}
