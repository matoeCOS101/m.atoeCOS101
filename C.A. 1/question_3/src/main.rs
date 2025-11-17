//Bookshop Order System

use std::io;

fn main() {
    println!(" R - Rust for beginners -  N15 000");
    let _r:i64 = 15_000;
    println!(" A - AI Basics -  N12 500");
    let _a:i64 = 12_500;
    println!(" D - Data Structures in Rust -  N20 000");
    let _d:i64 = 20_000;
    println!(" N - Networking Essentials -  N18 000");
    let _n:i64 = 18_000;

    //input book code
    let mut book_code = String::new();
    println!("Please input book code");
    io::stdin().read_line(&mut book_code).expect("Not a valid string");

    //input book quantity
    let mut book_quantity = String::new();
    println!("Please input quantity of books required");
    io::stdin().read_line(&mut book_quantity).expect("Not a valid string");
    let book_quantity:i64 = book_quantity.trim().parse().expect("Not a valid number");

    if book_code == "R" {

        let _amount = 15_000 * book_quantity;
        if book_quantity > 3 {
            let _amount = (15_000 * book_quantity) - ((10/100) * (15_000 * book_quantity)); 
        }
        println!("Your total cost is {}", _amount);
    }

if book_code == "A" {

        let _amount2 = 12_500 * book_quantity;
        if book_quantity > 3 {
            let _amount2 = (12_500 * book_quantity) - ((10/100) * (12_500 * book_quantity)); 
        }
        println!("Your total cost is {}", _amount2);
    }
if book_code == "D" {

        let _amount3 = 20_000 * book_quantity;
        if book_quantity > 3 {
            let _amount3 = (20_000 * book_quantity) - ((10/100) * (20_000 * book_quantity)); 
        }
        println!("Your total cost is {}", _amount3);
    }
if book_code == "N" {

        let _amount4 = 18_000 * book_quantity;
        if book_quantity > 3 {
            let _amount4 = (18_000 * book_quantity) - ((10/100) * (18_000 * book_quantity)); 
        }
        println!("Your total cost is {}", _amount4);
    }

}
