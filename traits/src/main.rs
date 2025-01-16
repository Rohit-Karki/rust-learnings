// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, 
// both of which are string slices that live at least as long as lifetime 'a. The function signature also tells 
// Rust that the string slice returned from the function will live at least as long as lifetime 'a. 
// In practice, it means that the lifetime of the reference returned by the longest function is the same as the 
// smaller of the lifetimes of the values referred to by the function arguments. 
// These relationships are what we want Rust to use when analyzing this code.

fn main() {
    let r;
    
    let x = 5;
    r = &x;
    
    println!("r: {r}");
}
