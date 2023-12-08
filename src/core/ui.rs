use ratatui::{
  prelude::{Alignment, Frame},
  style::{Color, Style},
  widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{core::app::App, components::test::Test};

use super::navigator::{Navigator, Route};

pub fn render(app: &mut App, f: &mut Frame) {
  // let mut navigator = Navigator::new(vec![
  //   Route::new("/", Box::new(Test{}))
  // ]);
  // navigator.run(f);
  // f.render_widget(
  //   Paragraph::new(format!(
  //     "
  //       Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
  //       Press `j` and `k` to increment and decrement the counter respectively.\n\
  //       Counter: {}
  //     ",
  //     app.counter
  //   ))
  //   .block(
  //     Block::default()
  //       .title("Counter App")
  //       .title_alignment(Alignment::Center)
  //       .borders(Borders::ALL)
  //       .border_type(BorderType::Rounded),
  //   )
  //   .style(Style::default().fg(Color::Yellow))
  //   .alignment(Alignment::Center),
  //   f.size(),
  // )
}