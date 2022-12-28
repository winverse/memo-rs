use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Memo {
    title: String,
    text: String,
}

struct Memos {
    inner: HashMap<String, Memo>,
}

impl Memos {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, memo: Memo) {
        self.inner.insert(memo.title.clone(), memo);
    }

    fn get_all(&self) -> Vec<&Memo> {
        let mut memos = vec![];
        for memo in self.inner.values() {
            memos.push(memo);
        }
        memos
    }

    fn get_one(&self, title: &str) -> Option<&Memo> {
        self.inner.get(title)
    }

    fn remove(&mut self, title: &str) -> bool {
        self.inner.remove(title).is_some()
    }

    fn update(&mut self, title: &str, text: &str) -> bool {
        match self.inner.get_mut(title) {
            Some(memo) => {
                memo.text = text.to_owned();
                true
            }
            None => false,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter the your data again")
    }

    let input = buffer.trim().to_owned();

    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn add_memo(memos: &mut Memos) {
    println!("Memo title:");

    let title = match get_input() {
        Some(input) => input,
        None => return,
    };

    println!("Memo text:");

    let text = match get_input() {
        Some(input) => input,
        None => return,
    };

    let memo = Memo { title, text };
    memos.add(memo);
    println!("Memo Added")
}

fn remove_memo(memos: &mut Memos) {
    for memo in memos.get_all() {
        println!("{:?}", memo);
    }

    println!("Enter the memo title to remove:");

    let title = match get_input() {
        Some(input) => input,
        None => return,
    };

    if memos.remove(&title) {
        println!("Removed!");
    } else {
        println!("Not found memo");
    }
}

fn update_memo(memos: &mut Memos) {
    for memo in memos.get_all() {
        println!("{:?}", memo);
    }

    println!("Please enter the title to update:");
    let title = match get_input() {
        Some(input) => input,
        None => return,
    };

    match memos.get_one(&title) {
        Some(_) => (),
        None => {
            println!("Not found memo");
            return;
        }
    }

    println!("Please enter the text");
    let text = match get_input() {
        Some(input) => input,
        None => return,
    };

    if memos.update(&title, &text) {
        println!("Updated!");
    } else {
        println!("Not found memo")
    }
}

fn show_memos(memos: &Memos) {
    for memo in memos.get_all() {
        println!("{:?}", memo);
    }
}

fn display_menu() {
    fn show() {
        println!("");
        println!("== Manage Memos ==");
        println!("1. Add memo");
        println!("2. View memos");
        println!("3. Remove memo");
        println!("4. Update memo");
        println!("q. quit");
        println!("");
        println!("Enter selection:");
    }

    let mut memos = Memos::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return,
        };

        if input == "q" {
            println!("GoodBye");
            return;
        }

        match input.as_str() {
            "1" => add_memo(&mut memos),
            "2" => show_memos(&memos),
            "3" => remove_memo(&mut memos),
            "4" => update_memo(&mut memos),
            "q" => break,
            _ => {
                println!("Invalid command")
            }
        }
    }
}

fn main() {
    display_menu();
}
