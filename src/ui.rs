use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

pub fn ui_list() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = vec![
        "Item 1",
        "Item 2",
        "Item 3",
        "Item 4",
        "Item 5",
    ];
    let mut list_state = ListState::default();
    list_state.select(Some(0)); 

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let items: Vec<ListItem> = items
                .iter()
                .map(|i| ListItem::new(*i))
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Selectable List"))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        // Handle Input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    let i = match list_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                items.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    list_state.select(Some(i));
                }
                KeyCode::Down => {
                    let i = match list_state.selected() {
                        Some(i) => {
                            if i >= items.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    list_state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = list_state.selected() {
                        println!("You selected: {}", items[i]);
                        break;
                    }
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

