pub fn stdin_trimmed_line() -> String {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Error reading line");
    String::from(buffer.trim())
}