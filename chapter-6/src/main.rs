fn main() {
    section_6_1();
    section_6_2();
    section_6_3();
}

fn section_6_1() {
    let i = 10;
    let status = if i > 5 {
        println!("{:?}", i);
    } else {
        println!("if expression");
    };
}

fn section_6_2() {
    let i = Some(10);
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    plus_one(i);

    let msg = {
        let mut dandelion_control = 10;
        dandelion_control += 20;
        dandelion_control
    };

    let msg2 = {
        let a = 10;
        ;
        a
    };
}

fn section_6_3() {
    let name;
    let a = 10;
    if a > 2 {
        name = "a";
    } else {
        name = "b";
    }

    child();

    fn child() {
        // do not use variable name and a
        let _b = 10;
    }
}
