pub struct Vigenere {
    key: String,
}

impl std::fmt::Debug for Vigenere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vigenere").finish()
    }
}

trait AsciiShift {
    fn ascii_lshift(self, shift: Self) -> Self;
    fn ascii_rshift(self, shift: Self) -> Self;
}

const ALPHA_RANGE_LEN: u8 = 126 - 32 + 1;

impl AsciiShift for u8 {
    fn ascii_lshift(self, shift: Self) -> Self {
        ((self - 32) + (shift - 32)) % ALPHA_RANGE_LEN + 32
    }
    fn ascii_rshift(self, shift: Self) -> Self {
        ((self - 32 + ALPHA_RANGE_LEN) - (shift - 32)) % ALPHA_RANGE_LEN + 32
    }
}

impl Vigenere {
    pub fn new(key: String) -> Option<Self> {
        if key.chars().all(|val| val.is_ascii_graphic() || val == ' ')
            && !key.is_empty()
        {
            Some(Self { key })
        } else {
            None
        }
    }

    pub fn decipher<I: Iterator<Item = u8>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = u8> + use<'_, I> {
        let shift = |(l, r): (u8, u8)| AsciiShift::ascii_rshift(l, r);

        self.cipher_inner(inner, shift)
    }

    pub fn cipher<I: Iterator<Item = u8>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = u8> + use<'_, I> {
        let shift = |(l, r): (u8, u8)| AsciiShift::ascii_lshift(l, r);

        self.cipher_inner(inner, shift)
    }

    pub(self) fn cipher_inner<
        InputIter: Iterator<Item = u8>,
        Fun: FnMut((u8, u8)) -> u8,
    >(
        &self,
        inner: InputIter,
        shift: Fun,
    ) -> impl Iterator<Item = u8> + use<'_, InputIter, Fun> {
        let chars = self.key.chars().map(|ch| ch as u8).cycle();

        inner
            .filter(|ch| ch.is_ascii_graphic() || *ch == b' ')
            .zip(chars)
            .map(shift)
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

    #[test]
    fn ascii_shift_works() {
        for i in b'A'..=b'Z' {
            for j in b'A'..=b'Z' {
                assert!(i.ascii_lshift(j).ascii_rshift(j) == i);
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

        let inner = b"FIRST SECOND THIRD".to_vec();
        let shift = |(l, r)| AsciiShift::ascii_lshift(l, r);
        let result = vigenere.cipher_inner(inner.iter().cloned(), shift);

        assert_eq!(vigenere.decipher(result).collect::<Vec<_>>(), inner);
    }
}
