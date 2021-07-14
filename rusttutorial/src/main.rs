use std::io; //use io from the standard libraries

fn main() {
    loop{
        println!("\n\nPlease input a corresponding number: ");
        println!("0: Quit\n1: Run Test\n2: Guessing game\n");

        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read input"); //sets reference to selection to inputted value
        match selection.trim().parse::<u8>() { //matches selection (with removed whitespaces and made into u8 int) with below
            Ok(0) => break, //quits
            Ok(1) => testing(), //runs testing
            Ok(2) => guessgame(),
            Ok(_) => continue, //if other val re-does loop
            Err(_) => continue, //same for err
        };
    }
}

fn testing() {
    println!("Hello, world!");
    let i32num: i16 = -255; //creates new var as signed 16-bit int
    let unsignedarch: usize = 4_000_000_000; //creates new unsigned architecture-max int //can use _'s as separator
    let float8: f32 = 3.5; //creates new single-precision float
    let tru: bool = true; //creates new boolean with value true
    let aint = 5; //creates new int (compiler can often determine type at compile-time
    let mut mutable = 8; //variables are not mutable by default (adding mut changes that)
    mutable = mutable + 1;
    println!("i32num is {} and unsignedarch is {}", i32num, unsignedarch);
    print!("A float is {}", float8); //as you can see println adds a newline after the printed string
    println!("a bool is {}", tru);
    println!("aint is {}", aint); //print's and println's act in a similar way to how they do in java
    println!("a mutable var was 8, but now is: {}", mutable);
}

fn guessgame() { //new function called guessgame (from the tutorial)
    //code
}
