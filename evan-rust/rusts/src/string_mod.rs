pub fn fn_init() {
    let hello_world = "Hello, World!";
    let hello_world: &'static str = "Hello, evan!";
    println!("{}",hello_world);
    let story = "Once upon a time...";
    let ptr = story.as_ptr();
    let len = story.len();
    println!("\npointer val is {:?}",*ptr);
}

pub fn run()
{
    fn_init();
}