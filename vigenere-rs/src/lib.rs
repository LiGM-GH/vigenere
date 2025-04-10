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

trait IsAscii {
    fn is_ascii(&self) -> bool;
}

trait IsAsciiExtended: IsAscii {
    fn is_ascii_extended(&self) -> bool;
    fn is_ascii_all(&self) -> bool {
        self.is_ascii_extended() || self.is_ascii()
    }
}

impl IsAscii for char {
    fn is_ascii(&self) -> bool {
        self.is_ascii()
    }
}

impl IsAscii for u8 {
    fn is_ascii(&self) -> bool {
        self.is_ascii()
    }
}

impl IsAsciiExtended for char {
    fn is_ascii_extended(&self) -> bool {
        (*self as u8).is_ascii_extended()
    }
}

impl IsAsciiExtended for u8 {
    fn is_ascii_extended(&self) -> bool {
        127 < *self && *self <= 225u8
    }
}

#[allow(dead_code)]
mod consts {
    //! ```image
    //! ascii range
    //! [ control | graphic | extended ]
    //! ^^       ^ ^       ^ ^        ^^
    //! ||       | |       | |        ||
    //! +|-------|-|-------|-|--------||---- ASCIIALL_START
    //!  +-------|-|-------|-|--------||---- ASCII_CONTROL_START
    //!          +-|-------|-|--------||---- ASCII_CONTROL_END
    //!            +-------|-|--------||---- ASCII_START
    //!                    +-|--------||---- ASCII_END
    //!                      +--------||---- ASCIIEXT_START
    //!                               +|---- ASCIIEXT_END
    //!                                +---- ASCIIALL_END
    //! ```
    pub const ASCII_CONTROL_START: u8 = 0;
    pub const ASCII_CONTROL_END: u8 = 31;
    pub const ASCII_START: u8 = 32;
    pub const ASCII_END: u8 = 127;
    pub const ASCIIEXT_START: u8 = 128;
    pub const ASCIIEXT_END: u8 = 255;
    pub const ASCIIALL_START: u8 = ASCII_CONTROL_START;
    pub const ASCIIALL_END: u8 = ASCIIEXT_END;
    pub const ASCIIALL_RANGE_LEN: u8 = ASCIIALL_END - ASCII_CONTROL_END;
}
use consts::{ASCII_START, ASCIIALL_RANGE_LEN};

impl AsciiShift for u8 {
    fn ascii_lshift(self, shift: Self) -> Self {
        ((self - ASCII_START) + (shift - ASCII_START)) % ASCIIALL_RANGE_LEN
            + ASCII_START
    }
    fn ascii_rshift(self, shift: Self) -> Self {
        dbg!("No overflow here");
        let rel_self = self - ASCII_START;
        dbg!("No overflow still");
        let rel_shift = shift - ASCII_START;
        dbg!("No overflow shift");
        let rel_plus = ASCIIALL_RANGE_LEN - rel_shift;
        dbg!("No overflow plus");
        let abs_sum = rel_self + rel_plus;
        dbg!("Overflow sum");
        let result = abs_sum % ASCIIALL_RANGE_LEN + ASCII_START;
        dbg!("Overflow here");
        result
    }
}

impl Vigenere {
    pub fn new(key: String) -> Option<Self> {
        if key.chars().all(|val| val.is_ascii_all() || val == ' ')
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

    pub(crate) fn cipher_inner<
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
        let shift = |(l, r)| AsciiShift::ascii_lshift(dbg!(l as char) as u8, dbg!(r as char) as u8);
        let result = vigenere.cipher_inner(inner.iter().cloned(), shift);

        assert_eq!(vigenere.decipher(result).collect::<Vec<_>>(), inner);
    }
}
