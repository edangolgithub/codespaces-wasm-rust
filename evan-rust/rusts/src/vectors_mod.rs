use std::{
    sync::{LazyLock, Mutex},
    vec,
};

static VECTOR: LazyLock<Mutex<Vec<i32>>> = LazyLock::new(|| Mutex::new(vec![1, 2, 3, 4, 5]));

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

pub fn vec2() {
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    v.shrink_to(2);
    println!("{:?}", v);
    println!("{:?}", v.capacity());

    v.extend([4, 5, 6]);
    println!("{:?}", v);
    println!("{:?}", v.capacity());

    v.truncate(1);
    println!("{:?}", v);
    println!("{:?}", v.capacity());

    v.extend([2, 3, 4, 5, 6]);
    v.swap_remove(0);
    println!("{:?}", v);
}

pub fn vec3() {
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

pub fn vec4() {
    let mut vec = vec!['a', 'b', 'c'];
    vec.insert(1, 'd');
    println!("{:?}", vec);
    vec.insert(4, 'e');
    println!("{:?}", vec);
    vec.remove(1);
    println!("{:?}", vec);
}

pub fn vec5() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    vec.retain(|a| a % 2 == 0);
    println!("{:?}", vec);
}

pub fn vec6() {
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    println!("{:?}", vec);
    println!("{:?}", vec2);
}

pub fn borrow_checker() {
    fn foo(v: Vec<i32>) {
        println!("{:?}", v);
    }

    let a = vec![1, 2, 3];
    foo(a); // a is moved
    //foo(a); // ❌ cannot use a again
    fn bar(v: &Vec<i32>) {
        println!("{:?}", v);
    }

    let a = vec![1, 2, 3];
    bar(&a); // ok
    bar(&a); // ok again
}

pub fn fun_drain() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.drain(1..3); //[1, 4, 5]
    println!("{:?}", v);
}
pub fn fn_split() {
    let mut v = vec![1, 2, 3, 4, 5];
    let v1 = v.split_off(2);
    println!("\nsplit off");
    println!("{:?}", v);
    println!("{:?}", v1);

    let v2 = vec![1, 2, 3, 4, 5];
    let v3 = v2.split_at(1);
    println!("\nsplit at \n{:?}", v3);

    let mut v4 = [1, 0, 3, 0, 5, 6];
    let (left, right) = v4.split_at_mut(2);

    // println!("split at mut {:?}", v4); // ❌ should fail here
    println!("{:?}", left); // last use of left
    println!("{:?}", right); // last use of right

    println!("{:?}", v4); // ✔ works

    let slice = [10, 40, 33, 20];
    let iter = slice.split(|num| num % 3 == 0);
    println!("\nsplit");
    for part in iter {
        println!("{:?}", part);
    }
    let slice = [10, 40, 33, 20];
    let parts: Vec<&[i32]> = slice.split(|n| n % 3 == 0).collect();

    println!("\nsplit collect");
    println!("{:?}", parts); // prints all parts at once
}

pub fn fn_chunks() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let x = v.chunks(3);
    println!("{:?}", x);

    let slice = ['l', 'o', 'r', 'e', 'm'];
    let (chunks, remainder) = slice.as_chunks::<2>();
    println!("{:?}", chunks);
    println!("{:?}", remainder);
}

pub fn fn_within() {
    let mut characters = vec!['a', 'b', 'c', 'd', 'e'];
    characters.extend_from_within(3..);
    println!("{:?}", characters);

    let mut numbers = vec![0, 1, 2, 3, 4];
    numbers.extend_from_within(1..3);
    println!("{:?}", numbers);

    let mut strings = vec![
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    ];
    strings.extend_from_within(1..=2);
    println!("{:?}", strings);
}

pub fn fn_swap() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.swap(0, 1);
    println!("{:?}", v);
}
pub fn fn_iter() {
    let v = vec![1, 2, 3, 4, 5];
    let x = v.iter();
    for index in x {
        println!("{}", index);
    }
    println!("{:?}", v);
    let x1 = &mut [1, 2, 4];
    for elem in x1.iter_mut() {
        *elem += 2;
    }
    println!("{:?}", x1);
}

pub fn fn_contains() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let c = v.contains(&4);
    println!("tt{:?}", c);
    assert!(v.starts_with(&[1]));

    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    assert_eq!(s.binary_search(&13), Ok(9));

    // let mut slice = ["foo", "Foo", "BAZ", "Bar", "bar", "baz", "BAZ"];

    // let (dedups, duplicates) = slice.dedup_by(|a, b| a.eq_ignore_ascii_case(&b));
    // println!("{:?}", dedups);
    // println!("{:?}", duplicates);
}

pub fn fn_tovec() {
    let arr = [1, 2, 3, 4];
    let v = arr.to_vec();
    println!("{:?}", v);
}


pub fn run() {
   let vt=vec![1,2,3,4,5];
   let m= vt.repeat(5);
   println!("{:?}",m);
}
