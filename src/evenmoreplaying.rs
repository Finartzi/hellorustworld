pub fn playingevenmore() {
    structures();
    println!(" --- ");
    enumerations();
    println!(" --- ");
    enumerate();
    println!(" --- ");
    loopings();
    println!(" --- ");
    let mut x: u8 = value_in_cents(Coin::Nickel);
    println!("{}", x);
    println!(" --- ");
    letting();
    println!(" --- ");
    vectors();
    println!(" --- ");
}

struct Person {
    name: String,
    age: u8,
    hobby: String,
}

// Tuple structs (named structs)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// making a struct printable
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
fn structures() {
    let age: u8 = 30;
    let p: Person = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };
    println!("name: {}, age: {} and hobby: {}", p.name, p.age, p.hobby);

    let v: Point = Point(0, 127, 255);
    check_color(v);
    println!("Success");

    let scale: i32 = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); // Print debug in stderr
    println!("{:?}", &rect1) // printing with debug notation in stdout
}

fn check_color(p: Point) {
    let Point(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn enumerations() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    // Option Enum
    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        println!("Success");
    } else {
        panic!("NEVER LET THIS RUN!");
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn enumerate() {
    let a: [i32; 4] = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("The {}. element is {}", i + 1, v);
    }
}

fn loopings() {
    let mut count: i32 = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }
        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }
    assert!(count == 30);
    println!("success loopings");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Pattern match - not in other languages?
fn value_in_cents(coin: Coin) -> u8 {
    // all cases must be handled in match, see next function letting
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn letting() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

enum MyEnum {
    Foo,
    Bar,
}

fn vectors() {
    let mut count: i32 = 0;
    let v: Vec<MyEnum> = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    assert_eq!(count, 2);
    println!("Success vector")
}
