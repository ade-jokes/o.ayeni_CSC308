use std::io

fn main() {
	let bill= 12000.0;
	let discount;

	if bill > 10000.0 {
		discount= 0.15;
	} else if bill > 5000.0 {
		discount= 0.10;
	} else {
		discount= 0.0;
	}

	let dis_amo = bill * discount;
	let f_bill = bill - dis_amo;


	println!("Original Bill: ", bill);
	println!("Discount Applied: ", dis_amo);
	println!("Final Bill: ", f_bill);
}