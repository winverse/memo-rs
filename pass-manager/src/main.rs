use crossterm::event::Event::Key;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use std::error::Error;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::{Frame, Terminal};

const APP_KEYS_DESC: &str = r#"
L:           Show/Hide List
U:           On list, It's copy the Username
P:           On list, It's copy the Password
D:           On list, It's Delete
E:           On list, It's Edit
S:           Search
Insert:      Insert new Password
Tab:         Go to next field
Shift+Tab:   Go to previous filed
Esc:         Exit insert mode
"#;

#[derive(Eq, PartialEq)]
enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    Submit,
    Search,
    List,
    Delete,
}

#[derive(Clone)]
struct Password {
    title: String,
    username: String,
    password: String,
}
impl Password {
    pub fn new(title: String, username: String, password: String) -> Self {
        Self {
            title,
            username,
            password,
        }
    }
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
    fn new() -> Self {
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

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn clear_fields(&mut self) {
        self.new_title.clear();
        self.new_username.clear();
        self.new_password.clear();
    }

    pub fn insert(&mut self) {
        let password = Password::new(
            self.new_title.to_owned(),
            self.new_username.to_owned(),
            self.new_password.to_owned(),
        );
        self.passwords.push(password);
        self.clear_fields();
        self.change_mode(InputMode::Normal);
    }

    pub fn search(&mut self) {
        self.search_list = self
            .passwords
            .iter()
            .filter(|password| password.title.starts_with(&self.search_txt))
            .cloned()
            .collect();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = PassManager::new();

    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut PassManager,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|frame| ui(frame, state))?;
        if let Key(key) = event::read()? {
            match state.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => {
                        state.change_mode(InputMode::Search);
                    }
                    KeyCode::Char('l') => {
                        state.change_mode(InputMode::List);
                    }
                    // insert key
                    KeyCode::Char('\u{f746}') => {
                        state.change_mode(InputMode::Title);
                    }
                    _ => {}
                },
                InputMode::Title => match key.code {
                    KeyCode::Esc => {
                        state.clear_fields();
                        state.change_mode(InputMode::Normal);
                    }
                    KeyCode::Char(c) => {
                        state.new_title.push(c);
                    }
                    KeyCode::Backspace => {
                        state.new_title.pop();
                    }
                    KeyCode::Tab => {
                        state.change_mode(InputMode::Username);
                    }
                    _ => {}
                },
                InputMode::Username => match key.code {
                    KeyCode::Esc => {
                        state.clear_fields();
                        state.change_mode(InputMode::Normal);
                    }
                    KeyCode::Char(c) => {
                        state.new_username.push(c);
                    }
                    KeyCode::Backspace => {
                        state.new_username.pop();
                    }
                    KeyCode::Tab => {
                        state.change_mode(InputMode::Password);
                    }
                    KeyCode::BackTab => {
                        state.change_mode(InputMode::Title);
                    }
                    _ => {}
                },
                InputMode::Password => match key.code {
                    KeyCode::Esc => {
                        state.clear_fields();
                        state.change_mode(InputMode::Normal);
                    }
                    KeyCode::Char(c) => {
                        state.new_password.push(c);
                    }
                    KeyCode::Backspace => {
                        state.new_password.pop();
                    }
                    KeyCode::Tab => {
                        state.change_mode(InputMode::Submit);
                    }
                    KeyCode::BackTab => {
                        state.change_mode(InputMode::Username);
                    }
                    _ => {}
                },
                InputMode::Submit => match key.code {
                    KeyCode::Esc => {
                        state.clear_fields();
                        state.change_mode(InputMode::Normal);
                    }
                    KeyCode::BackTab => state.change_mode(InputMode::Password),
                    KeyCode::Enter => {
                        state.insert();
                    }
                    _ => {}
                },
                InputMode::Search => match key.code {
                    KeyCode::Esc => {
                        state.change_mode(InputMode::Normal);
                    }
                    KeyCode::Char(c) => {
                        state.search_txt.push(c);
                        state.search();
                    }
                    KeyCode::Backspace => {
                        state.search_txt.pop();
                        state.search();
                    }
                    _ => {}
                },
                InputMode::List => match key.code {
                    KeyCode::Char('l') => {
                        state.change_mode(InputMode::Normal);
                    }
                    KeyCode::Esc => {
                        state.change_mode(InputMode::Normal);
                    }
                    _ => {}
                },
                InputMode::Delete => {}
            }
        }
    }
}

// static dispatch
// ui함수는 B 타입이 Backend 트레잇을 구현하는 경우에만 호출한다.
fn ui<B: Backend>(frame: &mut Frame<B>, state: &mut PassManager) {
    let parent_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    let new_section_block = Block::default()
        .title("New Password")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(new_section_block, parent_chunk[0]);
    new_section(frame, state, parent_chunk[0]);

    let list_section_block = Block::default()
        .title("List of password")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(list_section_block, parent_chunk[1]);
    list_section(frame, state, parent_chunk[1]);
}

fn new_section<B: Backend>(frame: &mut Frame<B>, state: &mut PassManager, area: Rect) {
    let new_section_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(4),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    let desc = Paragraph::new(APP_KEYS_DESC);
    frame.render_widget(desc, new_section_chunk[0]);

    let title_input = Paragraph::new(state.new_title.to_owned())
        .block(
            Block::default()
                .title("Title")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Title => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    frame.render_widget(title_input, new_section_chunk[1]);

    let username_input = Paragraph::new(state.new_username.to_owned())
        .block(
            Block::default()
                .title("Username")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Username => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    frame.render_widget(username_input, new_section_chunk[2]);

    let password_input = Paragraph::new(state.new_password.to_owned())
        .block(
            Block::default()
                .title("Password")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Password => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    frame.render_widget(password_input, new_section_chunk[3]);

    let submit_btn = Paragraph::new("Submit")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Submit => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    frame.render_widget(submit_btn, new_section_chunk[4]);
}

fn list_section<B: Backend>(frame: &mut Frame<B>, state: &mut PassManager, area: Rect) {
    let list_to_show = if state.mode == InputMode::Search {
        state.search_list.to_owned()
    } else {
        state.passwords.to_owned()
    };
    let items: Vec<ListItem> = list_to_show
        .into_iter()
        .map(|item| match state.mode {
            InputMode::List => ListItem::new(format!(
                "{}: {} - {}",
                item.title.to_owned(),
                item.username.to_owned(),
                item.password.to_owned()
            )),
            _ => ListItem::new(Span::from(item.title)),
        })
        .collect();

    let list_chunks = Layout::default()
        .margin(2)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let search_input = Paragraph::new(state.search_txt.to_owned())
        .block(
            Block::default()
                .title("Search")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Search => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    frame.render_widget(search_input, list_chunks[0]);

    let list = List::new(items)
        .block(Block::default())
        .highlight_symbol("->")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_stateful_widget(list, list_chunks[1], &mut state.list_state);
}
