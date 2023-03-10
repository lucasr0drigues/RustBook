fn main() {
    
    loops();
    loop_label();
    whiles();
    fors();
    for_with_reverse_range();
}

fn loops(){

    println!("");
    println!("loop");

    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label(){

    println!("");
    println!("loop label");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        
        let mut remaining = 10;

        loop { 
            println!("Remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn whiles(){

    println!("");
    println!("while");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn fors(){

    println!("");
    println!("for");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_with_reverse_range(){

    println!("");
    println!("for_with_reverse_range");

    for number in (1..4).rev(){
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}