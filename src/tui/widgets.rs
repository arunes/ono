use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct TopWidget {}
pub struct SnippetListWidget {
    pub counter: u8,
}
pub struct SnippetDetailWidget {}

impl Widget for TopWidget {
    fn render(self, _area: Rect, _buf: &mut Buffer) {}
}

impl Widget for SnippetListWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for SnippetDetailWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" selected snippet ".bold());
        let instructions = Line::from(vec![
            " Navigate ".into(),
            "<↑/↓>".blue().bold(),
            " Select ".into(),
            "<Ender>".blue().bold(),
            " Quit ".into(),
            "<Ctrl-C> ".blue().bold(),
        ]);

        Block::default()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .borders(Borders::ALL)
            .render(area, buf);
    }
}
