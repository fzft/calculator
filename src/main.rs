mod calculator;

use std::cell::RefCell;


fn main() {
   let tokens = calculator::Calculator::parse(" 2*(6 + 4)").unwrap();
    println!("{:?}", tokens);
    let expr = calculator::Calculator::expression(tokens);
    println!("{:?}", expr);
    let value = calculator::Calculator::evaluate(expr);
    println!("{:?}", value);
}
