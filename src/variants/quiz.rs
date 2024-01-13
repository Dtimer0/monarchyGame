use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use crate::toolbox::*;
use crate::types::*;

pub fn quiz() {
    println!("Welcome to the quiz variant of this game! Here, you will be asked a series of questions. You will be given 4 options, and you will select which one you think is correct. Press any key to continue: ");
    let _ = readline();
    println!("After answering each question, there will be 5 seconds for you to read the correct answer and explanation, before the next question comes up. You will be able to read the explanation again by scrolling up.");
    std::thread::sleep(Duration::from_secs(5));
    quiz_walkthrough();
        
}
// fn quiz_test() {
//     let quiz: [QuizBlock; 16] = build_quiz();
//     let quiz_len = quiz.len();
//     let mut score: i8 = 0;
//     let start = Instant::now();
//     for Question in quiz {
//         let letter_to_num: HashMap<String, i8> = HashMap::from([("A".to_owned(), 0), ("B".to_owned(), 1), ("C".to_owned(), 2), ("D".to_owned(), 3)]);
//         println!("");
//         println!("Date: {}", Question.date());
//         println!("Question: {} Did the Canadian government:", Question.question);
//         println!("");
//         println!("A. {}", Question.answers[0].answer);
//         println!("B. {}", Question.answers[1].answer);
//         println!("C. {}", Question.answers[2].answer);
//         println!("D. {}", Question.answers[3].answer);
//         let answer = readline();
//         if let Ok(answer) = answer.parse::<i8>() {
//             let answer = answer - 1;
//             if Question.answers[answer as usize].is_correct {
//                 score += 1;
//             } 
//         } 
//         else if let Some(answer_num) = letter_to_num.get(&answer.to_uppercase()) {
//             let answer_num = *answer_num as usize;
//             if Question.answers[answer_num].is_correct {
//                 score += 1;
//             } 
//         } else {
//             println!("You did not enter a valid answer, so your answer was marked as incorrect.");
//         }
//     }
//     println!("");
//     println!("Your score is: {}/{}, or a {}%", score, quiz_len, (score as f32 / quiz_len as f32) * 100.0);
//     println!("Your time was: {} seconds", start.elapsed().as_millis() as f32 / 1000.0);
// }
fn quiz_walkthrough() {
    let quiz: [QuizBlock; 16] = build_quiz();
    let mut score: i8 = 0;
    let mut quiz_len = quiz.len();
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
        else if let Some(answer_num) = letter_to_num.get(&answer.to_uppercase()) {
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
        sleep(Duration::from_secs(3));

    }
    println!("You have finished the quiz!");
    println!("Your score is: {}/{}, or a {}%", score, quiz_len, (score as f32 / quiz_len as f32) * 100.0);
    println!("As you can see, the Canadian government relationship with Britain has changed since 1914. Canada was a early colony of Britain and so they had a very close relationship. However, Canada became more independent and the Canadian government began to distance itself more from the British Monarchy. This is shown by Canada refusing to participate in British warfare(Such as the Suez Crisis), and Canada signing treaties independently(Such as the Treaty of Versailles) more often. Today, the average Canadian citizen is also less loyal to the British government. This was recently reflected in a poll which showed 54% of Canadians believe Canada should end all ties to the British monarchy(Ipsos, 2023). Overall, Canada has been becoming more and more legally distant from the British Monarchy; signing treaties independently, allowing internal justices systems, the patriation of our constituion, and more. However, the Canadian government is still very loyal to Britain, and despite our legal separation, we still have a very close relationship.");
}

pub fn build_quiz() -> [QuizBlock; 16] {
    return [
    QuizBlock {
        date: Date {
            year: 1914,
            month: "July",
            day: 28,
        },
        question:"War is breaking out in Europe. The Canadian Prime Minister meets with his Cabinet to decide what to do.",
        answers: [
            QuizAnswer {
                answer: "Wait for instructions from Britain",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Send soldiers to act as peacekeepers",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send soldiers to aid Serbia",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send soldiers to aid Austria-Hungary",
                is_correct: false,
            },
        ],
        answer_explanation: "After the assassination of Archduke Franz Ferdinand, the heir to the throne of Austria-Hungary, Austria-Hungary declared war on Serbia. As Canada was still a colony of the British Empire, the Prime Minister at the time, Robert Borden, decided to wait for instructions from Britain. This is an early example of Canada's dependence on Britain for direction, and it shows us how closely Canada and Britain were intertwined",
    },
    QuizBlock {
        date: Date {
            year: 1914,
            month: "August",
            day: 3,
        },
        question: "Britain issues an ultimatum to Germany asking that they withdraw their forces from Belgium, or they will be at war. Germany does not concede, and Britain is now at war. Britain asks for an initial 25,000 soldiers to help fight from Canada.",
        answers: [
            QuizAnswer {
                answer: "Send the 25,000 soldiers Britain requested",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send 25,000 soldiers, and enact the War Measures Act 21 days later",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Refuse to enter into the war",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send 25,000 soldiers, and start conscriptions",
                is_correct: false,
            },
        ],
        answer_explanation: "In response to Britain's request, Canada sent 25,000 soldiers. Canada also enacted the War Measures Act, which gave the government much more power during wars, such as the ability to censor the press and the power to declare the Military Service Act (which allows conscription, although the government did not conscript at that time). Canada sent 25,000 soldiers to fight in a war that was not putting them in immediate danger, to support Britain. In addition, they declared strong laws which had a big impact on Canadians, for the sake of the war",
    },
    QuizBlock {
        date: Date {
            year: 1917,
            month: "August",
            day: 29,
        },
        question: "300,000 Canadian soldiers had already volunteered to fight overseas, a massive amount for the population of only 8 million. However, Britain needs more soldiers.",
        answers: [
            QuizAnswer {
                answer: "Tell Britain that we cannot support the strain on our population",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Start a military recruitment campaign to try to get another 10,000 soldiers to fight, but refuse any more after",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Declare the Military Service Act, forcing conscription to all able male citizens between 20 and 45",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Declare the Military Service Act, forcing conscription to all able citizens between 20 and 45",
                is_correct: false,
            },
        ],
        answer_explanation: "Canada enacts the Military Service Act due to pressure from Britain. This forces citizens into the Canadian Armed Forces. This was a very controversial decision, as we'll learn about later. Conscription to support another country's war is a dramatic example of Canada's loyalty to Britain",
    },
    QuizBlock {
        date: Date {
            year: 1917,
            month: "December",
            day: 7,
        },
        question: "There is outcry about forced conscription. Riots break out in Quebec, as much of French Canada has opposed the war, and they refuse to be forced to fight in it.",
        answers: [
            QuizAnswer {
                answer: "Continue the program to conscript more soldiers",
                is_correct: false,
            },
            QuizAnswer {
                answer: "End the Military Service Act",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Call an election fought on the matter of Conscription",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Make a public statement about the lack of sense of Civic Duty in French Canada, and call French Canadians traitors to Canada",
                is_correct: false,
            },
        ],
        answer_explanation: "At the time, the Prime Minister was Robert Borden. In response to the French outcry, he called an election known as the \"Khaki Election\" (Khaki is the colour of the Canadian Armed Forces uniform), which was primarily fought on the issue of conscription. The Prime Minister's decision to call an election shows us that the Canadian government was willing to consider a result that would not benefit Britain, showing some distancing between Canada and Britain. However, Borden won the election, and continued the conscription program. This shows that the Canadian public supported the war, and still supported Britain",
    },
    QuizBlock {
        date: Date {
            year: 1919,
            month: "June",
            day: 28,
        },
            question: "The war ended nearly a year ago. Canada is called to join deliberations in Paris on what the punishment for losing the war for Germany, Austria-Hungry, and Turkey will be. It is decided that Germany will have to pay reparations, Germany and the Ottoman Empire will hand overseas possessions to France and Britain, and the countries will draw new national boundaries to better represent ethnic borders. Canada is at the Paris Peace Conference, and each leader is signing the Treaty of Versailles. Initially, Canada was brought in for deliberations, but would not sign the Treaty, as it was a British Colony. However, Canada contributed a lot to the war, and deserves recognition.",
        answers: [
            QuizAnswer {
                answer: "Demand the right to sign the Treaty of Versailles",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Ask Britain politely to sign the Treaty Independently",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Wait until everyone else has signed and left. Then have someone track the carrier of the Treaty and secretly sign it when they aren't looking",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Do nothing",
                is_correct: false,
            },
        ],
        answer_explanation: "Canada had contributed a lot to the war by sending many soldiers to Britain and providing more than 3 billion dollars in US money, and tons of equipment and raw materials to Britain. Initially, Canada was just brought in for deliberations, and would not be allowed to sign the treaty, as it was a British Colony. However, Canada felt that they deserved to sign. This was a major step in Canadian distancing from the British Monarchy, as it was one of the first times Canada signed an international treaty independently, and Canada had never signed one of this magnitude",
    },
    QuizBlock {
        date: Date {
            year: 1931,
            month: "December",
            day: 11,
        },
        question: "The Statute of Westminster increased the sovereignty of British Colonies by removing almost all power of the British Government to Legislate in British Dominions.",
        answers: [
            QuizAnswer {
                answer: "Embrace this change fully",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Embrace the change, but assure Britain that Canada will continue to be a large supporter of British action",
                is_correct: false,
            },
            QuizAnswer {
                answer: "State that this change is not enough, and the British government needs to give us more sovereignty",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Publicly state that this change gives Canada full sovereignty, and that we are fully separated from Britain",
                is_correct: false,
            },
            ],
            answer_explanation: "The Statue of Westminster was introduced because it was felt by Britain that British Colonies should be considered to be their own governing bodies and should be allowed a justice system not involved with the Crown. Canada embraced this change fully, and it was a major step towards Canada's independence, as it represented Britain recognizing Canada as a state that should have a purely internal justice system",
            },
        QuizBlock {
            date: Date {
                year: 1939,
                month: "September",
                day: 10,
            },
            question: "WW2 is breaking out in Europe. Britain asks for Canada's help.",
            answers: [
                QuizAnswer {
                    answer: "Declare war on Germany and send Infantry Battalions to assist the British",
                    is_correct: false,
                },
                QuizAnswer {
                    answer: "Declare war on Britain",
                    is_correct: false,
                },
                QuizAnswer {
                    answer: "Not declare war",
                    is_correct: false,
                },
                QuizAnswer {
                    answer: "Declare war on Germany but discourage Infantry Battalions, and instead provide resources to assist Britain",
                    is_correct: true,
                },
            ],
            answer_explanation: "The Canadian government predicted a high amount of casualties, and discouraged citizens from volunteering in Infantry Battalions. Instead, they provided resources to Britain, such as food, equipment, and raw materials. This would help the cause, and make sure Canada would not suffer from a large population loss due to the war. This is an example of Canada's changing attitudes, as during the first world war, Canada was happy to send many soldiers to fight for Britain",
        },
    QuizBlock {
        date: Date {
            year: 1940,
            month: "January",
            day: 1,
        },
        question: "France has fallen, and Canada is now Britain's most important ally.",
        answers: [
            QuizAnswer {
                answer: "Send more soldiers",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Provide naval defense against German submarines",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send more equipment",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Do all of the above",
                is_correct: true,
            },
        ],
        answer_explanation: "France had just fallen, and Britain was in desperate need of help. Britain was struggling from German submarine attacks, and so asked Canada to provide defense from them. Additionally, Britain needed more soldiers, equipment, and supplies, and Canada was happy to oblige. Canada supporting Britain, when Britain was in a losing position, shows us that Canada was still very committed to Britain",
    },
    QuizBlock {
        date: Date {
            year: 1940,
            month: "January",
            day: 1,
        },
        question: "Britain needs more money to continue funding the war.",
        answers: [
            QuizAnswer {
                answer: "Grant the British ~3 billion dollars",
                is_correct: true,
            },
            QuizAnswer {
                answer: "Grant the British over 100 billion dollars",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send no money",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Tell the British we can't afford sending money, but we can send soldiers",
                is_correct: false,
            },
        ],
        answer_explanation: "Known as the \"Billion Dollar Gift\", Canada sent over 3 billion dollars to Britain to help fund the war. This was a massive amount of money, and was a huge help to Britain. This shows Canada's steady loyalty to Britain in volatile times, despite Canada's growing legal separation",
    },
QuizBlock {
    date: Date {
        year: 1956,
        month: "October",
        day: 29,
    },
    question: "Britain, France, and Israel decide to invade Egypt and the Gaza Strip. Britain asks for our help.",
    answers: [
        QuizAnswer {
            answer: "Send soldiers to fight alongside the British",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Send funds and resources to the British, but refuse sending soldiers",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Refuse to assist in the takeover",
            is_correct: true,
        },
        QuizAnswer {
            answer: "Support Egypt by sending resources and soldiers",
            is_correct: false,
        },
    ],
    answer_explanation: "Canada flatly refused to assist in the invasion, one of only a few times Canada has refused to assist Britain in a war. This shows us that Canada is not afraid make a decision that does not help Britain, and that Canada is becoming more separated from Britain",
},

QuizBlock {
    date: Date {
        year: 1964,
        month: "June",
        day: 15,
    },
    question: "Canadians are unhappy with our current flag, still bearing the Union Jack.",
    answers: [
        QuizAnswer {
            answer: "Ignore them",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Propose a new design, that still has the Union Jack, just smaller",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Ask Britain to make a design for us",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Form a committee to design a new flag",
            is_correct: true,
        },
    ],
    answer_explanation: "The committee designed the current Canadian flag, which was adopted in 1965. The flag was designed by George Stanley, and was based on the flag of the Royal Military College of Canada. This marked a major step towards Canada's independence, as having an independent flag, to the world, is a sign of independence from Britain",
},
QuizBlock {
    date: Date {
        year: 1980,
        month: "May",
        day: 14,
    },
    question: "The Premier of Quebec; René Lévesque, is calling a referendum to decide whether Quebec should advocate to secede from Canada.",
    answers: [
        QuizAnswer {
            answer: "Make a statement supporting Quebec's attempt at secession",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make no statement on the matter",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make a statement saying Canada does support the referendum, that it is unconstitutional, disrespectful, and stupid",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make a statement that the government does not support it, because Quebec is a very important part of Canadian culture, and that it would be tragic loss to Canadians",
            is_correct: true,
        },
    ],
    answer_explanation: "The referendum ended in a 40/60 split, with 60% of Quebecers voting to stay with Canada. The fact that this was called at all shows us how the Quebecois feel about Canada, and how they feel about their relationship with Britain. Unsurprisingly, there has rarely been much support for Britain in Quebec, due to a large amount of French Canadians. This shows us how a large part of Canada is not loyal to the British Monarchy",
},
QuizBlock {
    date: Date {
        year: 1982,
        month: "April",
        day: 17,
    },
    question: "Canada's governing party feels that it could be losing favour with Canadians, and feels it should do something to gain favour.",
    answers: [
        QuizAnswer {
            answer: "Attempt to patriate the Canadian Constitution and create a Charter of Rights and Freedoms(C)",
            is_correct: true,
        },
        QuizAnswer {
            answer: "Reduce taxes",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Spend more money on campaigning",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Print more money to reduce inflation",
            is_correct: false,
        },
    ],
    answer_explanation: "Patriating the Canadian Constitution and creating a Charter of Rights and Freedoms was a incredibly significant step towards full independence from Britain. This allowed Canada to modify our constitution without British approval. This was, another major step towards Canadian legal independence from Britain, as it essentially let us create our own specific laws",
},
QuizBlock {
    date: Date {
        year: 1995,
        month: "October",
        day: 25,
    },
    question: "Another Referendum is being held by the Parti Quebecois.",
    answers: [
        QuizAnswer {
            answer: "Support the cause",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Talk about how important Quebec is to Canada, as well as \"suggesting\" there would be great economic consequences for Quebec",
            is_correct: true,
        },
        QuizAnswer {
            answer: "Talk about how Canada would invade Quebec to take it back if it succeeds in succession",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make no statement",
            is_correct: false,
        },
    ],
    answer_explanation: "In a famous speech by Jean Chrietien, he talked about how important Quebec was to Canada, and how it would be a great loss to Canada if Quebec were to secede. He made this speech because Quebec is an important part of Canada, and the Canadian economy would suffer greatly from this secession. This time, the vote was incredibly close, at around 50.6% to 49.4%. This shows us the rising support for Quebec separatism from the last vote, which ended 60%/40%. This shows us Quebec's interest in removing themselves from Canada, and by extension, Britain",
},
QuizBlock {
    date: Date {
        year: 2020,
        month: "January",
        day: 14,
    },
    question: "Prince Harry and his wife Megan have decided to leave Britain and move to Canada.",
    answers: [
        QuizAnswer {
            answer: "Privately send a message to Harry and Megan supporting their move",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Discuss that he is obviously supportive of them moving to Canada",
            is_correct: true,
        },
        QuizAnswer {
            answer: "Attempt to deny them citizenship on the grounds of suspicious criminal history",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Ignore questions about it and remain neutral",
            is_correct: false,
        },
    ],
    answer_explanation: "Expressing support for Prince Harry and Megan's decision to move to Canada was a great diplomatic approach that respects their personal choices, while also making sure the Queen still understand that the Prime Minister is loyal to her. The Queen allowed Harry and Megan to live between Canada and Britain, and our Prime Minister expressing support for the decision shows us Canada's respect and loyalty for Britain",
},
QuizBlock {
    date: Date {
        year: 2022,
        month: "September",
        day: 9,
    },
    question: "The Queen has just died.",
    answers: [
        QuizAnswer {
            answer: "Make a heartfelt speech about her passing, and how much Canada will miss her",
            is_correct: true,
        },
        QuizAnswer {
            answer: "Say she was really not all that great",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Announce that Canada will be fully leaving Britain, severing all ties in government",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make a speech about how important she was to Canada, but that we are very excited to have King Charles as our new Monarch",
            is_correct: false,
        },
    ],
    answer_explanation: "After the Queen had passed, the Prime Minister made a heartfelt speech, where he was moved to tears. This showed our appreciation for the British Monarchy, and how much Canada values our continuing relationship with Britain",
},
    ];
}