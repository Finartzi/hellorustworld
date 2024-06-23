// https://www.youtube.com/watch?v=BpPEoZW5IiY

fn main() {
    println!("Hello, world of Rust!");
    just_playing();
}

use std::mem::size_of_val;
fn just_playing() {
    for c in 'a'..='z' {
        println!("{}", c as u8);
    }

    let c1: char = 'a'; // 4 bytes
    println!("{}", size_of_val(&c1));

    let x = 42;
    let y = 10;
    let z = add_numbers(x, y);
    println!("{}", z);

    pointers();
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}

fn pointers() {
    let x: Box<i32> = Box::new(5);
    let mut y: Box<i32> = Box::new(1);

    *y = 4;
    assert_eq!(*x, 5);
    println!("Success")
}
