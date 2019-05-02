use std::io;
use std::io::Write;
use std::collections::HashMap;

struct DataValue {
    value: String
}

impl DataValue {
    fn new(val: String) -> DataValue {
        DataValue{value: val}
    }
}

struct DataStore {
    store: HashMap<String, DataValue>
}

impl DataStore {
    fn new() -> DataStore {
        DataStore{store: HashMap::new()}
    }

    fn store_data(&mut self, key: String, val: DataValue) {
        self.store.insert(key, val);
    }

    fn get_data(&self, key: &String) -> Option<&DataValue> {
        self.store.get(key)
    }
}

fn main() {
    let mut kvstore: DataStore = DataStore::new();
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
                let val: DataValue = DataValue::new(commands.get(2).unwrap().to_string());
                kvstore.store_data(key, val);
            }
            "get" => {
                if commands.len() != 2 {
                    println!("Use > get <key>");
                    continue;
                }
                let key: String = commands.get(1).unwrap().to_string();
                let val: Option<&DataValue> = kvstore.get_data(&key);
                if val.is_none() {
                    println!("Unstored {}", key.to_string());
                    continue;
                }
                let val: String = val.unwrap().value.to_string();
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
