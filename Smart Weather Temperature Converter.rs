use std::io;

fn main() {
	println!("Smart weather Temperature Converter");

	println!("We are going to convert weathers from C TO F and vise versa");

	let celc= 36.5;
	let feren= 100.0;

	let to_fere= (celc * 9.0/5.0) + 32.0;
	let to_clesi= (feren - 32.0) * (5.0/9.0);

	println!("Temperature: ", celc, "°C" );
	println!("Converted: ", to_fere, "°F");

	println!("Temperature", feren, "°F" );
	println!("Converted", to_clesi, "°C");

}