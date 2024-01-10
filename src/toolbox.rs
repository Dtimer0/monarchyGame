use std::io::Write;

use crate::types::*;
#[allow(unreachable_code)]

pub fn readline() -> String {
    let mut line = String::new();
    print!("Enter: ");
    let _ = std::io::stdout().flush();   
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
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