use ciphered::{CipheredMessage, CipheredView};
use iced::{
    Alignment, Length, Task,
    widget::{row, text},
};
use key_choose::{KeyChooseMessage, KeyChooseView};

const ERR_VIGENERE: &str = "Key must only be ascii chars";
const ERR_VIEWINVALID: &str = "This view couldn't be created here";

mod ciphered;
mod key_choose;

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
    Ciphered(CipheredView),
    Error(&'static str),
}

#[derive(Debug)]
enum MainMessage {
    KeyChoose(KeyChooseMessage),
    Ciphered(CipheredMessage),
    Error(&'static str),
    CipherEnded(CipheredView),
}

impl Main {
    fn update(&mut self, msg: MainMessage) -> Task<MainMessage> {
        match msg {
            MainMessage::KeyChoose(msg) => {
                let Main::KeyChoose(view) = self else {
                    return Task::done(MainMessage::Error(ERR_VIEWINVALID));
                };

                return view.update(msg);
            }

            MainMessage::Ciphered(msg) => {
                let Main::Ciphered(view) = self else {
                    return Task::done(MainMessage::Error(ERR_VIEWINVALID));
                };

                return view.update(msg);
            }
            MainMessage::Error(err) => *self = Main::Error(err),
            MainMessage::CipherEnded(ciphered_view) => {
                // *self = Main::Ciphered(ciphered_view)
            }
        }

        Task::none()
    }

    fn view(&self) -> iced::Element<'_, MainMessage> {
        match self {
            Main::KeyChoose(view) => view.view().map(MainMessage::KeyChoose),
            Main::Ciphered(view) => view.view().map(MainMessage::Ciphered),
            Main::Error(errmsg) => row![
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
