use clap::Parser;
use std::fs;

// CLI Args
#[derive(Parser, Debug)]
#[command(version,author, about)]
struct Args {
    /// A JSON file
    #[arg(short, long, value_name = "FILEPATH")]
    file: String,

    #[arg(long, default_value_t = false)]
    to_yaml: bool
}

fn main() {
    let args = Args::parse();
    let file_path = args.file;
    let file_str_result = fs::read_to_string(&file_path);
    match file_str_result {
        Ok(file_str) => {
            let json_value_result = serde_json::from_str::<serde_json::Value>(&file_str);
            match json_value_result {
                Ok(json_value) => {
                    if args.to_yaml {
                        // convert to yaml
                        let yaml_value = serde_yaml::from_str::<serde_yaml::Value>(&json_value.to_string()).unwrap();
                        println!("{}", serde_yaml::to_string(&yaml_value).unwrap());
                    } else {
                        // pretty print the json
                        println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
                    }
                },
                Err(e) => {
                    println!("Not a valid JSON file at {file_path}: {}", e.to_string());
                }
            }
        },
        Err(e) => {
            println!("Unable to read file at {file_path}: {}", e.to_string());
        }
    }
}
