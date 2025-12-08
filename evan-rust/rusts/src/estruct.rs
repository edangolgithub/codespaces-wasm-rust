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

fn display_java(java: &Coffee) {
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
    fn mut_coffee(mut self) {
        let name = "evu";
        self.name = name.to_string();
        println!("{:?}", self);
    }
    fn display_coffee1(&self) {
        println!("{:?}", self.name);
        println!("{:?}", self.price);
        println!("{:?}", self.is_hot);
    }

    fn is_hotter(self: &Self, other: &Self) -> bool {
        if self.price > other.price {
            return true;
        }
        false
    }
}

pub fn demo_borrow_checker() {
    let _my = Coffee {
        name: "name".to_string(),
        price: 34.5,
        is_hot: true,
    };
    _my.display_coffee();
    //println!("{:?}",_my);
}

pub fn example1() {
    let _my = Coffee {
        name: "name".to_string(),
        price: 34.5,
        is_hot: true,
    };
    let _my1 = &_my;
    println!("{:?}", _my1);
    _my.mut_coffee();
}
pub fn example2() {
    let _my = Coffee {
        name: "name".to_string(),
        price: 34.15,
        is_hot: true,
    };
    _my.display_coffee1();
    println!("{:?}", _my);
}

pub fn coffee_checker() {
    let one = Coffee {
        name: "one coffee".to_string(),
        price: 34.15,
        is_hot: true,
    };
    let two = Coffee {
        name: "two coffee".to_string(),
        price: 35.15,
        is_hot: true,
    };
    let cool_one = one.is_hotter(&two);
    println!("{}", cool_one);
}

pub fn run() {
      let two = Coffee {
        name: "two coffee".to_string(),
        price: 35.15,
        is_hot: true,
    };
   display_java(&two);
}
