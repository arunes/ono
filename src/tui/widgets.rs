use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize, palette::tailwind::SLATE},
    text::{Line, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget,
    },
};

use crate::store::Snippet;

pub struct TopWidget {}
pub struct SnippetListWidget<'a> {
    pub snippets: &'a [Snippet],
    pub state: ListState,
}
pub struct SnippetDetailWidget {}

impl Widget for &TopWidget {
    fn render(self, _area: Rect, _buf: &mut Buffer) {}
}

const TEXT_FG_COLOR: Color = SLATE.c200;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

impl From<&Snippet> for ListItem<'_> {
    fn from(value: &Snippet) -> Self {
        let line = Line::styled(format!("- {}", value.title), TEXT_FG_COLOR);
        // let line = match value.status {
        //     Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
        //     Status::Completed => {
        //         Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
        //     }
        // };
        ListItem::new(line)
    }
}

impl<'a> Widget for &mut SnippetListWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL);

        let items = self.snippets.iter().map(|snippet| ListItem::from(snippet));
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl Widget for &SnippetDetailWidget {
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
