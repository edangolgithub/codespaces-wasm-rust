use crate::estruct::BankAccount;
fn firstfn()
{
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

    let mut second_vector=Vec::new();
    second_vector.push(5);
    println!("{:?}",second_vector);

}
// struct BankAccount
// {
//     balance:i32,
//     verified:bool
// }
pub fn run_account()
{
    let my_account= BankAccount{
        balance:24,
        verified:true
    };
    print_account(&my_account);
    print_afain(&my_account);
}

fn print_account(account: &BankAccount)
{
    println!("{:?}",account.balance);
    println!("{:?}",account.verified);


}

fn print_afain(account: &BankAccount)
{
    println!("{:?}",account.balance);
    println!("{:?}",account.verified);


}
