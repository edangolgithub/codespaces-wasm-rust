pub fn evau() -> i32 {
    println!("hello");

    1
}

pub fn arr() {
    let nums = [1, 2, 3, 4, 5];
    let v = vec![1, 2, 3];

    for x in v.iter() {
        println!("{}", x); // x is &i32
    }

    println!("{:?}", v); // still usable ✔
}

pub fn genrik<T>(a: T) -> T {
    a
}
