pub fn slice_example() {
    let data = String::from("hello world");
    let slice = &data[4..9];
    let oneletter = slice.chars().nth(0).unwrap_or('3');

    println!("{:?}", slice.chars().nth(0).unwrap_or('3'));

    let mov = &data[1..4];
    print!("{:?}", data)
}

pub fn slice_example1() {
    let mut nums = [1, 2, 3, 4, 5];
    let last_two = &mut nums[3..5];
    last_two[1] = 56;
    println!("{:?}", last_two);
}

pub fn run() {
    slice_example1();
}
