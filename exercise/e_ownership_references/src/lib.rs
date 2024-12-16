pub fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("plural")
    } else {
        println!("singular")
    }
}

pub fn change(arg: &mut String) {
    if !arg.ends_with("s") {
        arg.push_str("s");
    }
}

pub fn eat(arg: String) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

pub fn bedazzle(arg: &mut String) {
    *arg = "sparkly".to_string();
}
