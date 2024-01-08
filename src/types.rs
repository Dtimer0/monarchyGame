pub struct QuizBlock {
    question: String,
    answers: [QuizAnswer; 4],
    answer_explanation: String,
}

pub struct QuizAnswer {
    answer: String,
    is_correct: bool,
}

pub struct CyoaBlock {
    question: String,
    answers: [cyoaAnswer; 4],
    leader_alias: String,
}

pub struct CyoaAnswer {
    alias: String,
    answer: String,
    is_correct: bool,
    weight: f32,
    public_favor: f32,
}