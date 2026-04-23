//! # [Ratatui] `List` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListState, Paragraph};
use serde::Deserialize;
use tokio::runtime::Runtime;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Debug, Deserialize)]
pub struct SurahResponse {
    pub code: i32,
    pub status: String,
    pub data: Vec<SurahDetail>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SurahDetail {
    pub number: i32,
    pub name: String,
    pub english_name: String,
    pub english_name_translation: String,
    pub number_of_ayahs: i32,
    pub revelation_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AyahResponse {
    pub code: i32,
    pub status: String,
    pub data: AyahDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AyahDetail {
    pub number: i32,
    pub name: String,
    pub english_name: String,
    pub english_name_translation: String,
    pub number_of_ayahs: i32,
    pub revelation_type: String,
    pub ayahs: Vec<AyahsList>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AyahsList {
    pub number: i32,
    pub text: String,
    pub number_in_surah: i32,
    pub juz: i32,
    pub manzil: i32,
    pub page: i32,
    pub ruku: i32,
    pub hizb_quarter: i32,
    pub sajda: bool,
}

enum FocusMode {
    SURAH,
    AYAH,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let rt: Runtime = tokio::runtime::Runtime::new()?;

    let mut list_state_surah = ListState::default().with_selected(Some(0));
    let mut list_state_ayah = ListState::default().with_selected(Some(0));
    let mut input: Input = Input::new("".to_string());
    let mut list_surah: Vec<SurahDetail> = Vec::new();
    let mut list_ayah: Vec<AyahsList> = Vec::new();
    let mut focus_mode: FocusMode = FocusMode::SURAH;

    rt.block_on(get_ayah(&mut list_surah));

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                render(
                    frame,
                    &mut list_state_surah,
                    &mut list_state_ayah,
                    &mut input,
                    &mut list_surah,
                    &mut list_ayah,
                    &focus_mode,
                )
            })?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Left => {
                        focus_mode = FocusMode::SURAH;
                    }
                    KeyCode::Right => {
                        focus_mode = FocusMode::AYAH;
                    }
                    KeyCode::Down => match focus_mode {
                        FocusMode::SURAH => {
                            list_state_surah.select_next();
                        }
                        FocusMode::AYAH => {
                            list_state_ayah.select_next();
                        }
                    },
                    KeyCode::Up => match focus_mode {
                        FocusMode::SURAH => {
                            list_state_surah.select_previous();
                        }
                        FocusMode::AYAH => {
                            list_state_ayah.select_previous();
                        }
                    },
                    KeyCode::Enter => match focus_mode {
                        FocusMode::SURAH => {
                            handle_event_enter(
                                &rt,
                                &mut input,
                                &mut list_state_surah,
                                &mut list_surah,
                                &mut list_ayah,
                            );
                        }
                        FocusMode::AYAH => {}
                    },
                    KeyCode::Esc => break Ok(()),
                    _ => match focus_mode {
                        FocusMode::SURAH => {
                            input.handle_event(&Event::Key(key));
                            list_state_ayah.select_first();
                        }
                        FocusMode::AYAH => {}
                    },
                }
            }
        }
    })
}

/// Render the UI with various lists.
fn render(
    frame: &mut Frame,
    list_state_surah: &mut ListState,
    list_state_ayah: &mut ListState,
    input: &mut Input,
    list_surah: &mut Vec<SurahDetail>,
    list_ayah: &mut Vec<AyahsList>,
    focus_mode: &FocusMode,
) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [title_area, search_area, content_area] = frame.area().layout(&layout);

    render_title(frame, title_area);
    render_input_search(frame, search_area, input);
    render_content(
        frame,
        content_area,
        list_state_surah,
        list_state_ayah,
        list_surah,
        input,
        list_ayah,
        focus_mode,
    );
}

pub fn render_title(frame: &mut Frame, area: Rect) {
    let title = Line::from_iter([Span::from("Al Quran").bold()]);
    frame.render_widget(title.centered(), area);
}

/// Render a list.
pub fn render_input_search(frame: &mut Frame, area: Rect, input: &mut Input) {
    let width = area.width.max(3) - 3;
    let scroll = input.visual_scroll(width as usize);

    let input_block = Paragraph::new(input.value())
        .style(Style::default().yellow())
        .scroll((0, scroll as u16))
        .block(Block::bordered().title("Search Surah"));

    frame.render_widget(input_block, area);
}

pub fn render_content(
    frame: &mut Frame,
    area: Rect,
    list_state_surah: &mut ListState,
    list_state_ayah: &mut ListState,
    list_surah: &mut Vec<SurahDetail>,
    input: &mut Input,
    list_ayah: &mut Vec<AyahsList>,
    focus_mode: &FocusMode,
) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(3)]);
    let [surah_area, ayah_area] = area.layout(&layout);

    let is_focus_surah = matches!(focus_mode, FocusMode::SURAH);

    render_surah(
        frame,
        surah_area,
        list_state_surah,
        list_surah,
        input,
        is_focus_surah,
    );
    render_ayah(
        frame,
        ayah_area,
        list_state_ayah,
        list_ayah,
        !is_focus_surah,
    );
}

pub fn render_surah(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    list_surah: &mut Vec<SurahDetail>,
    input: &mut Input,
    is_focus: bool,
) {
    let input_lowercase = input.value().to_lowercase().to_string();
    let items_filtered: Vec<String> = list_surah
        .iter()
        .filter(|x| x.english_name.to_lowercase().contains(&input_lowercase))
        .map(|s| format!("{}. {}", s.number, s.english_name))
        .collect();

    let list = List::new(items_filtered)
        .block(Block::bordered().title("Surah"))
        .style(if is_focus {
            Color::Yellow
        } else {
            Color::White
        })
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

pub fn render_ayah(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    list_ayah: &mut Vec<AyahsList>,
    isOnFocus: bool,
) {
    let list_ayah_string: Vec<String> = list_ayah
        .iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i + 1, s.text.clone()))
        .collect();
    let list = List::new(list_ayah_string)
        .block(Block::bordered().title("Ayah"))
        .style(if isOnFocus {
            Color::Yellow
        } else {
            Color::White
        })
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

pub async fn get_ayah(list: &mut Vec<SurahDetail>) {
    match reqwest::get("http://api.alquran.cloud/v1/surah").await {
        Ok(resp) => match resp.json::<SurahResponse>().await {
            Ok(data) => {
                list.clear();
                list.extend(data.data);
            }
            Err(e) => {
                // eprintln!("JSON error: {}", e);
            }
        },
        Err(e) => {
            // eprintln!("Request error: {}", e);
        }
    }
}

pub async fn get_ayah_detail(surah_id: i32, list: &mut Vec<AyahsList>) {
    let url = format!("http://api.alquran.cloud/v1/surah/{}", surah_id);
    match reqwest::get(url).await {
        Ok(resp) => match resp.json::<AyahResponse>().await {
            Ok(data) => {
                list.clear();
                list.extend(data.data.ayahs);
            }
            Err(e) => {
                // eprintln!("JSON error: {}", e);
            }
        },
        Err(e) => {
            // eprintln!("Request error: {}", e);
        }
    }
}

pub fn handle_event_enter(
    rt: &Runtime,
    input: &Input,
    list_state: &ListState,
    list_surah: &Vec<SurahDetail>,
    list_ayah: &mut Vec<AyahsList>,
) {
    let input_lowercase = input.value().to_lowercase().to_string();
    let items_filtered: Vec<&SurahDetail> = list_surah
        .iter()
        .filter(|x| x.english_name.to_lowercase().contains(&input_lowercase))
        .map(|s| s)
        .collect();

    let selected_item = list_state.selected().and_then(|i| items_filtered.get(i));

    if let Some(selected_item) = selected_item {
        rt.block_on(get_ayah_detail(selected_item.number, list_ayah));
    }
}
