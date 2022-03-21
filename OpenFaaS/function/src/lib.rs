extern crate rustc_serialize;
use rustc_serialize::json::Json;

// req = {"info":2}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Result  {
    success: bool,
    output: String,
    message: String,
}

pub fn handle(req : String) -> String {
    // parse json from string
    let _json =  match Json::from_str(&req){
        Ok(ok) => ok,
        Err(_) => {
            return String::from("Error");
        }
    };
    // do stuff
    // return
    return String::from("Success");
}

/*
let object = TestStruct {
    data_int: 1,
    data_str: "homura".to_string(),
    data_vector: vec![2,3,4,5],
};
// Serialize using `json::encode`
let encoded = json::encode(&object).unwrap();

// Deserialize using `json::decode`
let decoded: TestStruct = json::decode(&encoded).unwrap();
*/