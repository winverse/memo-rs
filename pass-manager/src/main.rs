use std::error::Error;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::{event, execute};
use crossterm::event::Event::Key;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::backend::{Backend, CrosstermBackend};
use tui::{Frame, Terminal};
use tui::widgets::ListState;

enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    Submit,
    Search,
    List,
    Delete
}

struct Password {
    title: String,
    username: String,
    password: String
}

struct PassManager {
    mode: InputMode,
    list_state: ListState,
    passwords: Vec<Password>,
    search_txt: String,
    search_list: Vec<Password>,
    new_title: String,
    new_username: String,
    new_password: String,
}
impl PassManager {
    fn new () -> Self {
        Self {
            mode: InputMode::Normal,
            list_state: ListState::default(),
            passwords: vec![],
            search_txt: String::new(),
            search_list: vec![],
            new_title: String::new(),
            new_username: String::new(),
            new_password: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut state = PassManager::new();

    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;

    if let Err(e) = result {
        println!("{}", e.to_string());
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut PassManager) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|frame| ui(frame, state))?;

        if let Key(key) =  event::read()? {
            match state.mode {
                InputMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(())
                        }
                        KeyCode::Char('s') => {
                            // search mode
                        }
                        KeyCode::Char('l') => {
                            // list mode
                        }
                        KeyCode::Insert => {
                            // insert mode
                        }
                        _ => {}
                    }
                }
                InputMode::Title => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from title to normal mode
                        }
                        KeyCode::Char(c) => {
                            state.new_title.push(c);
                        }
                        KeyCode::Backspace => {
                            state.new_title.pop();
                        }
                        _ => {}
                    }
                }
                InputMode::Username => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from username to normal mode
                        }
                        KeyCode::Char(c) => {
                            state.new_username.push(c);
                        }
                        KeyCode::Backspace => {
                            state.new_username.pop();
                        }
                        _ => {}
                    }
                }
                InputMode::Password => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from password to normal mode
                        }
                        KeyCode::Char(c) => {
                            state.new_password.push(c);
                        }
                        KeyCode::Backspace => {
                            state.new_password.pop();
                        }
                        _ => {}
                    }
                }
                InputMode::Submit => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from submit to normal mode
                        }
                        KeyCode::BackTab => {
                            // change mode to Password
                        }
                        _ => {}
                    }
                }
                InputMode::Search => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from search to normal mode
                        }
                        _ => {}
                    }
                }
                InputMode::List => {
                    match key.code {
                        KeyCode::Esc => {
                            // exit from list to normal mode
                        }
                        _ => {}
                    }
                }
                InputMode::Delete => {}
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, state: &mut PassManager) {}