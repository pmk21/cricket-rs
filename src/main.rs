use clap::{crate_version, value_parser, Arg, ArgAction, Command};
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
use display::ui::{draw_ui, UiState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("cricket-rs")
        .author("Prithvi MK <prithvikrishna49 AT gmail DOT com>")
        .version(crate_version!())
        .about("Fast and optimized live cricket score viewer in the terminal")
        .arg(
            Arg::new("tick-rate")
                .short('t')
                .long("tick-rate")
                .value_name("MILLISECONDS")
                .help("Sets match details refresh rate")
                .default_value("40000")
                .value_parser(value_parser!(u64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("match-id")
                .short('m')
                .long("match-id")
                .value_name("ID")
                .help("ID of the match to follow live")
                .default_value("0")
                .value_parser(value_parser!(u32))
                .action(ArgAction::Set),
        )
        .get_matches();

    let tick_rate = *matches.get_one("tick-rate").unwrap_or(&40000);
    let match_id = *matches.get_one("match-id").unwrap_or(&0);

    let mut app = if match_id == 0 {
        App::new().await
    } else {
        App::new_with_match_id(match_id).await
    };

    // UI part
    let mut ui_state = UiState::new(app.matches_info.len());
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = event::Events::new(tick_rate);

    loop {
        if !app.matches_info.is_empty() {
            terminal.draw(|f| {
                draw_ui(f, &app, &mut ui_state);
            })?;
        } else {
            safely_close_tui()?;
            println!("No live matches :(");
            break;
        }

        match events.next()? {
            event::Event::Input(key) => {
                match key {
                    Key::Ctrl('c') | Key::Char('q') => {
                        safely_close_tui()?;
                        break;
                    }
                    Key::Right => {
                        ui_state.add_focused_tab(1);
                    }
                    Key::Left => {
                        ui_state.sub_focused_tab(1);
                    }
                    Key::Down => {
                        ui_state.add_scrd_scroll(1);
                    }
                    Key::Up => {
                        ui_state.sub_scrd_scroll(1);
                    }
                    _ => {}
                };
            }

            event::Event::Tick => {
                let invalid_idx = app.update_on_tick().await;
                ui_state.update_on_tick(&invalid_idx);
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
