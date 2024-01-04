fn plusOne(option:Option<i32>)->Option<i32> {
    match option{
        Some(x) => Some(x+1),
        None => None,
    }
}
let one = Some(1);
let two = plusOne(x);
