use regex::Regex;
use std::io::BufRead;
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::Path,
};
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    // Create out directory
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::create_dir_all(&out_dir).unwrap();

    // Loop over all files in the list
    BufReader::new(File::open("lenra-api.files.txt").expect("Could not open the file list"))
        .lines()
        .map(|line| line.expect("Could not read line"))
        .filter(|line| !line.starts_with("#"))
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .for_each(|schema| {
            generate_structs(schema.as_str(), out_dir.as_str());
        });
}

fn generate_structs(schema_name: &str, out_dir: &str) {
    println!("Loading schema from {}", schema_name);
    if schema_name.ends_with(".schema.json") {
        json_schema_to_rust(schema_name, out_dir);
    } else if schema_name.ends_with("-api.yml") {
        open_api_to_rust(schema_name, out_dir);
    } else {
        println!("Skipping {}", schema_name);
    }
}

fn json_schema_to_rust(schema_name: &str, out_dir: &str) {
    let schema_path = format!("api/{}", schema_name);
    let content = std::fs::read_to_string(schema_path).unwrap();
    let regex: Regex = Regex::new(r#"(?m),\n\s+"default":.+$"#).unwrap();
    let fixed_content = regex.replace_all(&content, "");
    println!("Removing default values from {}", fixed_content);
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&fixed_content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
    );

    let mut out_file = Path::new(out_dir).to_path_buf();
    out_file.push(format!(
        "{}.rs",
        schema_name
            .replace("/", "_")
            .replace("-", "_")
            .replace(".schema.json", "")
    ));
    fs::write(out_file, contents).unwrap();
}

fn open_api_to_rust(schema_name: &str, out_dir: &str) {
    // let schema_path = PathBuf::from(format!("api/{}", schema_name));
    // let package_name = schema_name
    //     .replace("/", "_")
    //     .replace("-", "_")
    //     .replace(".yml", "");

    // TODO: manage Open API here
}
