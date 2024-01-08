use std::collections::HashMap;

use crate::toolbox::*;
use crate::types::*;

pub fn quiz() {
    println!("Welcome to the quiz variant of this game! Here, you will be asked a series of questions from the perspective of the current prime minister of Canada at the time of the question. You will be given four options, and you must choose which one is correct. You have 2 playable options, play this game as a test, where your score will be tallied at the end, and you will be given a grade, or, play it as a learning walkthrough, where after each question, you will be given the correct answer, and an explanation of why it is correct. In the test, entering any text that does not point to an answer into the console will be considered as leaving the question blank, and will be, marked as incorrect. In the walkthrough, you will be able to enter any text into the console, and it will be considered as entering the correct answer.");
    println!("Enter 1 to play the test, or 2 to play the walkthrough:");
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
    let quiz: [QuizBlock; 1] = [
    QuizBlock {
        date: Date {
            year: 1914,
            month: "July",
            day: 28,
        },
        question:"War is breaking out. You meet with your cabinet to decide what to do about it. Do you:",
        answers: [
            QuizAnswer {
                answer: "Wait for instructions from the Monarchy.",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Send soldiers to act as peacekeepers ",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send soldiers to aid Serbia ",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send soldiers to aid Austria-Hungary",
                is_correct: false,
            },
        ],
        answer_explanation: "After the assassination of Archduke Franz Ferdinand, the heir to the throne of Austria-Hungary, 
        Austria-Hungary declared war on Serbia. As Canada was still a colony of the British Empire, the Prime Minister at the time, 
        Robert Borden, decided to wait for instructions from the Monarchy.",
    }
    ];

    let mut score: i8 = 0;
    for Question in quiz {
        let letter_to_num: HashMap<String, i8> = HashMap::from([("A".to_owned(), 0), ("B".to_owned(), 1), ("C".to_owned(), 2), ("D".to_owned(), 3)]);
        println!("Date: {}", Question.date());
        println!("Question: {}", Question.question);
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
    println!("Your score is: {}/17, or a {}%", score, (score as f32 / 17.0) * 100.0);
}
fn quiz_walkthrough() {

}