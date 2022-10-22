

fn main() {
    println!("Type in text to translate:");
    let mut input_text = String::new();
    match std::io::stdin().read_line(&mut input_text) {
        Ok(_) => (),
        Err(e) => println!("error: {e}"),
    };
    println!("input_text is: {}", input_text);
}
