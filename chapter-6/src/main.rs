fn main() {
    section_6_1();
    section_6_2();
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
    let i = 10;
    let display_name = match i > 5 {
        Some(b) => b,
        None => {
            let s = 10;
            s
        }
    };

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
