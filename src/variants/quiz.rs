use crate::toolbox::readline;
use crate::types::*;

pub fn quiz() {
    println!("Welcome to the quiz variant of this game! Here, you will be asked a series of questions from the perspective 
    of the current prime minister of Canada at the time of the question. You will be given four options, and you must choose which one is correct. You have
    2 playable options, play this game as a test, where your score will be tallied at the end, and you will be given a grade, or,
    play it as a learning walkthrough, where after each question, you will be given the correct answer, and an explanation of why it is correct.
    In the test, entering any text that does not point to an answer into the console will be considered as leaving the question blank, and will be,
    marked as incorrect. In the walkthrough, you will be able to enter any text into the console, and it will be considered as entering the correct answer.
    Enter 1 to play the test, or 2 to play the walkthrough.");
    loop {
        let option = readline();
        if option == "1" {
            quiz_test();
        } else if option == "2" {
            quiz_walkthrough();
        } else {
            println!("Please enter a valid option(1 or 2).");
            continue;
        }
        break;
    }
}
fn quiz_test() {
    let quiz: [QuizBlock; 17] = [];
    let mut score: i8 = 0;
}
fn quiz_walkthrough() {

}