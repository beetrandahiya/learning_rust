use std::io;


fn main(){

    let mut n= String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    let n:usize = n.trim().parse()
        .expect("Please type a number!");

    let mut table :[[usize;2];10] = [[0;2];10];

    for i in 0..10{
        table[i][0] = i+1;
        table[i][1] = n * table[i][0];
    }

    for i in 0..10{
        println!("{} x {} = {}", n, table[i][0], table[i][1]);
    }

}