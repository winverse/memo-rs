use rand::prelude::*;

fn throw_error(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    data: Vec<u8>,
}
impl File {
    fn new() -> Self {
        Self {
            name: String::from(""),
            data: vec![],
        }
    }

    fn new_with_data(&mut self, name: &str, data: &Vec<u8>) -> &Self {
        self.name = String::from(name);
        self.data = data.clone();
        self
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut temp = self.data.clone();
        let read_length = self.data.len();
        save_to.reserve(read_length);
        save_to.append(&mut temp);
        Ok(read_length)
    }

    fn open(&self) -> Result<&File, String> {
        match throw_error(10_000) {
            true => return Err(String::from("Failed to open the file")),
            false => return Ok(&self),
        }
    }
    fn close(&self) -> Result<&File, String> {
        match throw_error(100_000) {
            true => return Err(String::from("Failed to closed the file")),
            false => return Ok(self),
        }
    }
}

fn main() {
    let mut new_file = File::new();
    let file_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let file_name = String::from("file4.txt");
    let file = new_file.new_with_data(&file_name, &file_data);

    let mut buffer: Vec<u8> = vec![];

    file.open().unwrap();
    let file_length = file.read(&mut buffer).unwrap();
    file.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, file_length);
    println!("{:?}", text);
}
