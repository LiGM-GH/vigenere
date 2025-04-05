#![warn(
    clippy::all,
    clippy::complexity,
    clippy::missing_const_for_fn,
    clippy::mod_module_files,
    clippy::nursery,
    clippy::perf,
    clippy::pedantic,
    clippy::style,
    clippy::suspicious
)]

use iced::{
    Alignment, Length, Task,
    widget::{row, text},
};
use key_choose::{KeyChooseMessage, KeyChooseView};

mod key_choose;

const ERR_VIEWINVALID: &str = "This view couldn't be created here";

fn main() -> Result<(), MainError> {
    iced::application("calc_task", Main::update, Main::view)
        .run_with(|| (Main::KeyChoose(KeyChooseView::new()), Task::none()))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error("Iced stopped with error: {0:?}")]
    Iced(#[from] iced::Error),
}

#[derive(Debug)]
enum Main {
    KeyChoose(KeyChooseView),
    Error(&'static str),
}

#[derive(Debug)]
enum MainMessage {
    KeyChoose(KeyChooseMessage),
    Error(&'static str),
}

impl Main {
    fn update(&mut self, msg: MainMessage) -> Task<MainMessage> {
        match msg {
            MainMessage::KeyChoose(msg) => {
                let Self::KeyChoose(view) = self else {
                    return Task::done(MainMessage::Error(ERR_VIEWINVALID));
                };

                return view.update(msg);
            }
            MainMessage::Error(err) => *self = Self::Error(err),
        }

        Task::none()
    }

    fn view(&self) -> iced::Element<'_, MainMessage> {
        match self {
            Self::KeyChoose(view) => view.view().map(MainMessage::KeyChoose),
            Self::Error(errmsg) => row![
                iced::widget::column![
                    text(*errmsg)
                        .height(Length::Fill)
                        .align_y(Alignment::Center)
                ]
                .width(Length::Fill)
                .align_x(Alignment::Center)
            ]
            .align_y(Alignment::Center)
            .into(),
        }
    }
}
