use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your energy consumption in kWh:");
    io::stdin().read_line(&mut input).unwrap();

    let usage: f64 = input.trim().parse().unwrap();
    let rate: f64;

    if usage > 200.0 {
        rate = 30.0;
    } else if usage > 100.0 {
        rate = 25.0;
    } else {
        rate = 20.0;
    }

    let total = usage * rate;

    println!("Your total electricity bill is ₦{:.2}", total);
}
