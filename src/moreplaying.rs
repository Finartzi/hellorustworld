pub fn playmore() {
    boxing();
    tuples();
    mutable();
    using_ref();
    replace_part();
}

fn boxing() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // 'name' is moved out of person, but 'age' is referenced
    let Person { name, ref age } = person;
    println!("The persons's age is {}", age);
    println!("The persons's name is {}", name);
}

fn tuples() {
    let t: (String, String) = (String::from("Hello"), String::from("world"));
    let _s: String = t.0.clone();
    println!("{:?}", t);

    let len = calculate_length(&_s);
    println!("The length of '{}' is {}.", _s, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutable() {
    let mut s: String = String::from("Hello");
    change(&mut s);
    println!("{}", &s);
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);
}

fn change(somestring: &mut String) {
    somestring.push_str(", world again");
}

fn using_ref() {
    let c: char = '有';
    let r1: &char = &c;

    let ref r2 = c;
    assert_eq!(*r1, *r2);

    // check the equality of the two address strings
    assert_eq!(get_address(r1), get_address(r2));
    println!("Success");

    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..11];

    print!("{}", hello);
    println!("{}", world);

    let t: &str = "Hello another world";
    println!("{}", t);

    let u: Box<str> = "Hello somewhat boxed".into();
    greetings(&u);

    let mut v: String = String::from("");
    v.push_str("Hallo myself");
    v.push('!');
    println!("{}", v);
}

fn greetings(s: &str) {
    println!("{}", s);
}
fn get_address(r: &char) -> String {
    format!("{:p}", r)
}

fn replace_part() {
    let s: String = String::from("I like dogs");
    // allocate new memory and store the modified string there
    let s1: String = s.replace("dogs", "cats");
    println!("{}", s1);
}
