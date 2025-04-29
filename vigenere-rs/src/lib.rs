const UTFGRAPHIC_START: u32 = 0x0020;
const UTFGRAPHIC_ENDED: u32 = 0xFFFD;

const UTFGRAPHIC_LEN: u32 = UTFGRAPHIC_ENDED - UTFGRAPHIC_START + 1;

pub struct Vigenere {
    key: String,
}

impl std::fmt::Debug for Vigenere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vigenere").finish()
    }
}

trait Shift: Sized {
    fn lshift(self, shift: Self) -> Option<Self>;
    fn rshift(self, shift: Self) -> Option<Self>;
}

impl Shift for char {
    fn lshift(self, shift: Self) -> Option<Self> {
        if UTFGRAPHIC_START > self as u32
            || self as u32 > UTFGRAPHIC_ENDED
            || UTFGRAPHIC_START > shift as u32
            || shift as u32 > UTFGRAPHIC_ENDED
        {
            return None;
        }

        char::from_u32(
            (self as u32 + shift as u32 - UTFGRAPHIC_START) % UTFGRAPHIC_LEN
                + UTFGRAPHIC_START,
        )
    }

    fn rshift(self, shift: Self) -> Option<Self> {
        if UTFGRAPHIC_START > self as u32
            || self as u32 > UTFGRAPHIC_ENDED
            || UTFGRAPHIC_START > shift as u32
            || shift as u32 > UTFGRAPHIC_ENDED
        {
            return None;
        }

        char::from_u32(
            (self as u32 + UTFGRAPHIC_LEN - shift as u32 - UTFGRAPHIC_START)
                % UTFGRAPHIC_LEN
                + UTFGRAPHIC_START,
        )
    }
}

impl Vigenere {
    pub fn new(key: String) -> Option<Self> {
        Some(Self { key })
    }

    pub fn decipher<I: Iterator<Item = char>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = char> + use<'_, I> {
        let shift = |(l, r): (char, char)| Shift::rshift(l, r);

        self.cipher_inner(inner, shift)
    }

    pub fn cipher<I: Iterator<Item = char>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = char> + use<'_, I> {
        let shift = |(l, r): (char, char)| Shift::lshift(l, r);

        self.cipher_inner(inner, shift)
    }

    pub(crate) fn cipher_inner<
        InputIter: Iterator<Item = char>,
        Fun: FnMut((char, char)) -> Option<char>,
    >(
        &self,
        inner: InputIter,
        shift: Fun,
    ) -> impl Iterator<Item = char> + use<'_, InputIter, Fun> {
        let chars = self.key.chars().cycle();

        inner.zip(chars).filter_map(shift)
    }
}

#[cfg(test)]
mod tests {
    macro_rules! dprint {
        (#$val:expr) => {{
            eprint!("{: <7} \t| ", format!("{:?}", $val));
            $val
        }};
        ($val:expr) => {{
            eprint!(
                "{: <7}: {: <10} \t| ",
                stringify!($val),
                format!("{:?}", $val)
            );
            $val
        }};
    }

    macro_rules! dprintln {
        (#$val:expr) => {{
            eprintln!("{: <7} \t| ", format!("{:?}", $val));
            $val
        }};
        ($val:expr) => {{
            eprintln!(
                "{: <7}: {: <10}",
                stringify!($val),
                format!("{:?}", $val)
            );
            $val
        }};
    }

    use super::*;

    #[test]
    fn ascii_shift_works() {
        for i in 'A'..='Z' {
            for j in 'A'..='Z' {
                assert!(i.lshift(j).unwrap().rshift(j).unwrap() == i);
            }
        }
    }

    #[test]
    fn vigenere_works() {
        let vigenere = Vigenere::new("Whatever it is".into());

        assert!(vigenere.is_some());

        let Some(vigenere) = vigenere else {
            unreachable!()
        };

        let inner = "FIrst seCOnd thiRD";
        let shift = |(l, r)| {
            dprint!(# "INPUT");
            Shift::lshift(dprint!(l), dprintln!(r))
        };

        let result = vigenere.cipher_inner(inner.chars(), shift);

        let shift = |(l, r)| {
            dprint!(# "OUTPUT");
            Shift::rshift(dprint!(l), dprintln!(r))
        };

        let deciphered =
            vigenere.cipher_inner(result, shift).collect::<String>();
        assert_eq!(deciphered.as_str(), inner);
    }

    #[test]
    fn cyrillic_vigenere() {
        let vigenere = Vigenere::new("Тестовый пароль".into());

        assert!(vigenere.is_some());

        let Some(vigenere) = vigenere else {
            unreachable!()
        };

        let message = "Кириллическое сообщение";

        let ciphered = vigenere.cipher(message.chars());

        let result = vigenere.decipher(ciphered).collect::<String>();

        println!("result: {:?}", result);
        println!("message: {:?}", message);
        assert!(result == message);
    }

    #[test]
    fn files_vigenere() {
        use file_utils::{read_file, write_file};

        let vigenere = Vigenere::new("Whatever".into())
            .expect("Vigenere couldn't be created");

        write_file(
            "../../tests/vigenere_input.txt",
            "FIRST SECOND THIRD".chars(),
        );

        let input_content = read_file("../../tests/vigenere_input.txt");
        eprintln!("\n\n  Input content: '{}'\n\n", input_content);
        let result = vigenere.cipher(input_content.chars());
        write_file("../../tests/vigenere.txt", result);

        let cipher_content = read_file("../../tests/vigenere.txt");
        let result = vigenere.decipher(cipher_content.chars());
        write_file("../../tests/vigenere_result.txt", result);

        let output_content = read_file("../../tests/vigenere_result.txt");
        eprintln!("\n\n  Output content: '{}'\n\n", output_content);

        assert_eq!(input_content, output_content);
    }

    mod file_utils {
        use itertools::Itertools;
        use std::{
            fs::File,
            io::{BufReader, BufWriter, Read, Write},
            path::Path,
        };

        const N_VALUES: usize = 4;

        pub fn read_file(path: impl AsRef<Path> + Copy) -> String {
            eprintln!(
                "This is what it looks like normally: {:?}",
                std::fs::read_to_string(path)
            );
            let file = File::open(path).expect("Couldn't get input path");

            let file = BufReader::new(file);

            let bytes = file
                .bytes()
                .filter_map(Result::ok)
                .inspect(|ch| {
                    dprintln!(*ch as char);
                })
                .chunks(N_VALUES);

            let chars_iter = bytes
                .into_iter()
                .flat_map(|chunk| String::from_utf8(chunk.collect::<Vec<u8>>()))
                .inspect(|read_file_chunk| {
                    dprintln!(read_file_chunk);
                })
                .flat_map(|val| val.chars().collect::<Vec<_>>());

            chars_iter.collect()
        }

        pub fn write_file(
            output_path: impl AsRef<Path>,
            result: impl Iterator<Item = char>,
        ) {
            let outfile = File::create(output_path)
                .expect("Couldn't write to the output file");

            let mut outfile = BufWriter::new(outfile);

            for slice in &result.chunks(N_VALUES) {
                if outfile
                    .write_all(slice.collect::<String>().as_bytes())
                    .is_err()
                {
                    panic!("Couldn't write to the file");
                };
            }
        }
    }
}
