#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate regex;
extern crate reqwest;

use std::fs::File;
use std::io::Read;
use serde_json::Value;
use reqwest::Client;
use regex::Regex;
use regex::RegexBuilder;

fn main() {
    let mut json_input = String::new();
    File::open("manifest.json").expect("manifest.json is not in working directory.").read_to_string(&mut json_input).expect("Json parsing failed!");
    if let Value::Object(mut json) = serde_json::from_str(&json_input).expect("Invalid json.") {
        {
            let files = json.get_mut("files").expect("No files key found, check your json.").as_array_mut().expect("Invalid files type. Check your json");
            let client = Client::new();
            for file in files.iter_mut() {
                if let Value::Object(entry) = file {
                    let project_id = entry.get("projectID").expect("missing projectID").as_i64().expect("ProjectID is not a number!");
                    let file_id = entry.get("fileID").expect("missing fileID").as_i64().expect("fileID is not a number!");

                    let mut response = client.get(&format!("https://minecraft.curseforge.com/mc-mods/{}",project_id)).send().unwrap();
                    let response_text = response.text().unwrap();
                    let mod_name = PROJECT_NAME_REGEX.captures(&response_text).expect("Curse returned an unexpected format. This tool no longer works.").name("name").expect("Curse returned an unexpected format. This tool no longer works.").as_str();
                    let project_url = response.url();
                    let file_url = format!("{}/files/{}",project_url,file_id);

                    println!("Project {}:\r\n\tMod name: {}\r\n\tProject URL: {}\r\n\tFile URL: {}", project_id, mod_name, project_url, file_url);


                    entry.insert("project_url".to_string(), serde_json::Value::String(project_url.to_string()));
                    entry.insert("name".to_string(), serde_json::Value::String(mod_name.to_string()));
                    entry.insert("file_url".to_string(), serde_json::Value::String(file_url));

                } else {
                    println!("Unexpected json formatting. Check your json");
                }

            }
        }
        let mut outputfile = File::create("manifest-generated.json").expect("Unable to create output file.");
        serde_json::to_writer_pretty(outputfile, &json).expect("Unable to write json to file");
        println!("Done.");
    }else{
        println!("Unexpected json formatting. Check your json");
    }
}

fn project_name_regex() -> Regex{
    RegexBuilder::new(r#"<h1 class="project-title[\W]+?a href=".+?"\W+span class="overflow-tip">(?P<name>.+?)</span>"#).build().unwrap()
}

lazy_static! {
    static ref PROJECT_NAME_REGEX: Regex = project_name_regex();
}