use regex::Regex;
use schemars::schema::{RootSchema, Schema, SchemaObject};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{
    env,
    fs::{self, File},
    io::BufReader,
};
use typify::{TypeSpace, TypeSpaceSettings, TypeStruct};

fn main() -> Result<(), String> {
    // Loop over all files in the list
    BufReader::new(File::open("lenra-api.files.txt").expect("Could not open the file list"))
        .lines()
        .map(|line| line.expect("Could not read line"))
        .filter(|line| !line.starts_with("#"))
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .for_each(|schema| {
            generate_structs(schema.as_str());
        });
    Ok(())
}

fn generate_structs(schema_name: &str) {
    println!("Loading schema from {}", schema_name);
    if schema_name.ends_with(".schema.json") {
        json_schema_to_rust(schema_name);
    }
    /* else if schema_name.ends_with("-api.yml") {
        open_api_to_rust(schema_name);
    } */
    else {
        println!("Skipping {}", schema_name);
    }
}

fn json_schema_to_rust(schema_name: &str) {
    let schema_path = format!("api/{}", schema_name);
    let content = std::fs::read_to_string(schema_path).unwrap();
    let regex: Regex = Regex::new(r#"(?m),\n\s+"default":.+$"#).unwrap();
    let content = regex.replace_all(&content, "");
    let mut schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema.clone()).unwrap();

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
    );
    let target_file_name = schema_name
        .replace("-", "_")
        .replace("/", "_")
        .replace(".schema.json", "");
    let out_file = PathBuf::from(format!("lenra-app/src/gen/{}.rs", target_file_name));
    let contents = if schema_name.starts_with("components/") {
        let additionnal_content = if schema_name.ends_with("lenra.schema.json") {
            build_component_functions(&mut type_space, schema)
        } else {
            let root_title = schema
                .schema
                .metadata()
                .title
                .as_ref()
                .clone()
                .unwrap()
                .clone();
            let component_type = type_space
                .iter_types()
                .find(|t| t.name() == root_title)
                .unwrap();
            let component_struct = match component_type.details() {
                typify::TypeDetails::Struct(struct_type) => struct_type,
                _ => unreachable!(),
            };

            build_component_function(&type_space, &component_struct, &mut schema.schema.clone())
        };
        let mut contents = contents;
        contents.push_str("\n\n");
        contents.push_str(&additionnal_content);
        contents
    } else {
        contents
    };
    fs::write(out_file.clone(), contents).unwrap();

    if let Some(rustfmt) = rustfmt_path() {
        let mut cmd = Command::new(rustfmt);
        cmd.arg(out_file)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        if let Err(error) = cmd.output() {
            println!("Error while running rustfmt: {}", error);
        }
    }
}

fn build_component_functions(type_space: &mut TypeSpace, root_schema: RootSchema) -> String {
    let components: Vec<SchemaObject> = root_schema
        .schema
        .subschemas
        .unwrap()
        .one_of
        .unwrap()
        .iter()
        .map(|s| match s {
            schemars::schema::Schema::Bool(_) => panic!("Unexpected bool"),
            schemars::schema::Schema::Object(schema) => {
                let reference = schema.reference.clone().unwrap();
                let reference = reference
                    .split("/")
                    .skip(2)
                    .collect::<Vec<&str>>()
                    .join("/");
                let ref_schema = root_schema
                    .definitions
                    .get(&reference)
                    .expect(format!("Could not find definition for {}", reference).as_str());

                match ref_schema {
                    Schema::Bool(_) => unreachable!("Unexpected bool"),
                    Schema::Object(schema) => schema.clone(),
                }
            }
        })
        .collect();
    let components_titles: Vec<String> = components
        .iter()
        .map(|schema| schema.clone().metadata().title.as_ref().unwrap().clone())
        .collect();
    type_space
        .iter_types()
        .filter(|t| components_titles.contains(&t.name()))
        .map(|el| {
            let schema = components
                .iter()
                .find(|&schema| {
                    el.name() == schema.clone().metadata().title.as_ref().unwrap().clone()
                })
                .expect("Schema not found");
            match el.details() {
                typify::TypeDetails::Struct(struct_type) => {
                    build_component_function(type_space, &struct_type, &mut schema.clone())
                }
                _ => unreachable!(),
            }
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn build_component_function(
    type_space: &TypeSpace,
    struct_type: &TypeStruct,
    schema: &mut SchemaObject,
) -> String {
    let title = schema
        .metadata
        .as_ref()
        .unwrap()
        .title
        .as_ref()
        .unwrap()
        .clone();

    let lower_case_title = title.to_lowercase();
    let mut params_builder = String::new();
    let mut instance_builder = String::new();
    let mut type_builder = String::new();
    let mut where_builder = String::new();
    let schema_object = schema.object.as_ref().unwrap();

    let required_props: Vec<String> = schema_object
        .required
        .iter()
        .filter(|prop| prop.clone() != "_type")
        .map(normalize_prop_name)
        .collect();

    let mut type_counter = 0;

    struct_type
        .properties()
        .filter(|(name, _)| {
            required_props
                .clone()
                .contains(&normalize_prop_name(&name.to_string()))
        })
        .for_each(|(name, type_id)| {
            let pos = type_counter;
            type_counter += 1;
            let type_name = type_space.get_type(&type_id).unwrap().name();
            if pos == 0 {
                type_builder.push_str("<");
                where_builder.push_str(" \nwhere\n");
            } else {
                type_builder.push_str(", ");
                params_builder.push_str(", ");
            }
            type_builder.push_str(format!("T{pos}").as_str());
            where_builder.push_str(format!("    T{pos}: std::convert::TryInto<{type_name}>,\n    T{pos}::Error: std::fmt::Display,\n").as_str());
            params_builder.push_str(format!("{}: T{pos}", name).as_str());
            instance_builder.push_str(format!("\n        .{}({})", name, name).as_str());
        });
    if type_counter > 0 {
        type_builder.push_str(">");
    }
    format!(
        r#"pub fn {lower_case_title}{type_builder}({params_builder}) -> builder::{title}{where_builder} {{
    {title}::builder()
        .type_("{lower_case_title}"){instance_builder}
}}"#
    )
}

fn normalize_prop_name(name: &String) -> String {
    name.to_lowercase().replace("_", "")
}

fn rustfmt_path() -> Option<PathBuf> {
    if let Ok(rustfmt) = env::var("RUSTFMT") {
        return Some(rustfmt.into());
    }
    #[cfg(feature = "which-rustfmt")]
    match which::which("rustfmt") {
        Ok(p) => Some(p),
        Err(e) => None,
    }
    #[cfg(not(feature = "which-rustfmt"))]
    None
}
