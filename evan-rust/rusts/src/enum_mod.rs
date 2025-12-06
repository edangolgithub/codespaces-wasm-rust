pub fn example() {
    let m = [String::from("hello"), String::from("world")];
    let n = m.get(12);
    let o = m.get(1);
    println!("{:?}", n.expect("may be 0"));
    println!("{:?}", o.unwrap().to_uppercase());
}

pub fn example1() {
    let m = [String::from("hello"), String::from("world")];
    let n = m.get(0);
    match n {
        Some(n) => println!("{:?}", n),
        None => println!("may be 0"),
    }
}
pub fn is_item_in_stock(flag1: bool, flag2: bool) -> Option<bool> {
    if flag1 && flag2 {
        Some(true)
    } else {
        Some(false)
    }
}
use std::fs;

pub fn read_file_for_result() {
    let content = fs::read_to_string("src/hello.txt");
    match content {
        Ok(text) => println!("{}", text),
        Err(e) => println!("Error reading file: {}", e),
    }
}

pub fn result_parse() {
    let textnum = "50";
    let x = textnum.parse::<i32>().unwrap();

    let name = "evan";
    let y = name.parse::<i32>();

    println!("{:?}", x);
    println!("{:?}", y);
}
pub fn divide(nominator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("zero".to_string())
    } else {
        Ok(nominator / denominator)
    }
}

pub fn run() {
    let result = divide(23.0, 0.0);
    match result {
        Ok(x) => println!("Result:{}", x),
        Err(e) => println!("Error:{}", e),
    }

    println!("{:?}", divide(34.0, 0.0));
}
