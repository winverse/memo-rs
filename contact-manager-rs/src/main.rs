use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MssingField(String),
}

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Contact Manager rs")]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "./src/data.csv")]
    data_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    Edit {
        id: i64,
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    List {},
    Remove {
        id: i64,
    },
    Search {
        query: String,
    },
}

struct Records {
    inner: HashMap<i64, Record>,
}
impl Records {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        let record = Record {
            id,
            name: name.to_owned(),
            email: email,
        };
        self.inner.insert(id, record);
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }

    fn into_vec(mut self) -> Vec<Record> {
        let mut records: Vec<_> = self.inner.drain().map(|(_, record)| record).collect();
        records.sort_by_key(|rec| rec.id);
        records
    }

    fn search(&self, name: &str) -> Vec<&Record> {
        self.inner
            .values()
            .filter(|record| record.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    fn remove(&mut self, id: i64) -> Option<Record> {
        self.inner.remove(&id)
    }
}

fn parse_record(record: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = record.split(',').collect();

    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };

    let name = match fields.get(1) {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MssingField("name".to_owned())),
    };

    let email = fields
        .get(2)
        .map(|email| email.to_string())
        .filter(|email| email != "");

    let record = Record { id, name, email };
    Ok(record)
}

fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    for (index, record) in records.split('\n').enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(rec) => recs.add(rec),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}: {}\n  > \"{}\"\n",
                            index + 1,
                            e,
                            record
                        );
                    }
                }
            }
        }
    }
    recs
}

fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_records(buffer, verbose))
}

fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write(b"id,name,email\n")?;

    for record in records.into_vec().into_iter() {
        let email = match record.email {
            Some(email) => email,
            None => "".to_owned(),
        };

        let line = format!("{},{},{}\n", record.id, record.name, email);
        file.write(line.as_bytes())?;
    }
    file.flush()?; // 기록 장치에 저장 되는 것을 기다리고 함수를 결과 값을 반환함
    Ok(())
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?
        }
        Command::Edit { id, name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            recs.edit(id, &name, email);
            save_records(opt.data_file, recs)?;
        }
        Command::List { .. } => {
            let recs = load_records(opt.data_file.clone(), opt.verbose)?;
            for record in recs.into_vec() {
                println!("{:?}", record)
            }
        }
        Command::Remove { id } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            if recs.remove(id).is_some() {
                save_records(opt.data_file, recs)?;
                println!("record deleted");
            } else {
                println!("record not found");
            }
        }
        Command::Search { query } => {
            let recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let result = recs.search(&query);
            if result.is_empty() {
                println!("record not found");
            } else {
                for recs in result {
                    println!("{:?}", recs);
                }
            }
        }
    }
    Ok(())
}
fn main() {
    let opt = Opt::from_args();
    println!("opt {:?}", opt);
    if let Err(e) = run(opt) {
        println!("an error occurred: {}", e);
    }
}
