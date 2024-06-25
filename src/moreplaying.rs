pub fn playmore() {
    boxing();
    tuples();
    mutable();
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
