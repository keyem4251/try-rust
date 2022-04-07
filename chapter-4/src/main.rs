fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // point allocated here.
        let label = format!("{:?}", point); // label allocated here.
        assert_eq!(label, "(0.625, 0.5)"); // both dropped here.
    }

    struct Person { name: String, birth: i32 }
    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s;
    // let u = s.clone(); value s borrowed here after move.

    let mut _s2 = "Govinda".to_string();
    _s2 = "Siddhartha".to_string();

    let mut _s3 = "Govinda".to_string();
    let _t = _s3;
    _s3 = "Siddhartha".to_string(); // not uninitialized because new value allocated

    let x = vec![10, 20, 30];
    if x.len() > 2 {
        f(x);
    } else {
        g(x);
    }
    // h(x); x is uninitialized here if either path use it

    let mut _y = vec![10, 20, 30];
    // while x.len() > 2 {
    //     // g(x); this only x would be moved in first iteration, uninitialized in second
    //     g(y); // move from y
    //     y = i(); // give y a fresh value
    // }

    // Build a vector of the strings "101", "102", ... "105"
    let mut v2 = Vec::new();
    for i in 101..106 {
        v2.push(i.to_string());
    }
    // 1. Pop a value off the end of the vector
    let fifth = v2.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move  a value out of the middle of the vector, and move the last element into its spot
    let second = v2.swap_remove(1);
    assert_eq!(second, "102");
    println!("{:?}", v2);

    // 3. Swap in another value for the one we're taking out
    let third = std::mem::replace(&mut v2[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v2, vec!["101", "104", "substitute"]);

    let v3 = vec!["liberte".to_string(), "egalite".to_string(), "fraternite".to_string()];
    for mut s in v3 {
        s.push('!');
        println!("{}", s);
        // println!("{:?}", v3); value borrowed here after move error
    }
    // println!("{:?}", v3); value borrowed here after move error

    struct Person2 { name: Option<String>, birth: i32 }
    let mut composers2 = Vec::new();
    composers2.push(Person2 { name: Some("Palestrine".to_string() ), birth: 1525 });
    // let first_name = std::mem::replace(&mut composers2[0].name, None);
    let first_name = composers2[0].name.take();
    assert_eq!(first_name, Some("Palestrine".to_string()));
    assert_eq!(composers2[0].name, None);

    let str1 = "somnambulance".to_string();
    let _str2 = str1;
    // println!("{}", str1); error

    let num1 = 10;
    let _num2 = num1;
    println!("{}", num1);

    #[derive(Copy, Clone)]
    struct Label { number: u32 }
    fn print(l: Label) { println!("STAMP: {}", l.number) }
    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here.
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan); // dropped here.
}

fn f(x: Vec<i32>) {
    println!("{:?}", x);
}

fn g(x: Vec<i32>) {
    println!("{:?}", x);
}

fn _h(x: Vec<i32>) -> Vec<i32>{
    println!("{:?}", x);
    x
}

fn _i() -> Vec<i32>{
    vec![10, 20, 30]
}
