pub fn fn_init() {
    let hello_world = "Hello, World!";
    let hello_world: &'static str = "Hello, evan!";
    println!("{}", hello_world);
    let story = "Ance upon a time...";
    let ptr = story.as_ptr();
    let len = story.len();
    unsafe {
        println!("\npointer val is {:?}", *ptr);
        println!(
            "\npointer char val is {:?}",
            std::str::from_utf8_unchecked(&[*ptr])
        );

        let slice = std::slice::from_raw_parts(ptr, len);
        println!("\nslice is {:?}", slice);

        let str_value = std::str::from_utf8_unchecked(slice);
        println!("\n string {:?}", str_value);
    }
    let sparkle_heart = vec![240, 159, 146, 150];

    // We can use the ? (try) operator to check if the bytes are valid
    let sparkle_heart = str::from_utf8(&sparkle_heart);

    println!("\nfrom utf8{:?}", sparkle_heart.unwrap());
}

fn fn_split() {
    let sen = "A quick brown fox jumps over the";
    let x = sen.split_at(4);
    println!("{:?}", x);

    let mut sen1 = String::from("A quick brown fox jumps over the");
    sen1.insert(0, 'x');
    sen1.push('t');
    let mm = sen1.replace("A", "Z");
    println!("{:?}", mm);
    let y = sen1.split_at_mut(4);

    println!("{:?}", y);

    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    println!("{:?}", v);

    let v: Vec<&str> = "".split('X').collect();
    println!("{:?}", v);

    let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
    println!("{:?}", v);

    let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    println!("{:?}", v);

    let v: Vec<&str> = "AABBCC".split("DD").collect();
    println!("{:?}", v);

    let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
    println!("{:?}", v);

    let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
    println!("{:?}", v);

    let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "def", "ghi"]);
}

fn fn_chars() {
    let sen = "A quick brown fox jumps over the";
    let chars = sen.chars();
    //println!("{:?}", chars);
    for c in chars {
        print!("{}", c);
    }
    println!();

    let iter = "A few words".split_whitespace();
    for part in iter {
        println!("{}", part);
    }
    let ascii = "hello!\n";
    let non_ascii = "Grüße, Jürgen ❤";

    println!("{}", ascii.is_ascii());
    println!("{}", non_ascii.is_ascii());

    for c in "❤\n!".escape_unicode() {
        print!("{c}");
    }
}

fn fn_borro() {
    let s: &str = "a";
    let ss: String = s.to_owned();

    let v: &[i32] = &[1, 2];
    let vv: Vec<i32> = v.to_owned();

    println!("{:?}", vv);
    println!("{:?}", ss);
    println!("{:?}", v);
    println!("{:?}", s);
}

fn dispose_v(v: &mut Vec<i32>) {
    println!("\nBefore change{:?}", v);
    v[0] = 9;
    let x: &mut Vec<i32> = v;
    x[1] = 78;

    println!("\nAfter change{:?}", v);
}
fn fn_drop() {
    let mut x = vec![1, 2, 3];
    dispose_v(&mut x);
    println!("\nOriginal{:?}", x); // ✅
}

pub fn run() {
    fn_drop();
}
