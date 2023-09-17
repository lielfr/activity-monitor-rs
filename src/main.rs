mod widgets;

use crate::widgets::{gen_cpu_usage, gen_mem_usage, gen_network_usage};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use ratatui::{Frame, Terminal};
use std::io::stdout;
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new();
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen,).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|f| draw(f, &mut sys)).unwrap();
        if event::poll(Duration::from_millis(250)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if KeyCode::Char('q') == key.code
                    || (KeyCode::Char('c') == key.code && key.modifiers == KeyModifiers::CONTROL)
                {
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(250));
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen,).unwrap();
    terminal.show_cursor().unwrap();
}

fn draw<B: Backend>(f: &mut Frame<B>, sys: &mut System) {
    sys.refresh_all();
    let size = f.size();
    let block = Block::default()
        .title("Activity Monitor")
        .borders(Borders::ALL);
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Length(42),
            Constraint::Length(7),
            Constraint::Percentage(10),
        ])
        .split(size);

    let cpu_usage = gen_cpu_usage(sys);

    f.render_widget(cpu_usage, layout[0]);

    let mem_usage = gen_mem_usage(sys);
    f.render_widget(mem_usage, layout[1]);

    let network_usage = gen_network_usage(sys);
    f.render_widget(network_usage, layout[2]);

    f.render_widget(block, size);
}
