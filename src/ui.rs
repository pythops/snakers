use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;

pub fn chunk(coordinates: &(u16, u16), r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(coordinates.1),
                Constraint::Length(1),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(coordinates.0),
                Constraint::Length(1),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let area = frame.size();

    let vmid_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(area)[1];

    let vh_mid_area = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vmid_area)[1];

    let container = Block::default().style(Style::default());

    frame.render_widget(container.clone(), vh_mid_area);

    let inside_container = container.inner(vh_mid_area);

    let (game_container, score_container) = {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(inside_container);
        (chunks[0], chunks[1])
    };

    if app.game_over {
        let game_over_block = Paragraph::new(format!(
            r#"
Game Over

score: {}

Press `r` to play again `q` to quit.
        "#,
            app.score
        ))
        .block(Block::default())
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(area);

        let popup_block = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(40),
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1];

        frame.render_widget(Clear, popup_block);
        frame.render_widget(game_over_block, popup_block);
        return;
    }

    let score_block = Paragraph::new(format!("Score: {}", app.score))
        .block(Block::default())
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);
    frame.render_widget(Clear, score_container);
    frame.render_widget(score_block, score_container);

    let game_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    frame.render_widget(Clear, game_container);
    frame.render_widget(game_block.clone(), game_container);

    let prey_block = Paragraph::new("◼")
        .block(Block::default())
        .style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .alignment(Alignment::Center);

    let prey_container = chunk(&app.prey, game_container);
    frame.render_widget(Clear, prey_container);
    frame.render_widget(prey_block, prey_container);

    let block = Paragraph::new(" ")
        .block(Block::default())
        .style(Style::default().bg(Color::Green));

    let head_container = chunk(&app.head.coordinates, game_container);
    frame.render_widget(Clear, head_container);
    frame.render_widget(block.clone(), head_container);

    for c in &app.body {
        let body_chunk = chunk(c, game_container);
        frame.render_widget(Clear, body_chunk);
        frame.render_widget(block.clone(), body_chunk);
    }

    app.boundaries = (game_container.width - 2, game_container.height - 2);
}
