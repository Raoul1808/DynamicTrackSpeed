use core::fmt;
use std::path::PathBuf;

use iced::{
    widget::{button, column, combo_box, container, radio, row, text},
    Alignment, Length, Sandbox, Settings, Size,
};
use rfd::FileDialog;

pub fn program_flow() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = Size::new(360., 400.);
    App::run(settings)
}

pub struct App {
    diff_state: combo_box::State<Difficulty>,
    selected_chart: Option<PathBuf>,
    selected_action: Option<Action>,
    selected_difficulty: Option<Difficulty>,
    selected_speeds: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Integrate,
    Extract,
    Remove,
}

#[derive(Debug, Clone)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    XD,
    RemiXD,
    Legacy,
}

impl Difficulty {
    const ALL: [Difficulty; 7] = [
        Difficulty::Easy,
        Difficulty::Normal,
        Difficulty::Hard,
        Difficulty::Expert,
        Difficulty::XD,
        Difficulty::RemiXD,
        Difficulty::Legacy,
    ];
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Normal => write!(f, "Normal"),
            Difficulty::Hard => write!(f, "Hard"),
            Difficulty::Expert => write!(f, "Expert"),
            Difficulty::XD => write!(f, "XD"),
            Difficulty::RemiXD => write!(f, "RemiXD"),
            Difficulty::Legacy => write!(f, "All (Legacy)"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectChart,
    SelectAction(Action),
    SelectDifficulty(Difficulty),
    SelectSpeeds,
    ProcessAndSave,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            diff_state: combo_box::State::new(Difficulty::ALL.to_vec()),
            selected_chart: None,
            selected_action: None,
            selected_difficulty: None,
            selected_speeds: None,
        }
    }

    fn title(&self) -> String {
        String::from("SRTB Integration Program")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectChart => {
                println!("Selecting chart");
                self.selected_chart = FileDialog::new()
                    .add_filter("Spin Rhythm Track Bundle", &["srtb"])
                    .pick_file();
                println!("Selected: {:?}", self.selected_chart);
            }
            Message::SelectAction(action) => {
                self.selected_action = Some(action);
            }
            Message::SelectDifficulty(difficulty) => {
                self.selected_difficulty = Some(difficulty);
            }
            Message::SelectSpeeds => {
                self.selected_speeds = FileDialog::new()
                    .add_filter("Speed Triggers File", &["speeds"])
                    .pick_file();
            }
            Message::ProcessAndSave => {
                unimplemented!()
            }
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let chart_selected = self
            .selected_chart
            .clone()
            .map(|c| {
                c.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or("None".into());
        let chart_select_label = text(&format!("Chart Selected: {}", chart_selected));
        let chart_select_button = button("Browse").on_press(Message::SelectChart);
        let select_row = row![chart_select_label, chart_select_button]
            .spacing(10)
            .align_items(Alignment::Start);

        let diff_combo_box = combo_box(
            &self.diff_state,
            "Difficulty...",
            self.selected_difficulty.as_ref(),
            Message::SelectDifficulty,
        )
        .width(128);
        let diff_label = text("Target difficulty:");
        let difficulty_row = row![diff_label, diff_combo_box]
            .align_items(iced::Alignment::Center)
            .spacing(10)
            .align_items(Alignment::Center);

        let radio_integrate = radio(
            "Integrate",
            Action::Integrate,
            self.selected_action,
            Message::SelectAction,
        );
        let radio_extract = radio(
            "Extract",
            Action::Extract,
            self.selected_action,
            Message::SelectAction,
        );
        let radio_remove = radio(
            "Remove",
            Action::Remove,
            self.selected_action,
            Message::SelectAction,
        );
        let radio_action = column![radio_integrate, radio_extract, radio_remove]
            .spacing(10)
            .align_items(Alignment::Start);

        let speeds_enabled = self.selected_action.is_some_and(|a| a == Action::Integrate);
        let speeds_selected = self
            .selected_speeds
            .clone()
            .map(|s| {
                s.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or("None".into());
        let speeds_label = text(format!("Selected speeds: {}", speeds_selected));
        let speeds_button =
            button("Browse").on_press_maybe(speeds_enabled.then_some(Message::SelectSpeeds));
        let speeds_row = row![speeds_label, speeds_button]
            .spacing(10)
            .align_items(Alignment::Center);

        let process_enabled = match speeds_enabled {
            true => {
                self.selected_chart.is_some()
                    && self.selected_speeds.is_some()
                    && self.selected_difficulty.is_some()
            }
            false => {
                self.selected_chart.is_some()
                    && self.selected_action.is_some()
                    && self.selected_difficulty.is_some()
            }
        };
        let process_button = button("PROCESS & SAVE")
            .on_press_maybe(process_enabled.then_some(Message::ProcessAndSave));

        let content_col = column![
            select_row,
            difficulty_row,
            radio_action,
            speeds_row,
            process_button
        ]
        .spacing(20)
        .align_items(Alignment::Center);
        container(content_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
