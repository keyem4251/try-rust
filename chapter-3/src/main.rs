fn main() {
    println!("build_vector: {:?}", build_vector());
    println!("int_wrap: {}", int_wrap());
    println!("cast_int: {:?}", cast_int());
    println!("float_: {:?}", float_());
    println!("bool_: {:?}", bool_());
    println!("char_: {:?}", char_());
    println!("tuple_: {:?}", tuple_());
    println!("pointer_: {:?}", pointer_());
    println!("list_vector_slice_: {:?}", list_vector_slice_());
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(20);
    v.push(10);
    v
}

fn int_wrap() -> i32 {
    let big_val = i32::MAX;
    let x = big_val.wrapping_add(1);
    x
}

fn cast_int() {
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2u16.pow(4), 16); // 2の4乗
    assert_eq!((-4i32).abs(), 4); // 絶対値
    assert_eq!(0b101101u8.count_ones(), 4)
}

fn float_() -> f64 {
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
    assert!((-1. / f32::INFINITY).is_sign_negative());

    (2.0_f64).sqrt()
}

fn bool_() {
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

fn char_() {
    assert_eq!('*' as i32, 42);
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('6'.is_digit(10), true);
}

fn tuple_() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

fn pointer_() {
    let text = 1;
    let a = &text;
    println!("{}", *a);
    println!("{}", text);
    let t = (12, "eggs");
    let _b = Box::new(t); // allocate a tuple in the heap
}

fn list_vector_slice_() {
    // list
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 1, 9];
    chaos.sort();
    assert_eq!(chaos, [1, 3, 5, 9]);

    // vector
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    fn _new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
        vec![0; rows * cols]
    }

    let mut v2 = Vec::new();
    v2.push("step");
    v2.push("on");
    v2.push("no");
    v2.push("pets");
    assert_eq!(v2, vec!["step", "on", "no", "pets"]);

    let v3: Vec<i32> = (0..5).collect();
    assert_eq!(v3, [0, 1, 2, 3, 4]);

    let mut v4 = vec!["a man", "a plan", "a canal", "panama"];
    v4.reverse();
    assert_eq!(v4, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v5 = Vec::with_capacity(2);
    assert_eq!(v5.len(), 0);
    assert_eq!(v5.capacity(), 2);

    v5.push(1);
    v5.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v5.push(3);
    assert_eq!(v5.len(), 3);
    assert_eq!(v5.capacity(), 4);

    let mut v6 = vec![10, 20, 30, 40, 50];
    v6.insert(3, 35);
    assert_eq!(v6, [10, 20, 30, 35, 40, 50]);

    v6.remove(1);
    assert_eq!(v6, [10, 30, 35, 40, 50]);

    let mut v7 = vec!["carmen", "miranda"];
    assert_eq!(v7.pop(), Some("miranda"));
    assert_eq!(v7.pop(), Some("carmen"));
    assert_eq!(v7.pop(), None);

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for language in languages {
        println!(
            "{}: {}",
            language,
            if language.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
