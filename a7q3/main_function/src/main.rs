extern crate calculator_lib;
use calculator_lib::cal::sum;
use calculator_lib::cal::minus;
use calculator_lib::cal::multiplication;
use calculator_lib::cal::division;
fn main() {
    let x=10;
    let y=5;

    println!("x = {}, y  {}",x,y);

    let z = sum(x,y);
    println!("Sum = {}",z);

    let z = minus(x,y);
    println!("Minus = {}",z);

    let z = division(x,y);
    println!("Multiplication = {}",z);

    let z = multiplication(x,y);
    println!("Division = {}",z);
}