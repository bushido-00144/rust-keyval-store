use std::io;
use std::io::Write;
use std::collections::HashMap;

type DataValue = String;

fn main() {
    let mut kvstore: HashMap<String, DataValue> = HashMap::new();
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
                if commands.len() != 3 {
                    println!("Use > store <key> <value>");
                    continue;
                }
                let key: String = commands.get(1).unwrap().to_string();
                let val: DataValue = commands.get(2).unwrap().to_string();
                kvstore.insert(key, val);
                println!("Stored data.");
            }
            "get" => {
                if commands.len() != 2 {
                    println!("Use > get <key>");
                    continue;
                }
                let key: String = commands.get(1).unwrap().to_string();
                if kvstore.get(&key).is_none() {
                    println!("Unstored: {}", &key);
                    continue;
                }
                let val: DataValue = kvstore.get(&key).unwrap().to_string();
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
