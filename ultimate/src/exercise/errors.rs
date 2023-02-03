// 1. Create a DolphinError type representing the following three conditions:
// - Hungry - The dolphin is hungry
// - TooYoung - The dolphin is too young
// - LongName - The dolphin's name is too long and annoying to say
//
// As a reminder, here are the 5 Guidelines for creating an error type:
// (1) Use an `enum` for your error type
// (2) Your error conditions should be enum variants grouped in as few enums as makes sense
// (3) Don't expose error types other than your own (not going to be a problem for this exercise)
// (4) Make your enum non-exhaustive
// (5) Implement the Debug, Display, and Error traits
// (5b) You can use thiserror's `Error` macro to derive the Display and Error traits
//
// Once you have completed defining the error type correctly, you should be able to run
// `cargo build --lib` without any build errors or warnings. Then go to main.rs and continue with #2

use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DolphinError {
    #[error("Dolphin is Hungry")]
    Hungry,
    #[error("The dolphin is too young")]
    TooYoung,
    #[error("The dolphin's name is too long and annoying to say")]
    LongName,
}

pub struct Dolphin {
    pub name: String,
    pub age: u8,
    pub hungry: bool,
}

impl Dolphin {
    pub fn say_your_name(&self) -> Result<String, DolphinError> {
        if self.name.len() > 10 {
            Err(DolphinError::LongName)
        } else {
            Ok(format!("Hi, my name is {} and I'm a Dolphin!", self.name))
        }
    }
    pub fn flip(&self) -> Result<String, DolphinError> {
        if self.age < 4 {
            Err(DolphinError::TooYoung)
        } else {
            Ok(format!("Yippee, I'm doing a flip!"))
        }
    }
    pub fn shake_hands(&self) -> Result<String, DolphinError> {
        if self.hungry {
            Err(DolphinError::Hungry)
        } else {
            Ok(format!("Nice to meet you, let's shake hands!"))
        }
    }
}

fn play_time(dolphin: &Dolphin) -> Result<Vec<String>, DolphinError> {
    let mut responses = vec![];

    // 2b. Call the .say_your_name() method on `dolphin`, use `?` to unwrap the value, and push
    // the value onto the `responses` vector.
    //
    let response = dolphin.say_your_name()?; // this can be done with an intermediate variable...
    responses.push(response); // ...or all on one line. Either way is fine!
                              //
                              // 2c. Do the same thing as #2b for the .flip() method
    responses.push(dolphin.flip()?); // ...or all on one line
    responses.push(dolphin.shake_hands()?); // 2d. Do the same thing as #2b for the .shake_hands() method

    Ok(responses)
}

fn main() {
    let dolphins = vec![
        Dolphin {
            name: "Augustinius".into(),
            age: 7,
            hungry: false,
        },
        Dolphin {
            name: "Bitty".into(),
            age: 2,
            hungry: true,
        },
        Dolphin {
            name: "Carson".into(),
            age: 5,
            hungry: true,
        },
        Dolphin {
            name: "Devin".into(),
            age: 6,
            hungry: false,
        },
    ];
    for dolphin in &dolphins {
        // Challenge: Change main() so that it returns a Result, and instead of handling the error
        // that play_time returns, use the try (?) operator to only handle the success condition.
        //
        // If done correctly, the output of the program will become much shorter. Since play_time
        // returns an Err variant the first time it is called, the try operator will return it from
        // main(), which will end the program at the first error. anyhow's Result will take care of
        // formatting the error output for us.
        match play_time(dolphin) {
            Ok(responses) => {
                println!("{} did a FABULOUS PERFORMANCE!", dolphin.name);
                for response in responses {
                    println!("  {}", response);
                }
            }
            Err(e) => println!("{} can't perform today: {}", dolphin.name, e.to_string()),
        }
    }
}
