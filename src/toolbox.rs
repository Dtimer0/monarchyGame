use crate::types::*;
#[allow(unreachable_code)]

pub fn readline() -> String {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    return line;
}
impl QuizBlock {
    pub fn date(&self) -> String {
        return format!("{} {}, {}", self.date.month, self.date.day, self.date.year);
    }
}


pub fn build_quiz() -> [QuizBlock; 16] {
    return [
    QuizBlock {
        date: Date {
            year: 1914,
            month: "July",
            day: 28,
        },
        question:"War is breaking out. The Prime Minister meets with his Cabinet to decide what to do.",
        answers: [
            QuizAnswer {
                answer: "Wait for instructions from the Monarchy",
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
        answer_explanation: "After the assassination of Archduke Franz Ferdinand, the heir to the throne of Austria-Hungary, 
        Austria-Hungary declared war on Serbia. As Canada was still a colony of the British Empire, the Prime Minister at the time, 
        Robert Borden, decided to wait for instructions from the Monarchy",
    },
    QuizBlock {
        date: Date {
            year: 1914,
            month: "August",
            day: 3,
        },
        question: "Britain issues an ultimatum to Germany asking that they withdraw their forces from Belgium, or they will be at war. Germany does not concede, and Britain, and Canada, are now at war. Britain asks for an initial 25,000 soldiers to help fight.",
        answers: [
            QuizAnswer {
                answer: "Send the 25,000 soldiers they requested",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Send 25 000 soldiers, and create the War Measures Act, to be declared 21 days later",
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
        answer_explanation: "In response to Britain's request for soldiers, Canada sent 25,000 soldiers, and started the creation of the War Measures Act, which gave the government much more power during wars, such as the ability to censor the press, and the power to declare the Military Service Act",
    },
    QuizBlock {
        date: Date {
            year: 1917,
            month: "August",
            day: 29,
        },
        question: "300,000 soldiers had already volunteered to fight overseas, a massive amount for the population of only 8 million. However, Britain needs more soldiers.",
        answers: [
            QuizAnswer {
                answer: "Tell Britain that we cannot support the strain on our population",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Campaign to try to get another 10,000 soldiers to fight, but refuse any more after",
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
        answer_explanation: "Canada declares the Military Service Act due to pressure from Britain. This forces citizens into the Canadian Armed Forces. This was a very controversial decision, as we'll learn about later",
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
        answer_explanation: "At the time, the current Prime Minister was Robert Borden. In response to the French outcry, he called an election known as the \"Khaki Election\"(Khaki is the colour of the Canadian Armed Forces uniform), which was primary fought on the issue of conscription. Borden won the election, and continued the conscription program",
    },
    QuizBlock {
        date: Date {
            year: 1919,
            month: "June",
            day: 28,
        },
            question: "The war ended nearly a year ago. Canada is called to join deliberations in Paris on what the punishment for losing the war for Germany, Austria-Hungry, and Turkey will be. It is decided that Germany will have to pay reparations, Germany and the Ottoman Empire will hand overseas possession to France and Britain, and the drawing of new national boundaries to better represent ethnic borders will be written. Canada is at the Paris Peace Conference, and each leader is signing the Treaty of Versailles. Initially, Canada was brought in for deliberations, but would not sign the Treaty, as it was a British Colony. However, Canada contributed a lot to the war, and deserves recognition.",
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
        answer_explanation: "Canada had contributed a lot to the war, by sending many soldiers to Britain and providing more than 3 billion dollars in US money, and tons of equipment and raw materials to Britain. Initially, Canada was just brought in for deliberations, but would not be allowed to sign the treaty, as it was a British Colony. However, Canada contributed a lot to the war, and felt that they deserved to sign",
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
                answer: "Embrace the change, but assure Britain that we will continue to be a large supporter of British action",
                is_correct: false,
            },
            QuizAnswer {
                answer: "State that this change is not enough, and the British government needs to give us more sovereignty",
                is_correct: false,
            },
            QuizAnswer {
                answer: "Publicly state that this change gives Canada full sovereignty, and that we are fully separated from the Monarchy",
                is_correct: false,
            },
            ],
            answer_explanation: "The Statue of Westminster was introduced because it was felt that British Colonies should be considered as their own governing bodies and should be allowed a justice system not invovled with the Crown. Canada embraced this change fully, and it was a major step towards Canada's independence",
            },
        QuizBlock {
            date: Date {
                year: 1939,
                month: "September",
                day: 10,
            },
            question: "WW2 is breaking out. Britain asks for help.",
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
                    answer: "Do not declare war",
                    is_correct: false,
                },
                QuizAnswer {
                    answer: "Declare war on Germany but discourage Infantry Battalions, and instead provide resources to assist Britain",
                    is_correct: true,
                },
            ],
            answer_explanation: "The government predicted a high amount of casualties, and discouraged citizens from volunteering in Infantry Battalions. Instead, they provided resources to Britain, such as food, equipment, and raw materials. This would help the cause, and make sure Canada would not suffer from a large population loss due to the war",
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
                answer: "Provide naval defence against German submarines",
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
        answer_explanation: "France had just fallen, and Britain was in desperate need of help. Britain was struggling from German submarines, and so asked Canada to provide defense from them. Additionally, they needed more soldiers, equipment, and supplies",
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
        answer_explanation: "Known as the \"Billion Dollar Gift\", Canada sent over 3 billion dollars to Britian to help fund the war. This was a massive amount of money, and was a huge help to Britain",
    },
QuizBlock {
    date: Date {
        year: 1956,
        month: "October",
        day: 29,
    },
    question: "Britain, France, and Israel devise to invade Egypt and the Gaza Strip. Britain asks for our help.",
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
    answer_explanation: "Canada flately refuses to assit in the invasion, one of only a few times Canada has refused to assist Britain in a war",
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
    answer_explanation: "This marked a major step towards Canada's independence, as having an independant flag, to the world, is a sign of independence from Britain. The committee designed the current Canadian flag, which was adopted in 1965. The flag was designed by George Stanley, and was based on the flag of the Royal Military College of Canada",
},
QuizBlock {
    date: Date {
        year: 1980,
        month: "May",
        day: 14,
    },
    question: "Premier of Quebec René Lévesque is calling a referendum to decide whether Quebec should advocate to secede from Canada.",
    answers: [
        QuizAnswer {
            answer: "Make a statment supporting Quebec's attempt at secession, and that we hope they succeed",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make no statement on the matter",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make a statement saying Canada does support the referendum, and it is unconstitutional, disrespectful, and stupid",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make a statement that the government does not support it, Quebec is a very important part of Canadian culture, and that it would be tragic to lose such an important part of our culture",
            is_correct: true,
        },
    ],
    answer_explanation: "The referendum ended in a 40/60 split, with 60% of Quebecers voting to stay with Canada. The fact that this was called at all shows us how the Quebecois feel about Canada, and how they feel about their relationship with the British Monarchy",
},
QuizBlock {
    date: Date {
        year: 1982,
        month: "April",
        day: 17,
    },
    question: "The party feels that we could be losing favour with Canadians, and feels we should do something to gain favour.",
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
    answer_explanation: "Patriating the Canadian Constitution and creating a Charter of Rights and Freedoms was a significant step towards gaining favour with Canadians and ensuring their rights and freedoms are protected",
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
    answer_explanation: "In a famous speech by Jean Chrietien, he talked about how important Quebec was to Canada, and how it would be a great loss to Canada if Quebec were to secede. He made this speech, because Quebec truly is an important part of Canada, and the Canadian economy would suffer greatly from this seccesion. This time, the vote was incredibly close",
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
            answer: "Discuss that you are obviously supportive of them moving to Canada",
            is_correct: true,
        },
        QuizAnswer {
            answer: "Publicly apologize to the queen, and attempt to deny them citizenship on the grounds of suspicious criminal history",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Ignore questions about it and remain neutral",
            is_correct: false,
        },
    ],
    answer_explanation: "Expressing support for Prince Harry and Megan's decision to move to Canada is a great diplomatic approach that respects their personal choices, while also making sure the Queen still understand that the prime minister is loyal to her",
},
QuizBlock {
    date: Date {
        year: 2022,
        month: "September",
        day: 9,
    },
    question: "The queen has just died.",
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
            answer: "Announce that Canada will be fully leaving the monarchy, severing all ties in government",
            is_correct: false,
        },
        QuizAnswer {
            answer: "Make a speech about how important she was to Canada, but that we are very excited to have King Charles as our new Monarch",
            is_correct: false,
        },
    ],
    answer_explanation: "After the queen had passed, the prime minister made a heartfelt speech, where he was moved to tears. This showed our appreciation for the British Monarchy, and how much we value our relationship with Britain",
},
    ];
}





pub fn build_cyoa() -> Vec<CyoaBlock> {
    Vec::from([
        CyoaBlock {
            date: Date {day: 28, month: "July", year: 1914},
            question: "War is breaking out. You meet with your cabinet to decide what to do about it.",
            answers: [
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "ultimatum",
                    answer: "Wait for instructions from the Monarchy",
                    public_favor: 1.0,
                    british_favor: 2.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "peacekeeping1",
                    answer: "Send soldiers to act as peacekeepers",
                    public_favor: 1.5,
                    british_favor: 0.8,
                }, 
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Send soldiers to aid Serbia",
                    public_favor: 1.0,
                    british_favor: 1.2,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Send the 25 000 soldiers they requested, but refuse more",
                    public_favor: 0.9,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Send 25 000 soldiers, and create the war measures act, to be declared 21 days later.",
                    public_favor: 0.8,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Refuse to enter into the war",
                    public_favor: 0.95,
                    british_favor: 0.5,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Tell Britain that we cannot support the strain on our population",
                    public_favor: 0.8,
                    british_favor: 0.2,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Campaign to try to get another 10 000 soldiers to fight, but refuse any more after",
                    public_favor: 0.8,
                    british_favor: 0.6,
                },
                CyoaAnswer {
                    game_over: false, 
                    pointer_alias: "riotsinquebec",
                    answer: "Declare the Military Service Act, forcing conscription to all able male citizens between 20 and 45.",
                    public_favor: 0.5,
                    british_favor: 1.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "riotsinquebec",
                    answer: "Declare the Military Service Act, forcing conscription to all able citizens between 20 and 45.",
                    public_favor: 0.4,
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
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Continue the program to conscript more soldiers",
                    public_favor: 0.2,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "End the Military Service Act",
                    public_favor: 0.8,
                    british_favor: 0.4,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "treaty",
                    answer: "Call an election fought on the matter of Conscription",
                    public_favor: 1.0,
                    british_favor: 1.0,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Demand the right to sign the Treaty of Versailles.",
                    public_favor: 1.0,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Ask Britain politely to sign the Treaty Independently",
                    public_favor: 0.8,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Do nothing",
                    public_favor: 0.5,
                    british_favor: 0.6,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "peacekeeping2",
                    answer: "Send more peacekeepers",
                    public_favor: 1.0,
                    british_favor: 0.7,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Withdraw all forces and refuse to participate in the war.",
                    public_favor: 0.7,
                    british_favor: 0.3,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Order your soldiers to fight for Britain",
                    public_favor: 0.7,
                    british_favor: 0.8,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "majorlossgermany",
                    answer: "Order your soldiers to fight for Germany",
                    public_favor: 0.1,
                    british_favor: -0.5,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "300000",
                    answer: "Order your soldiers to fight for Britain",
                    public_favor: 0.7,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Withdraw all forces and refuse to participate in the war.",
                    public_favor: 0.7,
                    british_favor: 0.3,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "whiteflag",
                    answer: "Tell your soldiers to return to being peacekeepers.",
                    public_favor: 0.7,
                    british_favor: 0.5,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "whiteflag",
                    answer: "Tell Germany you can no longer support them and withdraw all soldiers",
                    public_favor: 0.7,
                    british_favor: 0.7, 
                },
                CyoaAnswer {
                    game_over: true,
                    pointer_alias: "invasion",
                    answer: "Order your soldiers to continue fighting for Germany",
                    public_favor: 0.1,
                    british_favor: -0.5,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Apologize deeply for your mistake, and pledge forever allegiance in the future.",
                    public_favor: 0.75,
                    british_favor: 0.8,
                },
                CyoaAnswer {
                    game_over: true,
                    pointer_alias: "invasion",
                    answer: "Tell Britain you will never apologize, and that you will never be their ally again.",
                    public_favor: 0.1,
                    british_favor: -1.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "westminster",
                    answer: "Pay reparations and apologize",
                    public_favor: 0.8,
                    british_favor: 0.9,
                },
                CyoaAnswer {
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
                    game_over: false,
                    pointer_alias: "filler",
                    answer: "filler1",
                    public_favor: 0.0,
                    british_favor: 0.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "filler",
                    answer: "filler2",
                    public_favor: 0.0,
                    british_favor: 0.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "filler",
                    answer: "filler3",
                    public_favor: 0.0,
                    british_favor: 0.0,
                },
                CyoaAnswer {
                    game_over: false,
                    pointer_alias: "filler",
                    answer: "filler4",
                    public_favor: 0.0,
                    british_favor: 0.0,
                },
            ],
            alias: "westminster",
        }
    ])
}