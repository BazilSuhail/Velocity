//! Rendering only. `draw` is a pure function of `App`: it never mutates state or touches I/O.

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Cell, Clear, Paragraph, Row, Table, TableState, Wrap},
};

use crate::platform::{truecolor_supported, unicode_supported};
use crate::tui::app::{ACTIONS, AddStage, App, Mode, StatusKind};
use crate::tui::keymap;
use crate::tui::theme::{self, Theme};

const MIN_WIDTH: u16 = 30;
const MIN_HEIGHT: u16 = 8;
const BANNER_MIN_WIDTH: u16 = 60;
const BANNER_MIN_HEIGHT: u16 = 20;
const DETAIL_MIN_WIDTH: u16 = 100;
const UPDATED_MIN_WIDTH: u16 = 60;
const UPDATED_W: usize = 16;
const LABEL_WIDTH: u16 = 10;
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The VELO wordmark in block glyphs.
const BANNER: [&str; 6] = [
    "      ██╗   ██╗███████╗██╗     ██████╗ ",
    "      ██║   ██║██╔════╝██║    ██╔═══██╗",
    "      ██║   ██║█████╗  ██║    ██║   ██║",
    "      ╚██╗ ██╔╝██╔══╝  ██║    ██║   ██║",
    "       ╚████╔╝ ███████╗███████╗╚██████╔╝",
    "        ╚═══╝  ╚══════╝╚══════╝ ╚═════╝ ",
];
/// Gradient from red to yellow, one colour per banner row (truecolor terminals only).
const BANNER_GRADIENT: [Color; 6] = [
    Color::Rgb(255, 50, 50),
    Color::Rgb(255, 90, 30),
    Color::Rgb(255, 130, 10),
    Color::Rgb(255, 160, 0),
    Color::Rgb(255, 190, 0),
    Color::Rgb(255, 210, 0),
];
/// The same wordmark drawn with `#`, for terminals without Unicode.
const ASCII_LETTERS: [[&str; 5]; 4] = [
    ["#   #", "#   #", "#   #", " # # ", "  #  "],
    ["#####", "#    ", "#### ", "#    ", "#####"],
    ["#    ", "#    ", "#    ", "#    ", "#####"],
    [" ### ", "#   #", "#   #", "#   #", " ### "],
];

/// Per-frame context: avoids re-deriving theme, capabilities and visible keys in every helper.
struct Ctx<'a> {
    app: &'a App,
    t: &'static Theme,
    unicode: bool,
    truecolor: bool,
    keys: Vec<String>,
}

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let t = theme::current();
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        let row = Rect::new(
            area.x,
            area.y + area.height / 2,
            area.width,
            area.height.min(1),
        );
        let msg = format!("Terminal too small (need {MIN_WIDTH}x{MIN_HEIGHT})");
        frame.render_widget(Paragraph::new(Line::styled(msg, t.error)).centered(), row);
        return;
    }

    let cx = Ctx {
        app,
        t,
        unicode: unicode_supported(),
        truecolor: truecolor_supported(),
        keys: app.visible_keys(),
    };
    let full_header = area.width >= BANNER_MIN_WIDTH && area.height >= BANNER_MIN_HEIGHT;
    let header_h = if full_header {
        banner_rows(cx.unicode).len() as u16 + 1
    } else {
        1
    };

    let [header, body, status, footer] = Layout::vertical([
        Constraint::Length(header_h),
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(area);

    draw_header(frame, header, &cx, full_header);
    match &app.load_error {
        Some(err) => draw_error(frame, body, &cx, err),
        None if app.store.macros.is_empty() => draw_empty(frame, body, &cx),
        None => draw_browse(frame, body, &cx),
    }
    if app.load_error.is_none() {
        draw_overlay(frame, body, &cx);
    }
    draw_status(frame, status, &cx);
    frame.render_widget(
        Paragraph::new(hint_line(keymap::hints(&app.mode), footer.width, &cx)),
        footer,
    );
}

/// Banner rows for the current glyph set.
fn banner_rows(unicode: bool) -> Vec<String> {
    if unicode {
        BANNER.iter().map(|row| row.to_string()).collect()
    } else {
        (0..5)
            .map(|row| {
                ASCII_LETTERS
                    .iter()
                    .map(|letter| letter[row])
                    .collect::<Vec<_>>()
                    .join("  ")
            })
            .collect()
    }
}

fn draw_header(frame: &mut Frame, area: Rect, cx: &Ctx, full: bool) {
    let count = cx.app.store.macros.len();
    if !full {
        let line = Line::from(vec![
            Span::styled("VELO", cx.t.accent.add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("  v{VERSION}  |  Universal Command Macro Engine  |  {count} macros"),
                cx.t.muted,
            ),
        ]);
        frame.render_widget(Paragraph::new(line), area);
        return;
    }

    let rows = banner_rows(cx.unicode);
    let [banner, subtitle] =
        Layout::vertical([Constraint::Length(rows.len() as u16), Constraint::Length(1)])
            .areas(area);

    let lines: Vec<Line> = rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let style = if cx.truecolor && cx.unicode {
                Style::default()
                    .fg(BANNER_GRADIENT[i.min(BANNER_GRADIENT.len() - 1)])
                    .add_modifier(Modifier::BOLD)
            } else {
                cx.t.accent.add_modifier(Modifier::BOLD)
            };
            Line::styled(row, style)
        })
        .collect();
    frame.render_widget(Paragraph::new(lines), banner);

    let sub = Line::from(vec![
        Span::styled("  Universal Command Macro Engine", cx.t.text),
        Span::styled(
            format!("  |  v{VERSION}  |  {count} macros  |  by Bazil Suhail"),
            cx.t.muted,
        ),
    ]);
    frame.render_widget(Paragraph::new(sub), subtitle);
}

fn draw_error(frame: &mut Frame, area: Rect, cx: &Ctx, err: &str) {
    let lines = vec![
        Line::styled(err.to_string(), cx.t.error),
        Line::from(""),
        Line::styled(
            "Fix or move the file, then restart velo. Press q to quit.",
            cx.t.text,
        ),
    ];
    let block = panel(" Could not load macros ", cx).border_style(cx.t.error);
    frame.render_widget(
        Paragraph::new(lines).block(block).wrap(Wrap { trim: true }),
        area,
    );
}

fn draw_empty(frame: &mut Frame, area: Rect, cx: &Ctx) {
    let block = panel(" Macros (0) ", cx);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::styled("No macros yet.", cx.t.text.add_modifier(Modifier::BOLD)),
        Line::styled(
            "Press n to create one, or run: velo add <KEY> <COMMAND>",
            cx.t.muted,
        ),
    ];
    let rect = centered(inner, inner.width, lines.len() as u16);
    frame.render_widget(
        Paragraph::new(lines).centered().wrap(Wrap { trim: true }),
        rect,
    );
}

fn table_title(cx: &Ctx) -> String {
    let total = cx.app.store.macros.len();
    if cx.app.query.is_empty() {
        format!(" Macros ({total}) ")
    } else {
        format!(" Macros {}/{total} ", cx.keys.len())
    }
}

fn draw_browse(frame: &mut Frame, area: Rect, cx: &Ctx) {
    if cx.keys.is_empty() {
        let block = panel(table_title(cx), cx);
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let text = format!("No macros match '{}'. Esc clears the filter.", cx.app.query);
        let rect = centered(inner, inner.width, 1);
        frame.render_widget(
            Paragraph::new(Line::styled(text, cx.t.muted)).centered(),
            rect,
        );
        return;
    }

    let selected = cx.app.selected.min(cx.keys.len() - 1);
    if area.width >= DETAIL_MIN_WIDTH {
        let [table_area, detail_area] =
            Layout::horizontal([Constraint::Percentage(62), Constraint::Percentage(38)])
                .areas(area);
        draw_table(frame, table_area, cx, selected);
        draw_detail(frame, detail_area, cx, &cx.keys[selected]);
    } else {
        draw_table(frame, area, cx, selected);
    }
}

/// KEY, COMMAND and (when wide enough) UPDATED. The command column takes whatever is left.
fn draw_table(frame: &mut Frame, area: Rect, cx: &Ctx, selected: usize) {
    let show_updated = area.width >= UPDATED_MIN_WIDTH;
    let key_w = cx
        .keys
        .iter()
        .map(|k| k.chars().count())
        .max()
        .unwrap_or(3)
        .clamp(3, 20);
    let columns = if show_updated { 3 } else { 2 };
    // Borders take 2 cells, the highlight marker 2, and each gap between columns 2.
    let fixed = 4 + key_w + if show_updated { UPDATED_W } else { 0 } + 2 * (columns - 1);
    let cmd_w = (area.width as usize).saturating_sub(fixed).max(1);

    let rows = cx.keys.iter().map(|key| {
        let entry = cx.app.store.macros.get(key);
        let command = entry.map_or("", |e| e.command.as_str());
        let mut cells = vec![
            Cell::new(truncate(key, key_w, cx.unicode)).style(cx.t.accent),
            Cell::new(truncate(command, cmd_w, cx.unicode)).style(cx.t.text),
        ];
        if show_updated {
            let updated = entry
                .map(|e| e.updated_at.chars().take(UPDATED_W).collect::<String>())
                .unwrap_or_default();
            cells.push(Cell::new(updated).style(cx.t.muted));
        }
        Row::new(cells)
    });

    let mut headers = vec!["KEY", "COMMAND"];
    let mut widths = vec![Constraint::Length(key_w as u16), Constraint::Min(0)];
    if show_updated {
        headers.push("UPDATED");
        widths.push(Constraint::Length(UPDATED_W as u16));
    }

    let table = Table::new(rows, widths)
        .header(Row::new(headers).style(cx.t.accent.add_modifier(Modifier::BOLD)))
        .block(panel(table_title(cx), cx))
        .column_spacing(2)
        .row_highlight_style(cx.t.selected)
        .highlight_symbol(if cx.unicode { "▸ " } else { "> " });
    let mut state = TableState::new().with_selected(Some(selected));
    frame.render_stateful_widget(table, area, &mut state);
}

fn draw_detail(frame: &mut Frame, area: Rect, cx: &Ctx, key: &str) {
    let mut lines = vec![
        Line::styled(key.to_string(), cx.t.accent.add_modifier(Modifier::BOLD)),
        Line::from(""),
    ];
    if let Some(entry) = cx.app.store.macros.get(key) {
        lines.extend(
            entry
                .command
                .lines()
                .map(|l| Line::styled(l.to_string(), cx.t.text)),
        );
        lines.push(Line::from(""));
        lines.push(Line::styled(
            format!("Created  {}", entry.created_at),
            cx.t.muted,
        ));
        lines.push(Line::styled(
            format!("Updated  {}", entry.updated_at),
            cx.t.muted,
        ));
    }
    frame.render_widget(
        Paragraph::new(lines)
            .block(panel(" Detail ", cx))
            .wrap(Wrap { trim: false }),
        area,
    );
}

/// Modal layer for the actions menu, Add / Edit / ConfirmDelete and Help. Drawn over the body only.
fn draw_overlay(frame: &mut Frame, area: Rect, cx: &Ctx) {
    let app = cx.app;
    match &app.mode {
        Mode::Actions => draw_actions(frame, area, cx),
        Mode::Add(AddStage::Key) => draw_form(
            frame,
            area,
            cx,
            " Add macro ",
            &[("Key:", app.input.as_str(), true), ("Command:", "", false)],
        ),
        Mode::Add(AddStage::Command) => draw_form(
            frame,
            area,
            cx,
            " Add macro ",
            &[
                ("Key:", app.draft_key.as_str(), false),
                ("Command:", app.input.as_str(), true),
            ],
        ),
        Mode::Edit => {
            let key = app.draft_key.as_str();
            draw_form(
                frame,
                area,
                cx,
                " Edit macro ",
                &[("Key:", key, false), ("Command:", app.input.as_str(), true)],
            )
        }
        Mode::ConfirmDelete => draw_confirm_delete(frame, area, cx),
        Mode::Help => draw_help(frame, area, cx),
        Mode::Browse | Mode::Filter => {}
    }
}

/// The actions menu opened with Enter on a macro.
fn draw_actions(frame: &mut Frame, area: Rect, cx: &Ctx) {
    let key = cx.app.selected_key().unwrap_or_default();
    let rect = centered(area, 40, ACTIONS.len() as u16 + 3);
    frame.render_widget(Clear, rect);
    let block = panel(format!(" {key} "), cx);
    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let [list, hint] = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(inner);
    let marker = if cx.unicode { "▸ " } else { "> " };
    let items: Vec<Line> = ACTIONS
        .iter()
        .enumerate()
        .map(|(i, label)| {
            if i == cx.app.action {
                Line::styled(format!("{marker}{label}"), cx.t.selected)
            } else {
                Line::styled(format!("  {label}"), cx.t.text)
            }
        })
        .collect();
    frame.render_widget(Paragraph::new(items), list);
    frame.render_widget(
        Paragraph::new(hint_line(keymap::hints(&Mode::Actions), hint.width, cx)),
        hint,
    );
}

/// Each field is (label, value, is_active). The active field gets the terminal cursor at its end.
fn draw_form(frame: &mut Frame, area: Rect, cx: &Ctx, title: &str, fields: &[(&str, &str, bool)]) {
    let rect = centered(area, 64, fields.len() as u16 + 4);
    frame.render_widget(Clear, rect);
    let block = panel(title, cx);
    let inner = block.inner(rect);
    frame.render_widget(block, rect);

    let mut lines: Vec<Line> = fields
        .iter()
        .map(|(label, value, active)| {
            let value_style = if *active { cx.t.text } else { cx.t.muted };
            Line::from(vec![
                Span::styled(format!("{label:<w$}", w = LABEL_WIDTH as usize), cx.t.muted),
                Span::styled(value.to_string(), value_style),
            ])
        })
        .collect();
    lines.push(Line::from(""));
    lines.push(hint_line(keymap::hints(&cx.app.mode), inner.width, cx));
    frame.render_widget(Paragraph::new(lines), inner);

    if let Some(row) = fields.iter().position(|(_, _, active)| *active) {
        let (_, value, _) = fields[row];
        let x = inner.x + LABEL_WIDTH + value.chars().count() as u16;
        let x = x.min(inner.right().saturating_sub(1));
        frame.set_cursor_position((x, inner.y + row as u16));
    }
}

fn draw_confirm_delete(frame: &mut Frame, area: Rect, cx: &Ctx) {
    let key = cx.app.selected_key().unwrap_or_default();
    let rect = centered(area, 64, 3);
    frame.render_widget(Clear, rect);
    let block = panel(" Confirm delete ", cx).border_style(cx.t.warning);
    let text = Line::styled(format!("Delete '{key}'? y/N"), cx.t.warning);
    frame.render_widget(Paragraph::new(text).block(block), rect);
}

fn draw_help(frame: &mut Frame, area: Rect, cx: &Ctx) {
    let mut lines = vec![Line::styled("Keys", cx.t.accent)];
    for (key, action) in keymap::hints(&Mode::Browse) {
        lines.push(Line::from(vec![
            Span::styled(format!("{key:<12}"), cx.t.key_label),
            Span::styled(action.to_string(), cx.t.text),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::styled("Commands", cx.t.accent));
    for cmd in [
        "velo add <key> <command>",
        "velo update <key> <command>",
        "velo delete <key>",
        "velo list",
        "velo <key>            run a macro",
    ] {
        lines.push(Line::styled(cmd, cx.t.text));
    }
    lines.push(Line::from(""));
    lines.push(Line::styled("Press ? or Esc to close", cx.t.muted));

    let rect = centered(area, 64, lines.len() as u16 + 2);
    frame.render_widget(Clear, rect);
    frame.render_widget(Paragraph::new(lines).block(panel(" Help ", cx)), rect);
}

/// Bottom-most line: status or filter prompt on the left, macro count on the right.
fn draw_status(frame: &mut Frame, area: Rect, cx: &Ctx) {
    let app = cx.app;
    let count = Line::styled(
        format!("{}/{} macros", cx.keys.len(), app.store.macros.len()),
        cx.t.muted,
    );
    let count_w = count.width().min(area.width as usize) as u16;
    let [left, right] =
        Layout::horizontal([Constraint::Min(0), Constraint::Length(count_w)]).areas(area);

    if app.mode == Mode::Filter {
        let prompt = format!("/{}", app.query);
        frame.render_widget(
            Paragraph::new(Line::styled(prompt.clone(), cx.t.text)),
            left,
        );
        let x = left.x + prompt.chars().count() as u16;
        frame.set_cursor_position((x.min(left.right().saturating_sub(1)), left.y));
    } else {
        let line = match &app.status {
            Some(status) => {
                let style = match status.kind {
                    StatusKind::Ok => cx.t.success,
                    StatusKind::Error => cx.t.error,
                };
                Line::styled(status.text.clone(), style)
            }
            None => Line::styled(
                mode_label(app.mode),
                cx.t.accent_dim.add_modifier(Modifier::BOLD),
            ),
        };
        frame.render_widget(Paragraph::new(line), left);
    }
    frame.render_widget(Paragraph::new(count), right);
}

fn mode_label(mode: Mode) -> &'static str {
    match mode {
        Mode::Browse => "BROWSE",
        Mode::Actions => "ACTIONS",
        Mode::Filter => "FILTER",
        Mode::Add(_) => "ADD",
        Mode::Edit => "EDIT",
        Mode::ConfirmDelete => "DELETE",
        Mode::Help => "HELP",
    }
}

/// "key action" pairs separated by two spaces; ends with an ellipsis when the width runs out.
fn hint_line(hints: &[(&'static str, &'static str)], width: u16, cx: &Ctx) -> Line<'static> {
    let max = width as usize;
    let ellipsis = ellipsis(cx.unicode);
    let mut spans = Vec::new();
    let mut used = 0;
    for (i, (key, action)) in hints.iter().enumerate() {
        let sep = if i == 0 { 0 } else { 2 };
        let item = sep + key.chars().count() + 1 + action.chars().count();
        if used + item > max {
            if used + ellipsis.chars().count() <= max {
                spans.push(Span::styled(ellipsis, cx.t.muted));
            }
            break;
        }
        if sep > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(*key, cx.t.key_label));
        spans.push(Span::styled(format!(" {action}"), cx.t.muted));
        used += item;
    }
    Line::from(spans)
}

fn ellipsis(unicode: bool) -> &'static str {
    if unicode { "…" } else { "..." }
}

/// Char-count truncation (not bytes); control characters become spaces so a row stays one line.
fn truncate(s: &str, max: usize, unicode: bool) -> String {
    let flat: String = s
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    if flat.chars().count() <= max {
        return flat;
    }
    let ellipsis = ellipsis(unicode);
    let ellipsis_len = ellipsis.chars().count();
    if max <= ellipsis_len {
        return flat.chars().take(max).collect();
    }
    let mut out: String = flat.chars().take(max - ellipsis_len).collect();
    out.push_str(ellipsis);
    out
}

fn panel(title: impl Into<String>, cx: &Ctx) -> Block<'static> {
    Block::bordered()
        .title(title.into())
        .border_set(theme::border_set())
        .border_style(cx.t.border)
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}
