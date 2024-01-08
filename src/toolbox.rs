use crate::types::QuizBlock;

pub fn readline() -> String {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    return line;
}
impl QuizBlock {
    pub fn date(&self) -> String {
        return format!("{} {}, {}", self.date.month, self.date.day, self.date.year);
    }
}


