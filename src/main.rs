use std::env;
mod pos;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("input argument is: {}", args[1])
    } else {
        println!("Type in text to translate:");
        let mut input_text = String::new();
        match std::io::stdin().read_line(&mut input_text) {
            Ok(_) => (),
            Err(e) => println!("error: {e}"),
        };

        let mut origin = pos::Poi::origin();
        pos::Poi::new(
            String::from(input_text.trim()),
            42.0,
            10.0,
            &mut origin,
        );

        let json_str = match serde_json::to_string_pretty(&origin) {
            Ok(json_str) => json_str,
            Err(e) => {
                println!("serialiseError: {e}");
                return ()
            }
        };

        let path_name = String::from("test_target.json");
        match std::fs::write(path_name, json_str) {
            Ok(_) => (),
            Err(e) => {
                println!("writeError: {e}");
                return ()
            }
        };
    }
}
