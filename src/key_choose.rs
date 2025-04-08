use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use iced::{
    Alignment, Element, Length, Task,
    widget::{Column, button, column, horizontal_space, row, text_input},
};
use itertools::Itertools;
use vigenere_rs::Vigenere;

use crate::MainMessage;

const FILE_DIALOG_NAME: &str = "CHOOSE FILE";
const ERR_BAD_PASSWORD: &str =
    "Password should be only-ASCII-alphabetic and non-empty";

#[derive(Debug)]
pub enum FileOrText {
    File(PathBuf),
    Text(String),
    None,
}

impl FileOrText {
    pub fn take(&mut self) -> Self {
        let mut thing = Self::None;

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
}

impl KeyChooseView {
    const N_VALUES: usize = 4;

    pub const fn new() -> Self {
        Self {
            key: String::new(),
            input: FileOrText::None,
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

                let chars_iter = file.bytes().filter_map(Result::ok);

                let result = vigenere.cipher(chars_iter);

                let Some(outfile) = self.output_path.take() else {
                    unreachable!("Cipher executed while outpath is None");
                };

                let Ok(outfile) = File::create(outfile) else {
                    return Self::err("Couldn't create the file");
                };

                let mut outfile = BufWriter::new(outfile);

                {
                    for slice in &result.chunks(Self::N_VALUES) {
                        if outfile
                            .write_all(&slice.collect::<Vec<u8>>())
                            .is_err()
                        {
                            return Self::err(
                                "Something went wrong while writing result to the file",
                            );
                        };
                    }
                }

                Task::none()
            }
            FileOrText::Text(text) => {
                todo!();
            }
            FileOrText::None => {
                unreachable!("This should never happen!")
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

                let chars_iter = file.bytes().filter_map(Result::ok);

                let result = vigenere.decipher(chars_iter);

                let Some(outfile) = self.output_path.take() else {
                    unreachable!("Cipher executed while outpath is None");
                };

                let Ok(outfile) = File::create(outfile) else {
                    return Self::err("Couldn't create the file");
                };

                let mut outfile = BufWriter::new(outfile);

                {
                    for slice in &result.chunks(Self::N_VALUES) {
                        if outfile
                            .write_all(&slice.collect::<Vec<u8>>())
                            .is_err()
                        {
                            return Self::err(
                                "Something went wrong while writing result to the file",
                            );
                        };
                    }
                }

                Task::none()
            }
            FileOrText::Text(text) => {
                todo!();
            }
            FileOrText::None => {
                unreachable!("This should never happen!")
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

        let buttons =
            match (&self.output_path, &self.input, self.key.is_empty()) {
                (Some(_), FileOrText::File(_) | FileOrText::Text(_), false) => {
                    row![
                        button("Choose input file")
                            .on_press(KeyChooseMessage::InFileChoose),
                        button("Choose output file")
                            .on_press(KeyChooseMessage::OutFileChoose),
                        button("Cipher").on_press(KeyChooseMessage::Cipher),
                        button("Decipher").on_press(KeyChooseMessage::Decipher),
                    ]
                }
                _ => row![
                    button("Choose input file")
                        .on_press(KeyChooseMessage::InFileChoose),
                    button("Choose output file")
                        .on_press(KeyChooseMessage::OutFileChoose),
                ],
            };

        column![textbox, buttons,]
            .width(Length::Fill)
            .align_x(Alignment::Center)
    }
}
