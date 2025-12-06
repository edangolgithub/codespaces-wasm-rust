use std::collections::HashMap;

pub fn evan_hashmap() {
    let mut scores = HashMap::new();
    scores.insert("name1", "evan");
    scores.insert("name", "ram");
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    println!("{:?}", &scores["name"]);

    let mut map = HashMap::new();

    map.insert("key1", 100);
    map.insert("key1", 999); // overwrites
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    id: u32,
    name: String,
}

fn sec() {
    let mut map: HashMap<Person, i32> = HashMap::new();

    map.insert(
        Person {
            id: 1,
            name: "Evu".into(),
        },
        100,
    );

    println!("{:?}", map);
}
fn borrow() {
    let x = 10;
    let f = || println!("{}", x); // borrows &x

    let mut counter = 0;
    let mut f = || counter += 1; // borrows &mut counter

    f();

    let mut counter1 = 0;
    let mut f1 = || counter1 += 1; // borrows &mut counter
    f1();

    let s = String::from("hello");
    let f = move || println!("{}", s); // owns s
}

// let x = 42;
// let c = || x + 1;

// struct Closure { x: i32 }

// impl Closure {
//     fn call(&self) -> i32 {
//         self.x + 1
//     }
// }

pub fn c_losure() {
    let add = |a, b| a + b;

    println!("{}", add(2, 3)); // 5
}
pub fn closure1() {
    let x = 10;
    let add_x = |v| v + x;
    println!("{}", add_x(5)); // 15
}
pub fn closure2() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
}
