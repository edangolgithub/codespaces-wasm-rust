pub fn match_fn() {
    //let name: String = String::from("evan");
    let name = "evan";
    match name {
        "evan" => println!("evan"),
        "ram" => println!("ram"),
        "shyam" => println!("shyam"),

        _ => println!("not evan"),

        
    }
}

pub fn match_fn1()
{
    let num=4;
    match num{
        1|2|3|4=>println!("small"),
        5..=10=>println!("medium"),
        56=>println!("correct"),
        _=>println!("large")
    }
}
