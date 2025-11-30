use colored::*;

pub fn read_input<T: std::str::FromStr>() -> T {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        match trimmed.parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                println!("{}", "⚠️ Invalid input, try again.".yellow().bold());
            }
        }
    }
}
