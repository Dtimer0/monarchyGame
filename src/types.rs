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
#[derive(Clone)]
pub struct CyoaBlock {
    pub date: Date,
    pub question: &'static str,
    pub answers: [CyoaAnswer; 4],
    pub alias: &'static str,
}
#[derive(Clone)]
pub struct CyoaAnswer {
    pub is_historically_accurate: bool,
    pub game_over: bool,
    pub pointer_alias: &'static str,
    pub answer: &'static str,
    pub public_favor: f32,
    pub british_favor: f32,
}
#[derive(Clone)]
pub struct Date {
    pub year: i16,
    pub month: &'static str,
    pub day: u8,
}

pub struct GameOver {
    pub alias: &'static str,
    pub reason: &'static str,
}
