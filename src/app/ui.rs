// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use tui_logger::TuiLoggerSmartWidget;

use crate::app::core::{App, InputMode, TabItem};

pub struct Scroll {
    pub x: u16,
    pub y: u16,
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.tab_item {
        TabItem::Editor => draw_sql_editor_tab(f, app),
        TabItem::QueryHistory => draw_query_history_tab(f, app),
        TabItem::Context => draw_context_tab(f, app),
        TabItem::Logs => draw_logs_tab(f, app),
    }
}

fn draw_sql_editor_tab<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(20),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let help_message = draw_sql_editor_help(app);
    f.render_widget(help_message, chunks[0]);

    let tabs = draw_tabs(app);
    f.render_widget(tabs, chunks[1]);
    let editor = draw_editor(app);
    f.render_widget(editor, chunks[2]);
    draw_cursor(app, f, &chunks);
    let query_results = draw_query_results(app);
    f.render_widget(query_results, chunks[3]);
}

fn draw_query_history_tab<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let help_message = draw_default_help();
    f.render_widget(help_message, chunks[0]);

    let tabs = draw_tabs(app);
    f.render_widget(tabs, chunks[1]);
    let query_history = draw_query_history(app);
    f.render_widget(query_history, chunks[2])
}

fn draw_context_tab<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let help_message = draw_default_help();
    f.render_widget(help_message, chunks[0]);

    let tabs = draw_tabs(app);
    f.render_widget(tabs, chunks[1]);

    draw_context(f, app, chunks[2]);
}

fn draw_logs_tab<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let help_message = draw_default_help();
    f.render_widget(help_message, chunks[0]);

    let tabs = draw_tabs(app);
    f.render_widget(tabs, chunks[1]);
    let logs = draw_logs(app);
    f.render_widget(logs, chunks[2])
}

fn draw_sql_editor_help<'screen>(app: &App) -> Paragraph<'screen> {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing, "),
                Span::styled("c", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to clear the editor, "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" start 'rc' mode, "),
                Span::styled("#", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to change tabs."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to execute query after closing ';'"),
            ],
            Style::default(),
        ),
        InputMode::Rc => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit Rc mode, "),
                Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to load 'rc' file into editor, "),
                Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to reload 'rc' file, "),
                Span::styled("w", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to write 'rc' editor contents to file."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

fn draw_default_help<'screen>() -> Paragraph<'screen> {
    let (msg, style) = (
        vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit, "),
            Span::styled("#", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to change tabs."),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

fn draw_editor<'screen>(app: &App) -> Paragraph<'screen> {
    Paragraph::new(app.editor.input.combine_visible_lines())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("SQL Editor"))
}

fn draw_cursor<B: Backend>(app: &App, f: &mut Frame<B>, chunks: &[Rect]) {
    if let InputMode::Editing = app.input_mode {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[2].x + app.editor.get_cursor_column() + 1,
            // Move one line down, from the border to the input line
            chunks[2].y + app.editor.get_cursor_row() + 1,
        )
    }
}

fn draw_query_results(app: &App) -> Paragraph {
    // Query results not shown correctly on error. For example `show tables for x`
    let (query_results, duration) = match &app.query_results {
        Some(query_results) => {
            // debug!("Query results available");
            let query_meta = app.editor.history.last().unwrap();
            let results = if query_meta.query.starts_with("CREATE") {
                Paragraph::new(String::from("Table created"))
            } else {
                Paragraph::new(query_results.pretty_batches.as_str())
                    .scroll((query_results.scroll.x, query_results.scroll.y))
            };
            let query_duration_info = query_results.format_timing_info();
            (results, query_duration_info)
        }
        None => {
            // debug!("Query results not available");
            let last_query = &app.editor.history.last();
            let no_queries_text = match last_query {
                Some(query_meta) => {
                    if let Some(err) = query_meta.error.clone() {
                        Paragraph::new(err)
                    } else {
                        Paragraph::new(query_meta.query.clone())
                    }
                }
                None => Paragraph::new("No queries yet"),
            };
            (no_queries_text, String::new())
        }
    };

    let title = format!("Query Results {}", duration);
    query_results.block(Block::default().borders(Borders::TOP).title(title))
}

fn draw_tabs<'screen>(app: &App) -> Tabs<'screen> {
    let titles = TabItem::all_values()
        .iter()
        .map(|tab| tab.title_with_key())
        .map(|t| Spans::from(vec![Span::styled(t, Style::default())]))
        .collect();

    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tab_item.list_index())
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
}

fn draw_query_history<'screen>(app: &App) -> List<'screen> {
    let messages: Vec<ListItem> = app
        .editor
        .history
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![
                Spans::from(Span::raw(format!(
                    "Query {} [ {} rows took {:.3} seconds ]",
                    i, m.rows, m.query_duration
                ))),
                Spans::from(Span::raw(m.query.clone())),
                Spans::from(Span::raw(String::new())),
            ];
            ListItem::new(content)
        })
        .collect();

    List::new(messages).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Query History"),
    )
}

fn draw_logs(app: &App) -> TuiLoggerSmartWidget {
    TuiLoggerSmartWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        // .block(
        //     Block::default()
        //         .title("Logs")
        //         .border_style(Style::default())
        //         .borders(Borders::ALL),
        // )
        .state(&app.logs.state)
}

fn draw_context<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let context = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let exec_config = draw_execution_config(app);
    f.render_widget(exec_config, context[0]);

    let physical_opts = draw_physical_optimizers(app);
    f.render_widget(physical_opts, context[1]);
}

fn draw_execution_config<'screen>(app: &App) -> List<'screen> {
    let exec_config = app.context.format_execution_config().unwrap();
    let config: Vec<ListItem> = exec_config
        .iter()
        .map(|i| {
            let content = vec![Spans::from(Span::raw(i.to_string()))];
            ListItem::new(content)
        })
        .collect();

    List::new(config).block(
        Block::default()
            .borders(Borders::ALL)
            .title("ExecutionConfig"),
    )
}

fn draw_physical_optimizers<'screen>(app: &App) -> List<'screen> {
    let physical_optimizers = app.context.format_physical_optimizers().unwrap();
    let opts: Vec<ListItem> = physical_optimizers
        .iter()
        .map(|i| {
            let content = vec![Spans::from(Span::raw(i.to_string()))];
            ListItem::new(content)
        })
        .collect();

    List::new(opts).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Physical Optimizers"),
    )
}
