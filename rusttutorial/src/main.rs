fn main() {
    println!("Hello, world!");
    let i32num: i16 = -255; //creates new var as signed 16-bit int
    let unsignedarch: usize = 4_000_000_000; //creates new unsigned architecture-max int //can use _'s as seperators
    let float8: f32 = 3.5; //creates new single-precision float
    let tru: bool = true; //creates new boolean with value true
    let aint = 5; //creates new int (compiler can often determine type at compile-time
    println!("i32num is {} and unsignedarch is {}", i32num, unsignedarch);
    print!("A float is {}", float8); //as you can see println adds a newline after the printed string
    println!("a bool is {}", tru);
    println!("aint is {}", aint); //print's and println's act in a similar way to how they do in java
}

fn guessgame() { //new function called guessgame (from the tutorial)
    //code
}
