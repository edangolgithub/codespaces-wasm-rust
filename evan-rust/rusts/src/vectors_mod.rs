use std::sync::{LazyLock, Mutex};

static VECTOR: LazyLock<Mutex<Vec<i32>>> = LazyLock::new(|| Mutex::new(vec![1, 2, 3, 4, 5]));

pub fn run() {
    let mut v: Vec<i32> = Vec::new();
    v.push(67);
    v.push(523);
    v.push(2);
    v.push(87);
    v.push(12);
    let pop_val = v.pop();

    //println!("{}",pop_val.unwrap());
    match pop_val {
        Some(12) => println!("{:?} was popped", pop_val),
        Some(x) => println!("{x} was poped"),
        None => println!("valse was empt"),
    };

    println!("{:?}", v);

    let m = v.get(2);
    println!("{}", m.unwrap());
}

pub fn owner1() {
    let mut v = VECTOR.lock().unwrap();
    v.push(98);
    println!("{:?}", v);
}
pub fn owner2() {
    let mut v = VECTOR.lock().unwrap();
    v.push(99);
    println!("{:?}", v);
}

pub fn vec_iter() {
    let v = VECTOR.lock().unwrap();
    for x in v.iter() {
        println!("{}", x); // x is &i32
    }
    println!("{:?}", v); // still usable ✔
}

pub fn vec1() {
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x); // x is &i32
    }
    println!("{:?}", v); // still usable ✔
}
