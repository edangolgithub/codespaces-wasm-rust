pub struct BankAccount {
    pub balance: i32,
    pub verified: bool,
}

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
}

pub fn person_fn() {
    let ram: Person = Person {
        age: 4,
        name: "evam".to_string(),
    };
    println!("{:?}", ram);
}

pub fn run_coffee() {
    let java: Coffee = Coffee {
        name: "java".to_string(),
        price: 45.87,
        is_hot: true,
    };
    let name_owner = &java.name.chars();
    println!("{}", java.name);
    println!("{:?}", name_owner);
    display_java(&java);
}

pub fn display_java(java: &Coffee) {
    println!("{:?}", java.is_hot);
}

#[derive(Debug)]
struct Coffee {
    name: String,
    price: f32,
    is_hot: bool,
}

impl Coffee {
    fn new(name: String, price: f32, is_hot: bool) -> Self {
        Self {
            name,
            price,
            is_hot,
        }
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn display_coffee(self) {
        println!("{:?}", self.name);
        println!("{:?}", self.price);
        println!("{:?}", self.is_hot);
    }
    fn mut_coffee(mut self)
    {
        let name="evu";
        self.name=name.to_string();
        println!("{:?}",self);
    }
}

pub fn demo_borrow_checker(){
    let _my=Coffee{
    name:"name".to_string(),
    price:34.5,
    is_hot:true
   };
 _my.display_coffee();
 //println!("{:?}",_my);
}

pub fn run() {
   let _my=Coffee{
    name:"name".to_string(),
    price:34.5,
    is_hot:true
   };
   _my.mut_coffee();
}
