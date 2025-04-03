use std::str::Chars;

pub struct Vigenere {
    key: String,
}

trait AsciiShift {
    fn ascii_shift(self, shift: Self) -> Self;
}

impl AsciiShift for u8 {
    fn ascii_shift(self, shift: Self) -> Self {
        todo!()
    }
}

impl Vigenere {
    pub fn new(key: String) -> Option<Self> {
        if key.is_ascii() {
            Some(Self { key })
        } else {
            None
        }
    }

    pub fn cipher<I: Iterator<Item = u8>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = u8> + use<'_, I> {
        let chars = self.key.chars().map(|ch| ch as u8).cycle();
        let shift = |(l, r): (u8, u8)| l.ascii_shift(r);

        inner.zip(chars).map(shift)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
