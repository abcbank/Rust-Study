fn main() {

//    println!("Hello, world!");
    let_immutable();
    let_mutable();
    let_shadowing();
}

fn let_immutable(){
    println!("Example for immutable variable");
    let x = 5;
    println!("x: {}", x);
//  x = 6;
//  println!("x: {}", x);
//  let => immutable variable
}

fn let_mutable(){
    println!("Example for mutable variable");
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);
//  let mut => mutable variable    
}

fn let_shadowing(){
    println!("Example for shadowing variable");
    let x = 5;
    println!("x: {}", x);
    let x = x + 1;
    println!("x: {}", x);
    let x = x * 2;
    println!("x: {}", x);
}