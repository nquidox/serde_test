use serde_json;
use std::fs;
use std::fs::File;
use serde_json::json;


struct Human{
    name: String,
    surname: String
}


fn read_json_from_file(){
    let path = "content/info.json";
    let data = fs::read_to_string(path).expect("Unable to read info.json");

    // don't forget to give a type to next var
    let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");
    println!("First guy's name is {} {}", res[0]["name"], res[0]["surname"]);
}


fn write_to_file() {
    // info to write to json
    let person = ("Victor", "Tsoy");

    // write part
    let path = "content/new.json";
    let mut file = File::create(path).expect("Unable to create file");
    let res = json!({"name": person.0, "surname": person.1});
    serde_json::to_writer(file, &res);
}


fn main() {
    println!("JSON serde test");
    let guy1 = Human{name: "Phil".to_string(), surname: "Collins".to_string()};
    println!("Test name: {} {}", guy1.name, guy1.surname);

    // func calls
    read_json_from_file();
    write_to_file();
}
