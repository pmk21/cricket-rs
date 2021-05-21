use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;
mod cricbuzz_api;
mod display;
mod event;

use crate::event::Key;
use app::App;
use display::ui::draw_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await;

    // UI part
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new(5000);

    loop {
        if !app.matches_info.is_empty() {
            terminal.draw(|mut f| {
                draw_ui(&mut f, &app);
            })?;
        } else {
            safely_close_tui()?;
            println!("No live matches :(");
            break;
        }

        match events.next()? {
            event::Event::Input(key) => {
                match key {
                    Key::Ctrl('c') => {
                        safely_close_tui()?;
                        break;
                    }
                    Key::Right => {
                        if app.focused_tab < (app.matches_info.len() - 1) {
                            app.focused_tab += 1;
                        }
                    }
                    Key::Left => {
                        if app.focused_tab > 0 {
                            app.focused_tab = app.focused_tab.saturating_sub(1);
                        }
                    }
                    Key::Down => {
                        let tab_idx = app.focused_tab;
                        app.matches_info[tab_idx].scorecard_scroll =
                            app.matches_info[tab_idx].scorecard_scroll.saturating_add(2);
                    }
                    Key::Up => {
                        let tab_idx = app.focused_tab;
                        app.matches_info[tab_idx].scorecard_scroll =
                            app.matches_info[tab_idx].scorecard_scroll.saturating_sub(2);
                    }
                    _ => {}
                };
            }

            event::Event::Tick => {
                app.update_on_tick().await;
            }
        }
    }
    Ok(())
}

fn safely_close_tui() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
