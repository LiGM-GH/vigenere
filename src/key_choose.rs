use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

#[cfg(target_os = "windows")]
use iced::window::raw_window_handle::{
    DisplayHandle, RawWindowHandle, Win32WindowHandle, WindowHandle,
    XcbWindowHandle,
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
#[cfg(target_os = "windows")]
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use iced::{
    Alignment, Element, Length, Task,
    widget::{
        self, Column, button, column, horizontal_space, row,
        text_editor::Content, text_input, vertical_space,
    },
};
use itertools::Itertools;
use vigenere_rs::Vigenere;

use crate::MainMessage;

const FILE_DIALOG_NAME: &str = "CHOOSE FILE";
const ERR_BAD_PASSWORD: &str =
    "Password should be only-ASCII-alphabetic and non-empty";

const IDENTIFYING_MESSAGE: &str = "M%S$&#%";

#[derive(Debug)]
pub enum FileOrText {
    File(PathBuf),
    Text(iced::widget::text_editor::Content),
}

impl FileOrText {
    pub fn take(&mut self) -> Self {
        let mut thing = Self::Text(Content::new());

        std::mem::swap(self, &mut thing);

        thing
    }
}

#[derive(Debug)]
pub struct KeyChooseView {
    key: String,
    input: FileOrText,
    output_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum KeyChooseMessage {
    TextboxInput(String),
    InFileChoose,
    OutFileChoose,
    InFileSelected(PathBuf),
    OutFileSelected(PathBuf),
    Cipher,
    Decipher,
    BigtextAction(iced::widget::text_editor::Action),
}

impl KeyChooseView {
    const N_VALUES: usize = 4;

    pub fn new() -> Self {
        Self {
            key: String::new(),
            input: FileOrText::Text(Content::new()),
            output_path: None,
        }
    }

    pub fn task(msg: KeyChooseMessage) -> Task<MainMessage> {
        Task::done(MainMessage::KeyChoose(msg))
    }

    pub fn err(msg: &'static str) -> Task<MainMessage> {
        Task::done(MainMessage::Error(msg))
    }

    pub fn update(&mut self, msg: KeyChooseMessage) -> Task<MainMessage> {
        match msg {
            KeyChooseMessage::TextboxInput(thing) => self.key = thing,
            KeyChooseMessage::InFileChoose => {
                let file = rfd::FileDialog::new()
                    .set_title(FILE_DIALOG_NAME)
                    .pick_file();

                if let Some(file) = file {
                    return Self::task(KeyChooseMessage::InFileSelected(file));
                }

                println!("InFile not selected");
            }
            KeyChooseMessage::InFileSelected(file) => {
                self.input = FileOrText::File(file);
            }
            KeyChooseMessage::OutFileChoose => {
                #[cfg(target_os = "windows")]
                let file = {
                    let hwnd = GetForegroundWindow();
                    let handle =
                        iced::window::raw_window_handle::Win32WindowHandle::new(
                            hwnd,
                        );
                    let handle = unsafe {
                        iced::window::raw_window_handle::WindowHandle::borrow_raw(RawWindowHandle::Win32(handle))
                    };
                    let display = DisplayHandle::windows();

                    struct DisplayAndWinHandle(DisplayHandle, WindowHandle);
                    impl HasDisplayHandle for DisplayAndWinHandle {
                        fn display_handle(
                            &self,
                        ) -> Result<
                            DisplayHandle<'_>,
                            winit::raw_window_handle::HandleError,
                        > {
                            Ok(self.0.clone())
                        }
                    }

                    impl HasWindowHandle for DisplayAndWinHandle {
                        fn window_handle(
                            &self,
                        ) -> Result<
                            WindowHandle<'_>,
                            winit::raw_window_handle::HandleError,
                        > {
                            Ok(self.1.clone())
                        }
                    }

                    rfd::FileDialog::new()
                        .set_title(FILE_DIALOG_NAME)
                        .set_parent(&DisplayAndWinHandle(display, handle))
                        .save_file()
                };

                #[cfg(not(target_os = "windows"))]
                let file = rfd::FileDialog::new()
                    .set_title(FILE_DIALOG_NAME)
                    .save_file();

                if let Some(file) = file {
                    return Self::task(KeyChooseMessage::OutFileSelected(file));
                }

                println!("OutFile not chosen");
            }
            KeyChooseMessage::OutFileSelected(path) => {
                self.output_path = Some(path);
            }
            KeyChooseMessage::Cipher => {
                return self.cipher();
            }
            KeyChooseMessage::Decipher => {
                return self.decipher();
            }
            KeyChooseMessage::BigtextAction(action) => {
                if let FileOrText::Text(text) = &mut self.input {
                    text.perform(action);
                }
            }
        }

        Task::none()
    }

    fn cipher(&mut self) -> Task<MainMessage> {
        let Some(vigenere) = Vigenere::new(self.key.clone()) else {
            return Task::done(MainMessage::Error(ERR_BAD_PASSWORD));
        };

        match self.input.take() {
            FileOrText::File(path_buf) => {
                let Ok(file) = File::open(path_buf) else {
                    return Self::err("Couldn't open input file");
                };

                let file = BufReader::new(file);

                let bytes = file.bytes().filter_map(Result::ok).chunks(32);

                let chars_iter = bytes
                    .into_iter()
                    .flat_map(|chunk| {
                        String::from_utf8(chunk.collect::<Vec<u8>>())
                    })
                    .flat_map(|val| val.chars().collect::<Vec<_>>());

                let result = vigenere.cipher(chars_iter);

                let Some(outfile) = self.output_path.take() else {
                    unreachable!("Cipher executed while outpath is None");
                };

                let Ok(outfile) = File::create(outfile) else {
                    return Self::err("Couldn't create the file");
                };

                let mut outfile = BufWriter::new(outfile);

                for slice in &result.chunks(Self::N_VALUES) {
                    if outfile
                        .write_all(slice.collect::<String>().as_bytes())
                        .is_err()
                    {
                        return Self::err(
                            "Something went wrong while writing result to the file",
                        );
                    }
                }

                Task::none()
            }
            FileOrText::Text(text) => {
                let text = text.text();
                let result = vigenere
                    .cipher(IDENTIFYING_MESSAGE.chars().chain(text.chars()));
                let Some(outfile) = self.output_path.take() else {
                    return Self::err(
                        "Cipher path executed while output path is none",
                    );
                };

                let Ok(outfile) = File::create(outfile) else {
                    return Self::err("Couldn't create the file");
                };

                let mut outfile = BufWriter::new(outfile);

                for slice in &result.chunks(Self::N_VALUES) {
                    if outfile
                        .write_all(slice.collect::<String>().as_bytes())
                        .is_err()
                    {
                        return Self::err(
                            "Something went wrong while writing result to the file",
                        );
                    }
                }

                Task::none()
            }
        }
    }

    fn decipher(&mut self) -> Task<MainMessage> {
        let Some(vigenere) = Vigenere::new(self.key.clone()) else {
            return Task::done(MainMessage::Error(ERR_BAD_PASSWORD));
        };

        match self.input.take() {
            FileOrText::File(path_buf) => {
                let Ok(file) = File::open(path_buf) else {
                    return Self::err("Couldn't open input file");
                };

                let file = BufReader::new(file);

                let bytes = file.bytes().filter_map(Result::ok).chunks(32);

                let chars_iter = bytes
                    .into_iter()
                    .flat_map(|chunk| {
                        String::from_utf8(chunk.collect::<Vec<u8>>())
                    })
                    .flat_map(|val| val.chars().collect::<Vec<_>>());

                let mut result = vigenere.decipher(chars_iter);

                let Some(outfile) = self.output_path.take() else {
                    unreachable!("Cipher executed while outpath is None");
                };

                let Ok(outfile) = File::create(outfile) else {
                    return Self::err("Couldn't create the file");
                };

                let mut outfile = BufWriter::new(outfile);

                let identmsg = result
                    .by_ref()
                    .take(IDENTIFYING_MESSAGE.len())
                    .collect::<String>();

                if identmsg != IDENTIFYING_MESSAGE {
                    return Self::err("Message was not ciphered in this app");
                }

                for slice in &result.chunks(Self::N_VALUES) {
                    if outfile
                        .write_all(slice.collect::<String>().as_bytes())
                        .is_err()
                    {
                        return Self::err(
                            "Something went wrong while writing result to the file",
                        );
                    }
                }

                Task::none()
            }
            FileOrText::Text(text) => {
                let text = text.text();

                let mut result = vigenere.decipher(text.chars());

                let Some(outfile) = self.output_path.take() else {
                    unreachable!("Cipher executed while outpath is None");
                };

                let Ok(outfile) = File::create(outfile) else {
                    return Self::err("Couldn't create the file");
                };

                let mut outfile = BufWriter::new(outfile);

                let identmsg = result
                    .by_ref()
                    .take(IDENTIFYING_MESSAGE.len())
                    .collect::<String>();

                if identmsg != IDENTIFYING_MESSAGE {
                    return Self::err("Message was not ciphered in this app");
                }

                for slice in &result.chunks(Self::N_VALUES) {
                    if outfile
                        .write_all(slice.collect::<String>().as_bytes())
                        .is_err()
                    {
                        return Self::err(
                            "Something went wrong while writing result to the file",
                        );
                    }
                }

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, KeyChooseMessage> {
        row![self.content(),]
            .height(Length::Fill)
            .align_y(Alignment::Center)
            .into()
    }

    fn content(&self) -> Column<KeyChooseMessage> {
        let textbox = row![
            horizontal_space().width(Length::FillPortion(1)),
            text_input("Input your key", &self.key)
                .secure(true)
                .width(Length::FillPortion(2))
                .on_input(KeyChooseMessage::TextboxInput),
            horizontal_space().width(Length::FillPortion(1)),
        ];

        let val = match &self.input {
            FileOrText::Text(text) if text.text().trim().is_empty() => {
                Some(KeyChooseMessage::InFileChoose)
            }
            _ => None,
        };

        let buttons = match (&self.output_path, &self.input) {
            (Some(_), FileOrText::File(_) | FileOrText::Text(_))
                if !self.key.is_empty() =>
            {
                row![
                    button("Choose input file").on_press_maybe(val),
                    button("Choose output file")
                        .on_press(KeyChooseMessage::OutFileChoose),
                    button("Cipher").on_press(KeyChooseMessage::Cipher),
                    button("Decipher").on_press(KeyChooseMessage::Decipher),
                ]
            }
            _ => row![
                button("Choose input file").on_press_maybe(val),
                button("Choose output file")
                    .on_press(KeyChooseMessage::OutFileChoose),
            ],
        };

        let big_textfield: iced::widget::Row<'_, _, iced::Theme> =
            match &self.input {
                FileOrText::Text(text) => row![
                    horizontal_space().width(Length::FillPortion(1)),
                    iced::widget::container(
                        widget::text_editor(text)
                            .placeholder("Input your message")
                            .on_action(KeyChooseMessage::BigtextAction)
                    )
                    .width(Length::FillPortion(2)),
                    horizontal_space().width(Length::FillPortion(1)),
                ],
                FileOrText::File(_) => {
                    row![horizontal_space().width(Length::FillPortion(1)),]
                }
            };

        column![
            vertical_space().height(Length::FillPortion(8)),
            textbox,
            vertical_space().height(Length::FillPortion(1)),
            big_textfield,
            buttons,
            vertical_space().height(Length::FillPortion(8)),
        ]
        .width(Length::Fill)
        .align_x(Alignment::Center)
    }
}
