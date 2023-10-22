use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["TextGen", "GLaDOS", "MicCapture", "VoiceTranscription"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().on_black().white();
    f.render_widget(block, size);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![first.yellow(), rest.green()])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Rgb(50, 50, 50)),
        );
    f.render_widget(tabs, chunks[0]);
    let area = chunks[1];
    match app.titles[app.index] {
        "TextGen" => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(25),
                        Constraint::Min(8),
                        Constraint::Length(25),
                    ]
                    .as_ref(),
                )
                .split(area);

            { // left
                let par = Paragraph::new("bruh moment").block(
                    Block::default()
                        .title("Sections")
                        .borders(Borders::ALL),
                );
                f.render_widget(par, chunks[0]);
            }

            { // middle
                let par = Paragraph::new("bruh moment").block(
                    Block::default()
                        .title("Details")
                        .borders(Borders::ALL),
                );
                f.render_widget(par, chunks[1]);
            }
            { // right
                let par = Paragraph::new("bruh moment").block(
                    Block::default()
                        .title("Status")
                        .borders(Borders::ALL),
                );
                f.render_widget(par, chunks[2]);
            }
        }
        "GLaDOS" => {
            let block = Block::default()
                .title("GLaDOS TTS Server")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1])
        }
        "MicCapture" => {
            let block = Block::default()
                .title("Microphone Capture Server")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1])
        }
        "VoiceTranscription" => {
            let block = Block::default()
                .title("Voice Transcription Server")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1])
        }
        _ => unreachable!(),
    };
}
