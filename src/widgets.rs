use ratatui::layout::Alignment::Center;
use ratatui::prelude::*;
use ratatui::style::Style;
use ratatui::widgets::block::Position;
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders};
use sysinfo::{CpuExt, NetworkExt, NetworksExt, System, SystemExt};

pub fn gen_cpu_usage(sys: &mut System) -> BarChart {
    let cpu_data = sys
        .cpus()
        .iter()
        .enumerate()
        .map(|(id, data)| ((id + 1).to_string(), data.cpu_usage() as u64))
        .map(|(cpu_label, usage)| {
            Bar::default()
                .label(Line::from(cpu_label).alignment(Alignment::Center))
                .value(usage)
                .text_value(String::new())
        })
        .collect::<Vec<_>>();

    let cpu_data = BarGroup::default().bars(cpu_data.as_ref());

    let cpu_usage = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("CPU")
                .title_position(Position::Bottom)
                .title_alignment(Alignment::Center),
        )
        .bar_style(Style::new().yellow().on_red())
        .bar_width(3)
        .bar_gap(1)
        .max(100)
        .data(cpu_data);

    cpu_usage
}

pub fn gen_mem_usage(sys: &mut System) -> BarChart {
    let data = ((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0) as u64;
    let bar_data = BarGroup::default().bars(&[Bar::default()
        .label(Line::from("%USG").alignment(Alignment::Center))
        .value(data)]);

    let mem_usage = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("MEM")
                .title_position(Position::Bottom)
                .title_alignment(Center),
        )
        .bar_style(Style::new().green().on_red())
        .bar_width(4)
        .bar_gap(1)
        .data(bar_data)
        .max(100);

    mem_usage
}

pub fn gen_network_usage(sys: &mut System) -> BarChart {
    let network_data: Vec<_> = sys
        .networks()
        .iter()
        .filter(|(_, data)| data.received() > 0 || data.transmitted() > 0)
        .flat_map(|(interface_name, data)| {
            [
                Bar::default()
                    .label(Line::from(format!("{interface_name} R")))
                    .value(data.received()),
                Bar::default()
                    .label(Line::from(format!("{interface_name} T")))
                    .value(data.transmitted()),
            ]
        })
        .collect();

    let network_bar_group = BarGroup::default().bars(&network_data);

    BarChart::default()
        .block(
            Block::default()
                .title("NETWORK")
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center)
                .title_position(Position::Bottom),
        )
        .bar_width(3)
        .bar_style(Style::default().light_blue().on_red())
        .data(network_bar_group)
}
