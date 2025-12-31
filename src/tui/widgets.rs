use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize, palette::tailwind::SLATE},
    text::{Line, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget,
    },
};

use crate::store::Snippet;

pub struct TopWidget {}
pub struct SnippetListWidget<'a> {
    pub snippets: &'a [Snippet],
    pub state: ListState,
}
pub struct SnippetDetailWidget<'a> {
    pub snippet: Option<&'a Snippet>,
}

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

        let items = self.snippets.iter().map(ListItem::from);
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl<'a> Widget for &SnippetDetailWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Navigate ".into(),
            "<↑/↓>".blue().bold(),
            " Select ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Ctrl-C> ".blue().bold(),
        ]);

        if let Some(snippet) = self.snippet {
            let title_text = format!(" {} ", snippet.title);
            let title_block = Block::default()
                .title(Line::from(title_text).bold().centered())
                .title_bottom(instructions.centered())
                .borders(Borders::ALL)
                .padding(Padding::uniform(1));

            let mut lines: Vec<Line> = snippet.command.lines().map(Line::from).collect();
            if !snippet.description.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(snippet.description.as_str().dark_gray()));
            }

            Paragraph::new(Text::from(lines))
                .block(title_block)
                //.scroll((self.scroll, 0))
                .render(area, buf);
        } else {
            Block::default()
                .borders(Borders::ALL)
                .title(" No Snippet Selected ")
                .title_bottom(instructions)
                .render(area, buf);
        }
    }
}
