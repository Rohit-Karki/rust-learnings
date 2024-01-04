pub fn solve(input: &str) {
    let ans: u32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|char| char.is_digit(10));
            let first = chars
                .next()
                .expect("the line should have at least one digit");

            let num = match chars.last() {
                Some(last) => {
                    format!("{}{}", first, last)
                }
                None => {
                    format!("{}{}", first, first)
                }
            };
            num.parse::<u32>().unwrap()
        })
        .sum();
    print!("Answer is {}", ans)
}
