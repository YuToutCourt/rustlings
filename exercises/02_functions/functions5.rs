// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

/*

In Rust they are two way to return a value from a function:
- using the return keyword

fn square(num: i32) -> i32 {
    return num * num;
}
- without using the return keyword, this on don't take a semicolon at the end of the expression

fn square(num: i32) -> i32 {
    num * num
}


*/

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
