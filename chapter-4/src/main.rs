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

    let mut y = vec![10, 20, 30];
    while x.len() > 2 {
        // g(x); this only x would be moved in first iteration, uninitialized in second
        g(y); // move from y
        y = i(); // give y a fresh value
    }
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
    println!(x);
}

fn g(x: Vec<i32>) {
    println!(x);
}

fn h(x: Vec<i32>) -> Vec<i32>{
    println!(x);
    x
}

fn i() -> Vec<i32>{
    vec![10, 20, 30]
}
