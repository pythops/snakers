use crate::app::{App, AppResult, Direction};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if app.head.direction != Direction::LEFT {
                app.head.direction = Direction::RIGHT;
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if app.head.direction != Direction::RIGHT {
                app.head.direction = Direction::LEFT;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.head.direction != Direction::DOWN {
                app.head.direction = Direction::UP;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.head.direction != Direction::UP {
                app.head.direction = Direction::DOWN;
            }
        }

        KeyCode::Char('r') => {
            app.reset();
        }
        _ => {}
    }
    Ok(())
}
