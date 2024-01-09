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
impl CyoaBlock {
    pub fn date(&self) -> String {
        return format!("{} {}, {}", self.date.month, self.date.day, self.date.year);
    }
}

pub fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}