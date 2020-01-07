use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess Number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("#: {}",secret_number);

    // difference btw let mut and let?
    loop{
        println!("Type Number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Cannot read number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,    
        };

        println!("Input: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less =>
                println!("++"),
            Ordering::Greater =>
                println!("--"),
            Ordering::Equal => {
                println!("Equal");
                break;
            },
        }
    }
}
