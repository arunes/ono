use color_eyre::{Result, eyre::WrapErr};

use crossterm::event::KeyModifiers;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

use crate::{
    store::Snippet,
    tui::{
        self,
        widgets::{SearchWidget, SnippetDetailWidget, SnippetListWidget, TopWidget},
    },
};

#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,

    pub snippets: Vec<Snippet>,
    pub list_state: ListState,
}

enum ListMoveDirection {
    Up,
    Down,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(45),
                Constraint::Percentage(50),
                Constraint::Percentage(5),
            ])
            .split(frame.area());

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(outer_layout[1]);

        frame.render_widget(&TopWidget {}, outer_layout[0]);

        frame.render_widget(
            &mut SnippetListWidget {
                snippets: &self.snippets,
                state: self.list_state,
            },
            inner_layout[0],
        );

        frame.render_widget(
            &SnippetDetailWidget {
                snippet: self.list_state.selected().map(|idx| &self.snippets[idx]),
            },
            inner_layout[1],
        );

        frame.render_widget(&SearchWidget { query: "" }, outer_layout[2]);
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                self.exit()
            }
            KeyCode::Up => self.move_list_selection(ListMoveDirection::Up),
            KeyCode::Down => self.move_list_selection(ListMoveDirection::Down),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn move_list_selection(&mut self, direction: ListMoveDirection) {
        if self.snippets.is_empty() {
            return;
        }

        let current_index = self.list_state.selected().unwrap_or(0);
        let last_index = self.snippets.len().saturating_sub(1);

        match direction {
            ListMoveDirection::Up if current_index > 0 => self.list_state.select_previous(),
            ListMoveDirection::Down if current_index < last_index => self.list_state.select_next(),
            _ if self.list_state.selected().is_none() => self.list_state.select_first(),
            _ => {}
        }
    }
}
