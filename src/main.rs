use std::env;
use std::path::Path;

use serde_json::Value;

fn main() {
    let mut data = String::new();

    for line in std::io::stdin().lines() {
        data.push_str(&line.expect("Could not read line"));
    }

    let value: Value = serde_json::from_str(&data).expect("Could not parse JSON");
    let path = env::args().nth(1).expect("Path not supplied");
    let path = Path::new(&path);

    match leaktk_jf::write_files(&path, &value) {
        Err(err) => println!("{}", err),
        Ok(_) => println!("Files written to {}", path.display()),
    }
}
