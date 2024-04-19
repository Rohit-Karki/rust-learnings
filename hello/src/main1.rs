use std::io::{self, BufRead};
// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function...
//                         // ... and so is no longer valid here

//     let x = 5; // x comes into scope

//     makes_copy(x); // x would move into the function,
//                    // but i32 is Copy, so it's okay to still
//                    // use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main(){
//  let mut s = String::from("hello");

//     let r1 = &mut s;

//     println!("{}", r1);
// let r2 = &mut s;
//     println!("{}", r2);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// let mut user1 = User {
//     active: true,
//     username: String::from("someusername123"),
//     email: String::from("someone@example.com"),
//     sign_in_count: 1,
// };
// user1.email = String::from("anotheremail@example.com");
fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_line(&mut buffer);
    solve(&buffer)
}

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
