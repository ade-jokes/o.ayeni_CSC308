fn main() {
    let s1 = String::from("Hi");
    let s2 = String::from("amazing!");

    let result = longest(&s1, &s2);
    println!("The longer string is: {}", result);
}

// function borrows both and returns a reference
fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
