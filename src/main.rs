#![allow(non_snake_case)]

mod variants {
    pub mod cyoa;
    pub mod quiz;
}
pub mod toolbox;
pub mod types;


use crate::toolbox::readline;
use crate::variants::cyoa::cyoa;
use crate::variants::quiz::quiz;

fn main() {
    println!("Hello, welcome to this text-based adventure. This game will try to give a clearer picture to the question: How has the attitude towards the British monarchy of Canadians and the Canadian government changed since 1914? In this game, you will go through a series of questions, which you will answer by entering text into the console. You have two options: Play this game as a quiz, or play this game as a choose your own adventure.(Questions will be the same regardless of gamemode.). If you have not already, please read the document at https://docs.google.com/document/d/1qqTJPUFDhEs_ZjiJtj5Lmf5Ay7bchljRF-a4GYj7Ki0/edit.");
    println!("Enter 1 to play the quiz, or 2 to play the choose your own adventure:");
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
    println!("Sources:");
}
