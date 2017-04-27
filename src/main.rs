fn main() {
    let x = 5;
    let (x, y) = (1, 2);
    let x: i32 = 5; // signed integer 32 bites

    // by default bidings are immutable
    // let y = 5;
    // y = 10;
    // ^^^^^^ re-assignment of immutable variable

    let mut z = 5;
    z = 10;
    // OK

    // let a: i32;
    // println!("The value of a is {}", a);

    // scope

    let b: i32 = 17;
    {
        let c: i32 = 5;
        println!("The value of x is {} and value of y is {}", b, c); // OK
    }
    // println!("The value of x is {} and value of y is {}", b, c); // not OK


    let mut x: i32 = 13;
    x = 7;
    let x = x;

    let y = 4;
    let y = "I can also be bound to text!";
}
