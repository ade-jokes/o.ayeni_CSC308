fn main() {
    let mut name = String::from("Ada");
    
    print_name(&name);         // immutable borrow
    append_title(&mut name);   // mutable borrow
    
    println!("Final name: {}", name);
}

fn print_name(n: &String) {
    println!("Name: {}", n);
}

fn append_title(n: &mut String) {
    n.push_str(" Lovelace");
}
