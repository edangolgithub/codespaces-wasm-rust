
pub fn owner_int_fn() {
    let applw = 6;
    print_my_int(applw);

    println!("{:?}", applw);
}
pub fn owner_string_fn() {
    let name: String = String::from("evan");
    print_my_string(name.clone());
    println!("{:?}", name);
}
pub fn print_my_string(value: String) {
    println!("{:?}", value);
}

pub fn print_my_int(value: i32) {
    println!("{:?}", value);
}

pub fn push_str_fn() {
    let mut name: String = String::from("evan");
    name.push_str("ram");
    println!("{:?}", name);
    println!("{name}");
}

pub fn create_burger() {
    let burger: String = String::from("burger");   
    add_fries(burger);
}

pub fn add_fries(mut meal: String) {    
    meal.push_str(" add fries");
    println!("{:?}", meal);
}

fn var_scope()
{
    let s = String::from("hello");
}

pub fn run()
{
 var_scope();
}