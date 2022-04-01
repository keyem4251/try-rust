fn main() {
    println!("{:?}", build_vector());
    println!("{}", int_wrap());
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
