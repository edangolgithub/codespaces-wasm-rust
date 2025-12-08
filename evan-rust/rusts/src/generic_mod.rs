fn identity<T>(value: T) -> T {
    value
}

fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, 56)
}
#[derive(Debug, Clone)]
struct Treasue<T> {
    treasure: T,
    captain: String,
}
impl Treasue<String> {
    fn func1(&mut self) {
        self.captain = String::from("helloworls");
        self.treasure = "world".to_string();
        println!("{:?}", self)
    }
}

pub fn original_copied() {
    let original = Treasue {
        treasure: "ram".to_string(),
        captain: "beu".to_string(),
    };

    // create independent copy
    let mut copied = original.clone(); // new Treasue, separate from original [web:50][web:54]

    copied.func1(); // changes only `copied`

    println!("original: {:?}", original);
    println!("copied:   {:?}", copied);
}

impl Treasue<[&str; 3]> {
    fn func(&mut self) -> (String, usize,String) {
        self.captain = "ssds".to_ascii_lowercase();
        let len = self.treasure.len();
        (self.captain.clone(), len,self.treasure[0].to_lowercase())
    }
}

pub fn run() {
    let mut original = Treasue::<[&str; 3]> {
        treasure: ["A", "B", "C"],
        captain: "beu".to_string(),
    };

    let result = original.func();
    println!("{:?}", result);
    println!("{:?}", original);
}

