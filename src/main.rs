use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut kvstore: HashMap<String, String> = HashMap::new();
    println!("Start Rust KeyValueStore.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        // 文字列スライスのVec
        let commands:Vec<&str> = line.trim().split(" ").collect();

        let command: &str = commands.get(0).unwrap();

        match command {
            "store" => {
                let key: String = commands.get(1).unwrap().to_string();
                let val: String = commands.get(2).unwrap().to_string();
                kvstore.insert(key, val);
                println!("Stored data.");
            }
            "get" => {
                let key: String = commands.get(1).unwrap().to_string();
                let val: String = kvstore.get(&key).unwrap().to_string();
                println!("Getted value: {}", &val);
            }
            "quit" | "exit" => {
                println!("Bye!");
                return;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
