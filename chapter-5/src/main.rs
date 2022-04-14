use std::collections::HashMap;

fn main() {
    value_ref();
    value_ref2();
    value_ref3();
    ref_5_2();
}

type Table = HashMap<String, Vec<String>>;

fn show1(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn value_ref() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    show1(table);
    // assert_eq!(table["Gesualdo"][0], "many madrigals"); value borrowed here after move
}

fn show2(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn value_ref2() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);
    show2(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table)
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn value_ref3() {
    // Back to Rust code from this point onward.
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);
    assert_eq!(y, 64);

    struct Anime { name: &'static str, _bechdel_pass: bool }
    let aria = Anime { name: "Aria: The Animation", _bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let x2 = 10;
    let y2 = 20;
    let mut r = &x2;

    if y2 > 10 {
        r = &y2;
    }
    assert!(*r == 10 || *r == 20);

    let rx = &x2;
    let ry = &y2;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert_eq!(rrx, rry);
    // assert!(!std::ptr::eq(rx, ry)); compare address and panic

    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6); // create non name variable and equal life time to r
    assert_eq!(r + &1079, 1729); // &1079 life time is value_ref3
}

fn ref_5_2() {
    // borrowed value does not live long enough x
    // {
    //     let r;
    //     {
    //         let x = 1;
    //         r = &x;
    //     }
    //     assert_eq!(*r, 1);
    // }

    {
        let x = 1;
        {
            let r = &x;
            assert_eq!(*r, 1)
        }
    }

    fn g(p: &i32) { println!("{:?}", p) }
    let x = 10;
    g(&x);

    // fn f(p: &'static i32) { println!("{:?}", p) } error
    // f(&x);

    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s { s = r; }
        }
        s
    }
    let s;
    {
        let parabola = [9, 3, 18, 3, 2, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
    // assert_eq!(*s, 0); error
}
