use std::io;

fn main(){

	let mut input = String::new();

	println!("Enter your energy consumption in kWh");
	io::stdin().read_line(&mut input).unwrap();

	let usage: f64= input.trim().parse().unwrap();
	let rate: f64;
	
}