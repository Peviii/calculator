mod calculator;
/**não esquece de subir no git */
fn main() {
    let tokens = calculator::Calculator::parse("2 * 2 + 48 / 4");
    println!("{:?}", tokens);
    let expr = calculator::Calculator::expressions(tokens.unwrap());
    println!("{:?}", expr);
    let value = calculator::Calculator::evaluate(expr);
    println!("{:?}", value.unwrap());
}
