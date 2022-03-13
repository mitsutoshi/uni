use std::io;

fn main() {
    let mut input = String::new();
    let mut lines: Vec<String> = Vec::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let s = input.to_string();
        if !lines.contains(&s) {
            print!("{}", s);
            lines.push(s);
        }
        input.clear();
    }
}
