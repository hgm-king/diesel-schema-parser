use crate::{utils, Column, Table};
/**
 * generator.rs
 *
 * holds the code to receive a parsed schema file,
 * then generate model files based on a template
**/
use std::fmt;

#[derive(Debug, Clone)]
pub struct GenerateError;

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse the schema provided")
    }
}

type Result<T> = std::result::Result<T, GenerateError>;

// this is where we convert diesel data types into rust data types,
// atm it is not exhaustive
fn convert_datatype_to_rust(data_type: &str) -> &str {
    match data_type {
        "Text" => "String",
        "Int8" => "i64",
        "Int4" => "i32",
        "Float8" => "f64",
        "Float4" => "f32",
        "Boolean" => "bool",
        _ => panic!("We have not covered this data type"),
    }
}

// convert a database column from schema into a rust data structure element
fn generate_column(column: Column) -> String {
    match column {
        (name, (data_type, false)) => format!(
            "\n    pub {}: {},",
            name,
            convert_datatype_to_rust(&data_type)
        ),
        (name, (data_type, true)) => format!(
            "\n    pub {}: Option<{}>,",
            name,
            convert_datatype_to_rust(&data_type)
        ),
    }
}

// given a template file, a parsed schema of tables, and a folder path
// fill in the template for each table and save the model file to the folder
pub fn generate_models(template: &str, tables: &[Table], folder_path: &str) -> Result<usize> {
    let mut count = 0;
    println!("Generating Models");

    for table in tables.iter() {
        let mut template_copy = template.to_string();

        let name = table.name.to_owned();
        let columns = table.columns.to_owned();
        let _key = table.primary_key.to_owned();

        let save_path = format!("{}/{}-model.rs", folder_path, name);

        let mut column_string = String::new();

        for column in columns.into_iter() {
            column_string = format!("{}{}", column_string, generate_column(column));
        }

        template_copy = template_copy.replace("{table_name}", &name);
        template_copy = template_copy.replace("{columns}", &column_string);

        println!("-- {}", save_path);
        utils::save_file(save_path, &template_copy);

        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_columns() {
        assert_eq!(
            generate_column((String::from("index"), (String::from("Text"), false))),
            String::from("\n    pub index: String,")
        );
        assert_eq!(
            generate_column((String::from("index"), (String::from("Text"), true))),
            String::from("\n    pub index: Option<String>,")
        );
    }
}
