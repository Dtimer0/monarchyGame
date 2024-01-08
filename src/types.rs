pub struct QuizBlock {
    pub question: String,
    pub answers: [QuizAnswer; 4],
    pub answer_explanation: String,
}

pub struct QuizAnswer {
    pub answer: String,
    pub is_correct: bool,
}

pub struct CyoaBlock {
    pub question: String,
    pub answers: [CyoaAnswer; 4],
    pub leader_alias: String,
}

pub struct CyoaAnswer {
    pub alias: String,
    pub answer: String,
    pub is_correct: bool,
    pub weight: f32,
    pub public_favor: f32,
}

