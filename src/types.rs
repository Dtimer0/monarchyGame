pub struct QuizBlock {
    pub date: Date,
    pub question: &'static str,
    pub answers: [QuizAnswer; 4],
    pub answer_explanation: &'static str,
}

pub struct QuizAnswer {
    pub answer: &'static str,
    pub is_correct: bool,
}

pub struct CyoaBlock {
    pub date: Date,
    pub question: &'static str,
    pub answers: [CyoaAnswer; 4],
    pub leader_alias: &'static str,
}

pub struct CyoaAnswer {
    pub alias: &'static str,
    pub answer: &'static str,
    pub is_correct: bool,
    pub weight: f32,
    pub public_favor: f32,
}

pub struct Date {
    pub year: i16,
    pub month: &'static str,
    pub day: u8,
}

