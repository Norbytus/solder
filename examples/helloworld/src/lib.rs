use solder::*;
use solder::zend::*;
use solder::info::*;
use php_derive::php_function;

#[php_function(hello_world)]
pub fn php_hello_world(first: String, second: String) -> String {
    let first_name = String::try_from(args.get(0).expect("Not found first arg").clone()).ok().unwrap();
    let last_name = String::try_from(args.get(1).expect("Not found first arg").clone()).ok().unwrap();

    format!("Hello {} {}", first_name, last_name)
}

// php_module! {
//     name: "RustPhpModule",
//     version: "0.1",
//     info: "Rust php module",
//     functions: [
//         [
//             "helloWorld",
//             hello_world,
//             args: [
//                 ["firstName", 0, 0, 0],
//                 ["lastName", 0, 0, 0],
//             ]
//         ]
//     ]
// }
