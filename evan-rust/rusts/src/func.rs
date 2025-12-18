use crate::estruct::BankAccount;
fn firstfn() {
    println!("Hello, world!");
    let name = "evan";
    println!("my name is {}", name);
    add(3, 5);
}
fn add(a: i32, b: i32) {
    let c = a + b;
    println!("{}", c);
}

fn learnvector() {
    let mut vector_numbers = vec![1, 2, 3, 4, 5, 6];
    vector_numbers.push(7);
    println!("{:?}", vector_numbers);

    let mut second_vector = Vec::new();
    second_vector.push(5);
    println!("{:?}", second_vector);
}
// struct BankAccount
// {
//     balance:i32,
//     verified:bool
// }
pub fn run_account() {
    let my_account = BankAccount {
        balance: 24,
        verified: true,
    };
    print_account(&my_account);
    print_afain(&my_account);
}

fn print_account(account: &BankAccount) {
    println!("{:?}", account.balance);
    println!("{:?}", account.verified);
}

fn print_afain(account: &BankAccount) {
    println!("{:?}", account.balance);
    println!("{:?}", account.verified);
}

fn tuple_test() -> (String, i32, i32, i32, bool) {
    let tp = ("abc".to_owned(), 1, 1, 3, true);
    tp
}

fn fn_tup() {
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

     let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
pub fn run() {
    let x = true;
    let heart_eyed_cat = '😻';
}
