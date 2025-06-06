use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, List, ListState, Paragraph},
};

pub fn run_csv_editor(terminal: &mut DefaultTerminal, filename: String) -> Result<()> {
    let mut list_action_option = ListState::default();
    let mut popover = false;

    loop {
        let _ = terminal.draw(|frame: &mut Frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(frame.area());

            let action_list = List::new([
                Text::styled("Sort", Style::new().red()),
                Text::styled("Search", Style::new().red()),
                Text::styled("Info", Style::new().red()),
            ])
            .block(Block::bordered().title("Actions"))
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>");

            // Define the main paragraph
            let paragraph = Paragraph::new(Text::raw("Hello, world!"));

            // Define the popover paragraph
            let parpopover = Paragraph::new(Text::raw("Current mode is CSV"))
                .block(Block::bordered().title("Warning"))
                .style(Style::new().white().on_black()); // Optional: style for visibility

            // Render the main paragraph in layout[0]
            frame.render_widget(paragraph, layout[0]);

            // Render the action list in layout[1]
            frame.render_stateful_widget(action_list, layout[1], &mut list_action_option);

            // Render the popover if active
            /*   if popover {
                // Create a centered popover area (e.g., 30% of terminal width and 5 lines tall)
                let popover_area = centered_rect(30, 20, frame.area());
                frame.render_widget(parpopover, popover_area);
            } */

            if popover {
                // Centered Rect for the popover
                let popover_width = frame.area().width * 30 / 100; // 30% of terminal width
                let popover_height = 10; // Fixed height of 5 lines
                let popover_x = (frame.area().width - popover_width) / 2; // Center horizontally
                let popover_y = (frame.area().height - popover_height) / 2; // Center vertically
                let popover_area = Rect {
                    x: popover_x,
                    y: popover_y,
                    width: popover_width,
                    height: popover_height,
                };
                frame.render_widget(parpopover, popover_area);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Down | KeyCode::Char('l') => {
                    let i = match list_action_option.selected() {
                        Some(i) => {
                            if i >= 2 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    list_action_option.select(Some(i));
                }
                KeyCode::Up | KeyCode::Char('h') => {
                    let i = match list_action_option.selected() {
                        Some(i) => {
                            if i == 0 {
                                2
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    list_action_option.select(Some(i));
                }
                KeyCode::Char('m') => {
                    popover = !popover; // Toggle popover
                }
                _ => {}
            }
        }
    }
    Ok(())
}

// Helper to create a centered rectangle for the popover
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
