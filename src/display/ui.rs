use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    Frame,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
};

pub fn draw_ui<B>(f: &mut Frame<B>, app: &App) -> ()
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    
    let tab_titles = app
    .matches_info
    .iter()
    .map(|m| Spans::from(Span::styled(m.match_short_name.as_str(), Style::default().fg(Color::Green))))
    .collect();

    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Block Title"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.focused_tab as usize);
    f.render_widget(tabs, chunks[0]);
    draw_tab(f, chunks[1]);
}

fn draw_tab<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let _chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);

    let text = vec![
            Spans::from("This is a paragraph with several lines. You can change style your text the way you want"),
            Spans::from(""),
            Spans::from(vec![
                Span::from("For example: "),
                Span::styled("under", Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::styled("the", Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled("rainbow", Style::default().fg(Color::Blue)),
                Span::raw("."),
            ]),
            Spans::from(vec![
                Span::raw("Oh and if you didn't "),
                Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw(" you can "),
                Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" "),
                Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
                Span::raw(" your "),
                Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::raw(".")
            ]),
        ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
