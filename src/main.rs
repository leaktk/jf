use std::env;
use std::fs;
use std::io;
use std::path::Path;

use serde_json::Value;

fn write_files(path: &Path, value: &Value) {
    match value {
        Value::Array(array) => {
            for (i, value) in array.iter().enumerate() {
                write_files(path.join(i.to_string()).as_path(), value);
            }
        },
        Value::Object(map) => {
            for (key, value) in map.iter() {
                if key.contains("..") {
                    panic!("Potential file tree traversal");
                }

                write_files(&path.join(key).as_path(), value);
            }
        },
        _ => {
            let parent = path.parent().expect("Could not get parent dir");

            if !parent.is_dir()  {
                fs::create_dir_all(parent).expect("Could not create parent dir");
            }
            fs::write(path, value.to_string()).expect("Could not write value");
        }
    }
}

fn main() {
    let mut data = String::new();

    for line in io::stdin().lines() {
        data.push_str(&line.expect("Could not read line"));
    }

    let value: Value = serde_json::from_str(&data).expect("Could not parse JSON");
    write_files(&Path::new(&env::args().nth(1).expect("Path not supplied")), &value);
}
