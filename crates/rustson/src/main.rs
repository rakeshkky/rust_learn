use serde::*;

// User type
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: i32
}

fn main() {
    let json_str = "[{\"name\":\"Rakesh\",\"age\":26},{\"name\":\"Ammu\",\"age\":22}]";
    let users_result = serde_json::from_str::<Vec<User>>(&json_str);
    match users_result {
        Ok(users) => {
            let json_result = serde_json::to_string(&users);
            match json_result {
                Ok(json_str) => {
                    println!("{}", json_str);
                },
                Err(e) => {
                    println!("Error occurred while encoding to json string - {}", e.to_string());
                }
            }
        },
        Err(e) => {
            println!("Error occurred while parsing json string - {}", e.to_string());
        }
    }
}
