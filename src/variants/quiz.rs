use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use crate::toolbox::*;
use crate::types::*;

pub fn quiz() {
    println!("Welcome to the quiz variant of this game! Here, you will be asked a series of questions from of the government of Canada. You will be given four options, and you must choose which one is correct. You have 2 playable options, play this game as a test, where your score will be tallied at the end, and you will be given a grade, or, play it as a learning walkthrough, where after each question, you will be given the correct answer, and an explanation of why it is correct. In the test, entering any text that does not point to an answer into the console will be considered as leaving the question blank, and will be, marked as incorrect. In the walkthrough, you will be able to enter any text into the console, and it will be considered as entering the correct answer.");
    println!("Enter 1 to play the test, or 2 to play the walkthrough:");
    loop {
        let option = readline();
        if option == "1" {
            quiz_test();
        } else if option == "2" {
            println!("After answering each question, there will be 5 seconds for you to read the correct answer and explanation, before the next question comes up. You will be able to read the explanation again by scrolling up.");
            quiz_walkthrough();
        } else {
            println!("Please enter a valid option(1 or 2).");
            continue;
        }
        break;
    }
}
fn quiz_test() {
    let quiz: [QuizBlock; 16] = build_quiz();
    let quiz_len = quiz.len();
    let mut score: i8 = 0;
    let start = Instant::now();
    for Question in quiz {
        let letter_to_num: HashMap<String, i8> = HashMap::from([("A".to_owned(), 0), ("B".to_owned(), 1), ("C".to_owned(), 2), ("D".to_owned(), 3)]);
        println!("");
        println!("Date: {}", Question.date());
        println!("Question: {} Did the Canadian government:", Question.question);
        println!("");
        println!("A. {}", Question.answers[0].answer);
        println!("B. {}", Question.answers[1].answer);
        println!("C. {}", Question.answers[2].answer);
        println!("D. {}", Question.answers[3].answer);
        let answer = readline();
        if let Ok(answer) = answer.parse::<i8>() {
            let answer = answer - 1;
            if Question.answers[answer as usize].is_correct {
                score += 1;
            } 
        } 
        else if let Some(answer_num) = letter_to_num.get(&answer) {
            let answer_num = *answer_num as usize;
            if Question.answers[answer_num].is_correct {
                score += 1;
            } 
        } else {
            println!("You did not enter a valid answer, so your answer was marked as incorrect.");
        }
    }
    println!("");
    println!("Your score is: {}/{}, or a {}%", score, quiz_len, (score as f32 / quiz_len as f32) * 100.0);
    println!("Your time was: {} seconds", start.elapsed().as_secs());
}
fn quiz_walkthrough() {
    let quiz: [QuizBlock; 16] = build_quiz();
    for Question in quiz {
        let letter_to_num: HashMap<String, i8> = HashMap::from([("A".to_owned(), 0), ("B".to_owned(), 1), ("C".to_owned(), 2), ("D".to_owned(), 3)]);
        println!("");
        println!("Date: {}", Question.date());
        println!("Question: {} Did the Canadian government:", Question.question);
        println!("");
        println!("A. {}", Question.answers[0].answer);
        println!("B. {}", Question.answers[1].answer);
        println!("C. {}", Question.answers[2].answer);
        println!("D. {}", Question.answers[3].answer);
        let mut correct_answer = "";
        for i in 0..Question.answers.len() {
            if Question.answers[i].is_correct {
                correct_answer = Question.answers[i].answer;
            }
        }
        let answer = readline();
        if let Ok(answer) = answer.parse::<i8>() {
            let answer = answer - 1;
            if Question.answers[answer as usize].is_correct {
                println!("Correct!");
            } else {
                println!("Incorrect!");
            } 
        } 
        else if let Some(answer_num) = letter_to_num.get(&answer) {
            let answer_num = *answer_num as usize;
            if Question.answers[answer_num].is_correct {
                println!("Correct!");
            } else {
                println!("Incorrect!");
            } 
        } else {
            println!("You did not enter a valid answer, so your answer was marked as incorrect.");
        }
        println!("");
        println!("The correct answer was: {}", correct_answer);
        println!("");
        println!("{}", Question.answer_explanation);
        sleep(Duration::from_secs(5));

    }
}