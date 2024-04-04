use iced::{executor, Application, Command, Settings, Theme};

pub fn program_flow() -> iced::Result {
    App::run(Settings::default())
}

pub struct App;

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (App, Command::none())
    }

    fn title(&self) -> String {
        String::from("SRTB Integration Program")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        "wow!!!".into()
    }
}
