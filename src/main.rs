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
    // println!("{:#?}", app.matches_info[0].cricbuzz_info);

    // UI part
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new(250);

    loop {
        terminal.draw(|mut f| {
            draw_ui(&mut f, &app);
        })?;

        match events.next()? {
            event::Event::Input(key) => {
                match key {
                    Key::Ctrl('c') => {
                        disable_raw_mode()?;
                        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
                        break;
                    }
                    Key::Right => {
                        if app.focused_tab < ((app.matches_info.len() - 1) as u32) {
                            app.focused_tab += 1;
                        }
                    }
                    Key::Left => {
                        if app.focused_tab > 0 {
                            app.focused_tab = app.focused_tab.wrapping_sub(1);
                        }
                    }
                    Key::Down => {
                        let tab_idx = app.focused_tab as usize;
                        app.matches_info[tab_idx].scorecard_scroll =
                            app.matches_info[tab_idx].scorecard_scroll.saturating_add(2);
                    }
                    Key::Up => {
                        let tab_idx = app.focused_tab as usize;
                        app.matches_info[tab_idx].scorecard_scroll =
                            app.matches_info[tab_idx].scorecard_scroll.saturating_sub(2);
                    }
                    _ => {}
                };
            }

            _ => {}
        }
    }
    Ok(())
}
