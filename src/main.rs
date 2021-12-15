use serde_json;
use std::fs;
use std::fs::File;
use serde_json::{json, Value};


fn read_json_from_file(){
    let path = "content/info.json";
    let data = fs::read_to_string(path).expect("Unable to read info.json");

    // don't forget to give a type to next var
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("First guy's name is {} {}", res[0]["name"], res[0]["surname"]);
}


fn write_to_file(record: Vec<Value>) {
    let path = "content/new.json";
    let mut file = File::create(path).expect("Unable to create file");
    serde_json::to_writer(file, &record);
}


fn main() {
    println!("JSON serde test");
    let mut guys = vec!();

    let guy2 = ("Mark", "Twain");
    let res2 = json!({"name": guy2.0, "surname": guy2.1});
    guys.push(res2);

    let guy3 = ("Tom", "Sawyer");
    let res3 = json!({"name": guy3.0, "surname": guy3.1});
    guys.push(res3);

    read_json_from_file();
    write_to_file(guys);

}
