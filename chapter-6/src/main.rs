fn main() {
    section_6_1();
}

fn section_6_1() {
    let i = 10;
    let status = if i > 5 {
        println!("{:?}", i);
    } else {
        println!("if expression");
    };
}
