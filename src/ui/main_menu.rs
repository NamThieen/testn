use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

pub fn draw_main_menu(f: &mut Frame, app: &App) {
    let size = f.area();
    f.render_widget(ratatui::widgets::Clear, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // "Ratatui Quiz"
            Constraint::Min(0),    // spacer
            Constraint::Length(4), // the "menu"
            Constraint::Min(0),
            Constraint::Length(3), // spacer
            Constraint::Length(3), // bottom margin
        ])
        .split(size);
    // the title text "Ratatui yaml Quiz"
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    let title_text = Paragraph::new(Text::styled(
        "Ratatui Quiz",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Magenta),
    ))
    .block(title_block)
    .alignment(Alignment::Center)
    .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(title_text, chunks[0]);
    //
    // FIXME the menu
    //
    let menu = vec![
        "1. Start Quiz (s)".to_string(),
        "2. Create New Test (not yet implemented)".to_string(),
        "3. Load test from file (l)".to_string(),
        "4. Quit (q)".to_string(),
    ];
    let menu_items: Vec<ListItem> = menu
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let mut style = Style::default().fg(Color::White);
            if i == app.selected_menu_item {
                style = style.bg(Color::LightBlue).fg(Color::Black);
            }
            ListItem::new(item.as_str()).style(style)
        })
        .collect();

    let menu_list = List::new(menu_items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_symbol(">>");
    f.render_widget(menu_list, chunks[2]);

    let instructions = Paragraph::new(Span::styled(
        "Press 1, 2, 3 or 4 to select an option, or 'q' to quit.",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::ITALIC),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_type(BorderType::Plain),
    );
    f.render_widget(instructions, chunks[4]);
}
