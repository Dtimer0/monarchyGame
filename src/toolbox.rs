pub fn readline() -> String {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    return line;
}