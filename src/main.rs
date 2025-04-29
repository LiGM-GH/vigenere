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
    widget::{button, row, text},
};
use iced_aw::{Menu, MenuBar, menu::Item, menu_bar};
use iced_aw::{menu, menu_items};
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
    About,
    Error(&'static str),
}

#[derive(Debug, Clone)]
enum MainMessage {
    KeyChoose(KeyChooseMessage),
    ToggleAbout,
    Error(&'static str),
    None,
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
            MainMessage::ToggleAbout => match self {
                Self::About => *self = Self::KeyChoose(KeyChooseView::new()),
                _ => *self = Self::About,
            },
            MainMessage::None => {}
        }

        Task::none()
    }

    fn view(&self) -> iced::Element<'_, MainMessage> {
        let main_view = match self {
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

            Self::About => {
                iced::widget::row![
                iced::widget::column![iced::widget::text(
                    "Липкин Г.М.\nГруппа А-18-21\nВариант: шифр Виженера"
                ),
                iced::widget::button("Back").on_press(MainMessage::ToggleAbout)]
                .height(Length::Shrink)
                .width(Length::Fill)
                .align_x(Alignment::Center)
            ]
                .height(Length::Fill)
                .width(Length::Fill)
                .align_y(Alignment::Center)
                .into()
            }
        };

        let menu_tpl_1 =
            |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        let menu = menu_bar!((
            button("File").on_press(MainMessage::None),
            menu_tpl_1(menu_items!(
                (button("About").on_press(MainMessage::ToggleAbout))
            ))
        ));

        iced::widget::column![menu, main_view,].into()
    }
}
