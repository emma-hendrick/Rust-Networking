pub fn output<S: Into<String>>(text: S) {
    println!("{}", text.into());
}

pub fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let line_length: usize = line.len();
    line.truncate(line_length - 2);

    return line;
}