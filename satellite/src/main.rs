#[derive(Debug)]
struct Satellite {
    id: u64,
}
impl Satellite {
    fn recv(&self, mailbox: &mut MailBox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}
impl MailBox {
    fn new() -> Self {
        Self { messages: vec![] }
    }

    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &Satellite) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Station {}
impl Station {
    fn new() -> Self {
        Self {}
    }

    fn connect(&self, sat_id: u64) -> Satellite {
        Satellite { id: sat_id }
    }

    fn send(&self, mailbox: &mut MailBox, msg: Message) {
        mailbox.post(msg);
    }
}

fn main() {
    let mut mail_box = MailBox::new();

    let station = Station::new();

    let sat_ids: Vec<u64> = vec![1, 2, 3];

    for id in &sat_ids {
        let sat = station.connect(*id);
        let msg = Message {
            to: sat.id,
            content: String::from("hello world"),
        };
        station.send(&mut mail_box, msg);
    }

    for id in &sat_ids {
        let sat = station.connect(*id);
        let msg = sat.recv(&mut mail_box);

        println!("message: {:?}", msg);
    }
}
