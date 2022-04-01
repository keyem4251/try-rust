fn main() {
    println!("{:?}", build_vector());
}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(20);
    v.push(10);
    v
}
