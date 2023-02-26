use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("failed read File, {0}")]
    FailedReadFile(#[from] std::io::Error),
    #[error("Empty field {0}")]
    EmptyField(String),
    #[error("Parse Int error {0}")]+++
    ParseIntError(#[from] std::num::ParseIntError),
}

#[warn(dead_code)]
struct Record {
    id: i64,
    name: String,
    email: String,
}
impl Record {}

fn read_file() -> std::io::Result<String> {
    let mut file = File::open("./src/data.csv")?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn parse_line(fields: &str) -> Result<Record, ParseError> {
    let field: Vec<&str> = fields.split(",").collect();

    let id = match field.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyField("id".to_owned())),
    };

    let name = match field.get(1) {
        Some(name) => name.to_string(),
        None => return Err(ParseError::EmptyField("name".to_owned())),
    };

    let email = match field.get(2) {
        Some(email) => email.to_string(),
        None => return Err(ParseError::EmptyField("email".to_owned())),
    };

    let record = Record { id, name, email };

    Ok(record)
}

fn main() {
    let records = read_file().unwrap();

    let fields: Vec<_> = records
        .split("\n")
        .skip(1)
        .map(|field| field.trim())
        .collect();

    for field in fields {
        let record: Record = parse_line(field).unwrap();
        print!("{} ", record.id);
        print!("{} ", record.name);
        print!("{}\n", record.email);
    }
}
