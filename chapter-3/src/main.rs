fn main() {
    println!("build_vector: {:?}", build_vector());
    println!("int_wrap: {}", int_wrap());
    println!("cast_int: {:?}", cast_int());
    println!("float_: {:?}", float_());
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
