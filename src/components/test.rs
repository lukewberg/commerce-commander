use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::core::init::Result;
use crate::core::view::View;

pub struct Test {}

impl View for Test {
    fn draw(&mut self, f: &mut Frame<'_>) -> Result<()> {
        f.render_widget(
            Paragraph::new(format!(
                "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
      ",
            ))
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
            f.size(),
        );
        Ok(())
    }
}
