use iced::{Element, Task};

use crate::MainMessage;

#[derive(Debug)]
pub struct CipheredView {
}

#[derive(Debug)]
pub enum CipheredMessage {}

impl CipheredView {
    pub fn view(&self) -> Element<'_, CipheredMessage> {
        iced::widget::text("Whatever").into()
    }

    pub fn update(&mut self, msg: CipheredMessage) -> Task<MainMessage> {
        Task::none()
    }
}
