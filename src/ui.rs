use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Masked, Span};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{DefaultTerminal, Frame};

#[derive(Default)]
struct App {
    vertical_scroll: usize,
    vertical_scroll_state: ScrollbarState,
}

pub fn main(paragraphs: Vec<String>) -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal, paragraphs))
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal, paragraphs: Vec<String>) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| self.render(frame, paragraphs.clone()))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                last_tick = Instant::now();
                continue;
            }

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.scroll_down(),
                    KeyCode::Char('k') | KeyCode::Up => self.scroll_up(),
                    _ => {}
                }
            }
        }
    }

    fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
    }

    fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
    }

    fn render(&mut self, frame: &mut Frame, paragraphs: Vec<String>) {
        let area = frame.area();

        let chunks = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        let text: Vec<Line> = paragraphs.into_iter().map(Line::from).collect();

        self.vertical_scroll_state = self
            .vertical_scroll_state
            .content_length(text.len())
            .position(self.vertical_scroll);

        let v = self.vertical_scroll.min(u16::MAX as usize) as u16;

        let title = Block::new()
            .title_alignment(Alignment::Center)
            .title("Use j / k or ↑ ↓ to scroll — q to quit".bold());
        frame.render_widget(title, chunks[0]);

        let paragraph = Paragraph::new(text)
            .gray()
            .block(Block::bordered().gray().title("Vertical scrollbar".bold()))
            .scroll((v, 0));

        frame.render_widget(paragraph, chunks[1]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            chunks[1],
            &mut self.vertical_scroll_state,
        );
    }
}
