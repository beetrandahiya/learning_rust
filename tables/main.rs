use std::io;


fn main(){

    let mut n= 4;
    println!("Enter the number");
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    println!("{}", n);
}