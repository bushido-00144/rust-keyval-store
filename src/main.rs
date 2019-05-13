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

    fn store_data(&mut self, key: String, val: DataValue) -> Result<String, String> {
        if self.store.contains_key(&key) {
            return Err("Already exist key".to_string());
        }
        self.store.insert(key, val);
        Ok("Stored data".to_string())
    }

    fn get_data(&self, key: &String) -> Option<&DataValue> {
        self.store.get(key)
    }

    fn update_data(&mut self, key: String, val: DataValue) -> Result<String, String> {
        if !self.store.contains_key(&key) {
            return Err("Not found key".to_string());
        }
        self.store.insert(key, val);
        Ok("Updated data".to_string())
    }

    fn delete_data(&mut self, key: &String) -> Option<DataValue> {
        self.store.remove(key)
    }
}

fn store_from_cli(kvstore: &mut DataStore, commands: Vec<&str>) -> String {
    if commands.len() != 3 {
        return "Use > store <key> <value>".to_string();
    }
    let key: String = commands.get(1).unwrap().to_string();
    let val: DataValue = DataValue::new(commands.get(2).unwrap().to_string());
    match kvstore.store_data(key, val) {
        Ok(message) => message,
        Err(err) => err,
    }
}

fn get_from_cli(kvstore: &DataStore, commands: Vec<&str>) -> String {
    if commands.len() != 2 {
        return "Use > get <key>".to_string();
    }
    let key: String = commands.get(1).unwrap().to_string();
    let val: Option<&DataValue> = kvstore.get_data(&key);
    if val.is_none() {
        return "Unstored ".to_string() + &key;
    }
    let val: String = val.unwrap().value.to_string();
    return "Getted value: ".to_string() + &val;
}

fn update_from_cli(kvstore: &mut DataStore, commands: Vec<&str>) -> String {
    if commands.len() != 3 {
        return "Use > update <key> <value>".to_string();
    }
    let key: String = commands.get(1).unwrap().to_string();
    let val: DataValue = DataValue::new(commands.get(2).unwrap().to_string());
    match kvstore.update_data(key, val) {
        Ok(message) => message,
        Err(err) => err,
    }
}

fn delete_from_cli(kvstore: &mut DataStore, commands: Vec<&str>) -> String {
    if commands.len() != 2 {
        return "Use > delete <key>".to_string();
    }
    let key: String = commands.get(1).unwrap().to_string();
    let val: Option<DataValue> = kvstore.delete_data(&key);
    if val.is_none() {
        return "Unstored ".to_string() + &key;
    }
    return "Deleted data".to_string();
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
                println!("{}", store_from_cli(&mut kvstore, commands));
            }
            "get" => {
                println!("{}", get_from_cli(&kvstore, commands));
            }
            "update" => {
                println!("{}", update_from_cli(&mut kvstore, commands));
            }
            "delete" => {
                println!("{}", delete_from_cli(&mut kvstore, commands));
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

#[test]
fn cli_insret_test() {
    let mut kvstore: DataStore = DataStore::new();
    let commands: Vec<&str> ="store testkey testdata".split(" ").collect();
    let result: String = store_from_cli(&mut kvstore, commands);
    assert_eq!(result, "Stored data")
}

#[test]
fn cli_get_test() {
    let mut kvstore: DataStore = DataStore::new();
    kvstore.store.insert("testkey".to_string(), DataValue::new("testdata".to_string()));
    let commands: Vec<&str> ="get testkey".split(" ").collect();
    let result: String = get_from_cli(&mut kvstore, commands);
    assert_eq!(result, "Getted value: testdata")
}

#[test]
fn cli_update_test() {
    let mut kvstore: DataStore = DataStore::new();
    kvstore.store.insert("testkey".to_string(), DataValue::new("testdata".to_string()));
    let commands: Vec<&str> = "update testkey newdata".split(" ").collect();
    let result: String = update_from_cli(&mut kvstore, commands);
    assert_eq!(result, "Updated data")
}

#[test]
fn cli_delete_test() {
    let mut kvstore: DataStore = DataStore::new();
    kvstore.store.insert("testkey".to_string(), DataValue::new("testdata".to_string()));
    let commands: Vec<&str> = "delete testkey".split(" ").collect();
    let result: String = delete_from_cli(&mut kvstore, commands);
    assert_eq!(result, "Deleted data")
}
