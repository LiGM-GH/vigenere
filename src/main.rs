use iced::{
    widget::{button, column, horizontal_space, row, text_input, Column, Row}, Alignment, Length
};

fn main() -> Result<(), MainError> {
    iced::application("calc_task", Main::update, Main::view).run_with(
        || {
            (
                Main {
                    message: String::new(),
                },
                iced::Task::none(),
            )
        },
    )?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error("Iced stopped with error: {0:?}")]
    Iced(#[from] iced::Error),
}

struct Main {
    message: String,
}

#[derive(Debug, Clone)]
enum MainMessage {
    TextboxInput(String),
    ButtonPressed,
}

impl Main {
    fn update(&mut self, msg: MainMessage) {
        match msg {
            MainMessage::TextboxInput(thing) => self.message = thing,
            MainMessage::ButtonPressed => {}
        }
    }

    fn view(&self) -> Row<MainMessage> {
        row![self.content(),]
            .height(Length::Fill)
            .align_y(Alignment::Center)
    }

    fn content(&self) -> Column<MainMessage> {
        let textbox = row![
            horizontal_space().width(Length::FillPortion(1)),
            text_input("Input your message", &self.message)
                .width(Length::FillPortion(2))
                .on_input(MainMessage::TextboxInput),
            horizontal_space().width(Length::FillPortion(1)),
        ];

        column![
            button("This is working").on_press(MainMessage::ButtonPressed),
            textbox,
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
    }
}
