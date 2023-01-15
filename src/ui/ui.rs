use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};

use crate::{resource::Resource, ui::state::State};

pub struct UI<B>
where B: Backend, {
    terminal: Terminal<B>,
    state: State,
}

impl<B> UI<B>
where B: Backend, {
    pub fn new(terminal: Terminal<B>) -> Self {
        let state = State {
            ..Default::default()
        };
        UI {
            terminal, 
            state
        }
    }

    pub fn _draw(&mut self) {
        self.terminal
            .draw(|mut frame|{ 
                // render layout component
            }).unwrap();
    }

    //pub fn draw(self) -> Result<(), Box<dyn Error>> {
    //    // setup terminal
    //    enable_raw_mode()?;
    //    let mut stdout = io::stdout();
    //    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    //    let backend = CrosstermBackend::new(stdout);
    //    let mut terminal = Terminal::new(backend)?;

    //    // create app and run it
    //    let res = self.run_app(&mut terminal);

    //    // restore terminal
    //    disable_raw_mode()?;
    //    execute!(
    //        terminal.backend_mut(),
    //        LeaveAlternateScreen,
    //        DisableMouseCapture
    //    )?;
    //    terminal.show_cursor()?;

    //    if let Err(err) = res {
    //        println!("{:?}", err)
    //    }

    //    Ok(())
    //}

    //fn run_app<B: Backend>(self, terminal: &mut Terminal<B>) -> io::Result<()> {
    //    loop {
    //        terminal.draw(self.ui)?;

    //        if let Event::Key(key) = event::read()? {
    //            if let KeyCode::Char('q') = key.code {
    //                return Ok(());
    //            }
    //        }
    //    }
    //}

    //fn ui<B: Backend>(self, f: &mut Frame<B>) {
    //    // Wrapping block for a group
    //    // Just draw the block and the group on the same area and build the group
    //    // with at least a margin of 1
    //    let size = f.size();

    //    // Surrounding block
    //    let block = Block::default()
    //        .borders(Borders::ALL)
    //        .title("Main block with round corners")
    //        .title_alignment(Alignment::Center)
    //        .border_type(BorderType::Rounded);
    //    f.render_widget(block, size);

    //    let chunks = Layout::default()
    //        .direction(Direction::Vertical)
    //        .margin(4)
    //        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //        .split(f.size());

    //    // Top two inner blocks
    //    let top_chunks = Layout::default()
    //        .direction(Direction::Horizontal)
    //        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //        .split(chunks[0]);

    //    // Top left inner block with green background
    //    let block = Block::default().title("With borders").borders(Borders::ALL);
    //    f.render_widget(block, top_chunks[0]);

    //    // Top right inner block with styled title aligned to the right
    //    let block = Block::default().title("With borders").borders(Borders::ALL);
    //    f.render_widget(block, top_chunks[1]);

    //    // Bottom two inner blocks
    //    let bottom_chunks = Layout::default()
    //        .direction(Direction::Horizontal)
    //        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    //        .split(chunks[1]);

    //    // Bottom left block with all default borders
    //    let block = Block::default().title("With borders").borders(Borders::ALL);
    //    f.render_widget(block, bottom_chunks[0]);

    //    // Bottom right block with styled left and right border
    //    let block = Block::default().title("With borders").borders(Borders::ALL);
    //    f.render_widget(block, bottom_chunks[1]);
    //}
}
