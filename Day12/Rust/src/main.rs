extern crate rustc_serialize;

use rustc_serialize::json;

fn sum(o: &json::Json) -> i32 {
    match o {
        &json::Json::Object(ref o) => {
            let mut t = 0;

            for (k, v) in o {
                match v {
                    &json::Json::String(ref s) if s == "red" => return 0,

                    _ => t += sum(v),
                }
            }

            t
        }

        &json::Json::Array(ref a) => {
            let mut t = 0;

            for v in a {
                t += sum(v);
            }

            t
        }

        &json::Json::I64(n) => n as i32,
        &json::Json::U64(n) => n as i32,
        &json::Json::F64(n) => n as i32,

        _ => 0,
    }
}

fn main() {
    let r = json::Json::from_reader(&mut std::io::stdin()).expect("bad input");

    println!("Sum: {:?}", sum(&r));
}
