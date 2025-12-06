use std::sync::{LazyLock, Mutex};

static VECTOR: LazyLock<Mutex<Vec<i32>>> = LazyLock::new(|| Mutex::new(vec![1, 2, 3, 4, 5]));

pub fn owner1(){
    let mut v = VECTOR.lock().unwrap();
    v.push(98);
    println!("{:?}", v);
}
pub fn owner2(){
    let mut v = VECTOR.lock().unwrap();
    v.push(99);
    println!("{:?}", v);
}

pub fn vec_iter(){
    let v = VECTOR.lock().unwrap();
    for x in v.iter() {
        println!("{}", x); // x is &i32
    }
    println!("{:?}", v); // still usable ✔

}

pub fn vec1(){
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x); // x is &i32
    }
    println!("{:?}", v); // still usable ✔

}

pub fn run() {
 vec1();
}
