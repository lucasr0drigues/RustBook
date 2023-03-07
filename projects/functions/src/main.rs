fn main() {
    println!("Hello, world!");

    another_function(6, 'm');

    let x = five();

    println!("The value of x is: {x}");

    let sum = plus_one(x);

    println!("The value of x + 1 is: {sum}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}