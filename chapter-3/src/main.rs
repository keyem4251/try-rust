fn main() {
    println!("build_vector: {:?}", build_vector());
    println!("int_wrap: {}", int_wrap());
    println!("cast_int: {:?}", cast_int());
    println!("float_: {:?}", float_());
    println!("bool_: {:?}", bool_());
    println!("char_: {:?}", char_());
    println!("tuple_: {:?}", tuple_());
    println!("pointer_: {:?}", pointer_());
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

fn list_() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
}
