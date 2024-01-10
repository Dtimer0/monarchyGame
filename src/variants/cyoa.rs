use std::collections::HashMap;
use crate::types::*;
use crate::toolbox::*;
// As the game progresses, bonuses become larger, and penalties become smaller, as you are expected to lose more than you win.
pub fn cyoa() {
    println!("Welcome to this choose your own adventure game. This game has a public opinion rating, British opinion rating, and historical accuracy rating. If the public opinion rating, or British opinion rating ever fall below 10%, you will immediately lose. Historical accuracy rating will be showed to you at the end of the game. Otherwise, certain decisions can lead to you losing the game. There are 2 ways to win the game, see if you can find both!");
    let web = build_cyoa();
    let mut current = "start";
    let mut public_opinion = 0.0;
    let mut british_opinion = 0.0;
    let mut qs_answered = 0.0;
    let mut historical_accuracy = 0.0;
    let mut current_block: CyoaBlock = CyoaBlock {
        date: Date {day: 28, month: "July", year: 1914},
        question: "War is breaking out. You meet with your cabinet to decide what to do about it.",
        answers: [
            CyoaAnswer {
                is_historically_accurate: true,
                game_over: false,
                pointer_alias: "ultimatum",
                answer: "Wait for instructions from the Monarchy",
                public_favor: 1.0,
                british_favor: 2.0,
            },
            CyoaAnswer {
                is_historically_accurate: false,
                game_over: false,
                pointer_alias: "peacekeeping1",
                answer: "Send soldiers to act as peacekeepers",
                public_favor: 1.5,
                british_favor: 0.8,
            }, 
            CyoaAnswer {
                is_historically_accurate: false,
                game_over: false,
                pointer_alias: "300000",
                answer: "Send soldiers to aid Serbia",
                public_favor: 1.0,
                british_favor: 1.2,
            },
            CyoaAnswer {
                is_historically_accurate: false,
                game_over: false,
                pointer_alias: "majorlossgermany",
                answer: "Send soldiers to aid Austria-Hungary",
                public_favor: 0.3,
                british_favor: 0.1,
            },
            
        ],
        alias: "start",
    };
    'outer: loop {
        'forloopinweb: for i in &web {
            if i.alias == current {
                current_block = i.clone();
                break 'forloopinweb;
            }
        }
        let letter_to_num: HashMap<String, i8> = HashMap::from([("A".to_owned(), 0), ("B".to_owned(), 1), ("C".to_owned(), 2), ("D".to_owned(), 3)]);
        println!("");
        println!("Date: {}", current_block.date());
        println!("{} Do you:", current_block.question);
        println!("");
        println!("A. {}", current_block.answers[0].answer);
        println!("B. {}", current_block.answers[1].answer);
        println!("C. {}", current_block.answers[2].answer);
        println!("D. {}", current_block.answers[3].answer);
        let answer = readline();
        let answer_: CyoaAnswer;
        qs_answered += 1.0;
        if let Ok(answer) = answer.parse::<i8>() {
            let answer: usize = (answer - 1) as usize;
            answer_ = current_block.answers[answer].clone();

        } else if let Some(answer_num) = letter_to_num.get(&answer) {
            let answer_num = *answer_num as usize;
            answer_ = current_block.answers[answer_num].clone();
            
        } else {
            let answer_num = 3 as usize;
            answer_ = current_block.answers[answer_num].clone();
            println!("You did not enter a valid answer, so you were given the last answer.");
        }
        public_opinion += answer_.public_favor;
        british_opinion += answer_.british_favor;
        current = answer_.pointer_alias;
        println!("");
        println!("Current Public Opinion:");
        print!("[");
        for _ in 0..round((public_opinion / qs_answered) * 100.0, 0) as i32 {
            print!("█");
        }
        for _ in round((public_opinion / qs_answered) * 100.0, 0) as i32..100 {
            print!(" ");
        }
        print!("]");
        println!(" {}%", round((public_opinion / qs_answered) * 100.0, 0) as i32);
        println!("Current British Opinion:");
        print!("[");
        for _ in 0..round((british_opinion / qs_answered) * 100.0, 0) as i32 {
            print!("█");
        }
        for _ in round((british_opinion / qs_answered) * 100.0, 0) as i32..100 {
            print!(" ");
        }
        print!("]");
        println!(" {}%", round((british_opinion / qs_answered) * 100.0, 0) as i32);
        if answer_.is_historically_accurate {
            historical_accuracy += 1.0;
        }
        if round((public_opinion / qs_answered) * 100.0, 2) < 10.0 {
            current = "canadianoverthrow";
            break 'outer;
        }
        if round((british_opinion / qs_answered) * 100.0, 2) < 10.0 {
            current = "britishimpeach";
            break 'outer;
        }
        if answer_.game_over {
            break 'outer;
        }

    }
    for i in &build_over() {
        if i.alias == current {
            println!("Game Over!");
            println!("Your historical accuracy: {}%", round(historical_accuracy / qs_answered * 100.0, 2));
            println!("Reason: {}", i.reason);
            return;
        }
    }

}


pub fn build_cyoa() -> Vec<CyoaBlock> {
    Vec::from([
        CyoaBlock {
            date: Date {day: 28, month: "July", year: 1914},
            question: "War is breaking out. You meet with your cabinet to decide what to do about it.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "ultimatum",
                    answer: "Wait for instructions from the Monarchy",
                    public_favor: 0.85,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "peacekeeping1",
                    answer: "Send soldiers to act as peacekeepers",
                    public_favor: 0.9,
                    british_favor: 0.8,
                }, 
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Send soldiers to aid Serbia",
                    public_favor: 1.0,
                    british_favor: 1.2,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "majorlossgermany",
                    answer: "Send soldiers to aid Austria-Hungary",
                    public_favor: 0.3,
                    british_favor: 0.1,
                },
                
            ],
            alias: "start",
        }, 
        CyoaBlock {
            date: Date {day: 3, month: "August", year: 1914},
            question: "Britain issues an ultimatum to Germany asking that they withdraw their forces from Belgium, or they will be at war. August 4, 1914. Germany does not concede, and Britain, and Canada, are now at war. Britain asks for an initial 25 000 soldiers to help fight.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Send the 25 000 soldiers they requested, but refuse more",
                    public_favor: 0.9,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Send 25 000 soldiers, and create the war measures act, to be declared 21 days later.",
                    public_favor: 0.8,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Refuse to enter into the war",
                    public_favor: 0.95,
                    british_favor: 0.5,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "riotsinquebec",
                    answer: "Send 25 000 soldiers, and start conscriptions",
                    public_favor: 0.7,
                    british_favor: 1.0,
                },
            ],
            alias: "ultimatum",
        },
        CyoaBlock {
            date: Date {day: 29, month: "August", year: 1917},
            question: "300 000 soldiers had already volunteered to fight overseas, a massive amount for the population of only 8 million. However, Britain needs more soldiers.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Tell Britain that we cannot support the strain on our population",
                    public_favor: 0.8,
                    british_favor: 0.2,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Campaign to try to get another 10 000 soldiers to fight, but refuse any more after",
                    public_favor: 0.8,
                    british_favor: 0.6,
                },
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false, 
                    pointer_alias: "riotsinquebec",
                    answer: "Declare the Military Service Act, forcing conscription to all able male citizens between 20 and 45.",
                    public_favor: 0.5,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "riotsinquebec",
                    answer: "Declare the Military Service Act, forcing conscription to all able citizens between 20 and 45.",
                    public_favor: -0.3,
                    british_favor: 0.95,
                },

            ],
            alias: "300000",
        },
        CyoaBlock {
            date: Date {
                day: 7,
                month: "December",
                year: 1917,
            },
            question: "There is outcry about forced conscription. Riots break out in Quebec, as much of French Canada has opposed the war, and they refuse to be forced to fight in it.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Continue the program to conscript more soldiers",
                    public_favor: 0.2,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "End the Military Service Act",
                    public_favor: 0.8,
                    british_favor: 0.4,
                },
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Call an election fought on the matter of Conscription",
                    public_favor: 1.0,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Make a public statement about the lack of sense of Civic Duty in French Canada, and call French Canadians traitors to Canada, and continue to conscript.",
                    public_favor: 0.2,
                    british_favor: 0.35,
                },               
            ],
            alias: "riotsinquebec",
        },
        CyoaBlock {
            date: Date {day: 28, month: "June", year: 1919},
            question: "The war ended nearly a year ago. You are called to join deliberations in Paris on what the punishment for losing the war for Germany, Austria-Hungry, and Turkey. It is decided that Germany will have to pay reparations, Germany and the Ottoman Empire will hand overseas possession to France and Britain, and the drawing of new national boundaries to better represent ethnic borders. You are at the Paris Peace Conference, and each leader is signing the Treaty of Versailles. Initially, Canada was brought in for deliberations, but would not sign the Treaty, as it was a British Colony. However, Canada contributed a lot to the war, and deserves recognition.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Demand the right to sign the Treaty of Versailles.",
                    public_favor: 1.0,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Ask Britain politely to sign the Treaty Independently",
                    public_favor: 0.8,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Do nothing",
                    public_favor: 0.5,
                    british_favor: 0.6,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Wait until everyone else has signed and left, and then track the carrier of the Treaty and secretly sign it when they aren't looking.",
                    public_favor: 0.5,
                    british_favor: 0.5,
                },
            ],
            alias: "treaty",
        },
        CyoaBlock {
            date: Date {
                day: 14,
                month: "September",
                year: 1914
            },
            question: "The peacekeeping force has not been very successful. The war is becoming more and more deadly.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "peacekeeping2",
                    answer: "Send more peacekeepers",
                    public_favor: 1.0,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Withdraw all forces and refuse to participate in the war.",
                    public_favor: 0.7,
                    british_favor: 0.3,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Order your soldiers to fight for Britain",
                    public_favor: 0.7,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "majorlossgermany",
                    answer: "Order your soldiers to fight for Germany",
                    public_favor: 0.1,
                    british_favor: -0.5,
                }
            ],
            alias: "peacekeeping1",
        },
        CyoaBlock {
            date: Date {day: 31, month: "March", year: 1915},
            question: "The peacekeeping has been entirely unsuccessful. You have no choice, you must choose a side, or withdraw all soldiers.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "majorlossgermany",
                    answer: "Order your soldiers to fight for Germany",
                    public_favor: 0.1,
                    british_favor: -0.5,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Order your soldiers to fight for Britain",
                    public_favor: 0.7,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Withdraw all forces and refuse to participate in the war.",
                    public_favor: 0.7,
                    british_favor: 0.3,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Order your soldiers to fight for Britain and send another 50000 soldiers",
                    public_favor: 0.7,
                    british_favor: 0.9,
                },
            ],
            alias: "peacekeeping2",
        },
        CyoaBlock {
            date: Date {day: 30, month: "September", year: 1917},
            question: "You have lost a lot of soldiers, and it seems like you are about to lose the war.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "whiteflag",
                    answer: "Tell your soldiers to return to being peacekeepers.",
                    public_favor: 0.7,
                    british_favor: 0.5,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "whiteflag",
                    answer: "Tell Germany you can no longer support them and withdraw all soldiers",
                    public_favor: 0.7,
                    british_favor: 0.7, 
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: true,
                    pointer_alias: "invasion",
                    answer: "Order your soldiers to continue fighting for Germany",
                    public_favor: 0.1,
                    british_favor: -0.5,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false, 
                    pointer_alias: "treaty",
                    answer: "Order your soldiers to turn on the Germans",
                    public_favor: 0.7,
                    british_favor: 0.9,
                }
            ],
            alias: "majorlossgermany",
            
        },
        CyoaBlock {
            date: Date {day: 11, month: "November", year: 1918},
            question: "Germany waves the white flag, and the war is over. Britain is extremely angry you betrayed them. Now, you need to take action.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Apologize deeply for your mistake, and pledge forever allegiance in the future.",
                    public_favor: 0.75,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: true,
                    pointer_alias: "invasion",
                    answer: "Tell Britain you will never apologize, and that you will never be their ally again.",
                    public_favor: 0.1,
                    british_favor: -1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Pay reparations and apologize",
                    public_favor: 0.8,
                    british_favor: 0.9,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Pay reparations, but refuse to apologize",
                    public_favor: 0.85,
                    british_favor: 0.6,
                },
            ],
            alias: "whiteflag",
        },
        CyoaBlock {
            date: Date {day: 11, month: "December", year: 1931},
            question: "The Statute of Westminster, this increases the sovereignty of British Colonies by removing almost all power of the British Government to Legislate in Dominions.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ww2",
                    answer: "State that this change is not enough, and the British government needs to give us more sovereignty.",
                    public_favor: 1.0,
                    british_favor: 0.2,
                },
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "ww2",
                    answer: "Embrace this change fully",
                    public_favor: 1.0,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ww2",
                    answer: "Embrace the change, but assure Britain that we will continue to be a large supporter of British action",
                    public_favor: 1.0,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "lying",
                    answer: "Publicly state that this change gives Canada full sovereignty, and that we are fully separated from the Monarchy.",
                    public_favor: 9.0,
                    british_favor: 0.2,
                },
            ],
            alias: "westminster",
        },
        CyoaBlock {
            date: Date {day: 10, month: "September", year: 1939},
            question: "WW2 is breaking out. There is a predicted high death rate in Infantry.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ohnofrance",
                    answer: "Declare war on Germany and send Infantry Battalions to assist the British.",
                    public_favor: -0.6,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "ohnofrance",
                    answer: "Declare war on Germany but discourage Infantry Battalions, and instead provide resources to assist Britain.",
                    public_favor: 0.7,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: true,
                    pointer_alias: "poordecision",
                    answer: "Declare war on Britain",
                    public_favor: 0.1,
                    british_favor: -1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "britainfalling",
                    answer: "Do not declare war",
                    public_favor: 0.6,
                    british_favor: 0.2,
                },
            ],
            alias: "ww2",
        },
        CyoaBlock {
            date: Date {day: 1, month: "January", year: 1940},
            question: "France has fallen, and Canada is now Britains most important ally.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "gift",
                    answer: "Send more soldiers",
                    public_favor: 0.8,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "gift",
                    answer: "Provide naval defence against Germany submarines.",
                    public_favor: 0.8,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "gift",
                    answer: "Send more equipment.",
                    public_favor: 0.9,
                    british_favor: 0.75,
                },
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "gift",
                    answer: "Do all of the above.",
                    public_favor: 0.7,
                    british_favor: 1.0,
                },
                
            ],
            alias: "ohnofrance",
        },
        CyoaBlock {
            date: Date {day: 1, month: "January", year: 1940},
            question: "Britain needs more money to continue funding the war.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: true,
                    game_over: false,
                    pointer_alias: "suezinvasion",
                    answer: "Grant the British ~3 billion dollars",
                    public_favor: 0.6,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "suezinvasion",
                    answer: "Grant the British ~100 billion dollars",
                    public_favor: -0.8,
                    british_favor: 1.5,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: true,
                    pointer_alias: "axisw2",
                    answer: "Refuse to give Britain any money",
                    public_favor: 0.7,
                    british_favor: -1.0,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "suezinvasion",
                    answer: "Tell the British we can't afford sending money, but we can send soldiers. ",
                    public_favor: 0.8,
                    british_favor: 0.7,
                }
                
            ],
            alias: "gift",
        },
        CyoaBlock {
            date: Date {day: 13, month: "December", year: 1931},
            question: "Britain publicly calls you out for lying about Canada having full sovereignty.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ww2",
                    answer: "Apologize for lying.",
                    public_favor: 0.8,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ww2",
                    answer: "Apologize and offer raw resources as a symbol of our apology.",
                    public_favor: 0.7,
                    british_favor: 0.9,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: true,
                    pointer_alias: "poordecision",
                    answer: "Announce that this Britain is trying to take back their decision, and that you will invade Britain as revenge for the insult to Canada's pride.",
                    public_favor: 0.2,
                    british_favor: 0.0,
                },
                CyoaAnswer { //TODO: Remember this is a loop
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "lying",
                    answer: "Say that Canada really is fully independent",
                    public_favor: 0.7,
                    british_favor: 0.2,
                }
                
            ],
            alias: "lying",
        },
        CyoaBlock {
            date: Date {day: 4, month: "April", year: 1939},
            question: "Canada watches as Britain becomes closer and closer to falling, while Canada still sits idle.",
            answers: [
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ohnofrance",
                    answer: "Send soldiers to assist Britain.",
                    public_favor: 0.7,
                    british_favor: 1.4,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ohnofrance",
                    answer: "Send equipment and resources",
                    public_favor: 0.8,
                    british_favor: 0.7,
                }, 
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: true,
                    pointer_alias: "axisw",
                    answer: "Remain neutral",
                    public_favor: 0.5,
                    british_favor: 0.1,
                },
                CyoaAnswer {
                    is_historically_accurate: false,
                    game_over: false,
                    pointer_alias: "ohnofrance",
                    answer: "Secretly send resources to the Axis",
                    public_favor: 0.7,
                    british_favor: 0.9, // This is because Britain is not aware of this, and 9 should be around the right amount to keep neutral
                },
            ],
            alias: "britainfalling",
        }
    ])
}

pub fn build_over() -> Vec<GameOver> {
    Vec::from([
        GameOver {
            alias: "invasion",
            reason: "Britain decides they will invade Canada and bring it entirely back under British control. The government is destroyed.",
        },
        GameOver {
            alias: "britishimpeach",
            reason: "The British have lost all faith in you, and you have been taken over.",
        },
        GameOver {
            alias: "canadianoverthrow",
            reason: "The public has lost all faith in you, and you overthrown.",
        },
        GameOver {
            alias: "poordecision",
            reason: "That was a poor decision. Britain's army vastly overpowers yours and the government is overthrown.",
        },
        GameOver {
            alias: "axisw",
            reason: "Britain has fallen and despite Canada remaining neutral, the Axis is unstoppable, and without Britains protection, Canada is easily overpowered by the Axis",
        },
        GameOver {
            alias: "axisw2",
            reason: "Britain has fallen, due to a lack of resources to continue the war, and Canada is easily overpowered by the Axis",
        }
    ])

}