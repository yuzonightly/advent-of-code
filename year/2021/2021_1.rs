use std::env;
use std::fs;
// use std::any::type_name

fn main() {
    // let input = fs::read_to_string("./year/2021/2021_1.input").expect("fs::read_to_string error.");
    let input = fs::read_to_string("./year/2021/input/2021_1.input").expect("Error reading file.");

    // println!("{}\n", input);
    // match std::fs::read("./year/2021/2021_1.input") {
    //     Ok(bytes) => {
    //         println!("{:?}\n", &bytes[..]);
    //         // let split: Vec<u32> = bytes.to
    //     }
    //     Err(e) => {
    //         if e.kind() == std::io::ErrorKind::PermissionDenied {
    //             eprintln!("please run again with appropriate permissions.");
    //             return;
    //         }
    //         panic!("{}", e);
    //     }
    // }
}
