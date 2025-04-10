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
        IsAscii::is_ascii(self) || IsAsciiExtended::is_ascii_extended(self)
    }
}

impl IsAscii for char {
    fn is_ascii(&self) -> bool {
        IsAscii::is_ascii(&(*self as u8))
    }
}

impl IsAscii for u8 {
    fn is_ascii(&self) -> bool {
        ASCII_START <= *self && *self <= ASCII_END
    }
}

impl IsAsciiExtended for char {
    fn is_ascii_extended(&self) -> bool {
        (*self as u8).is_ascii_extended()
    }

    fn is_ascii_all(&self) -> bool {
        let self_ext = IsAsciiExtended::is_ascii_extended(self);
        let self_asc = IsAscii::is_ascii(self);
        self_asc || self_ext
    }
}

impl IsAsciiExtended for u8 {
    fn is_ascii_extended(&self) -> bool {
        ASCIIEXT_START <= *self
        // && *self <= ASCIIEXT_END // https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons
    }

    fn is_ascii_all(&self) -> bool {
        let self_ext = IsAsciiExtended::is_ascii_extended(self);
        let self_asc = IsAscii::is_ascii(self);
        self_asc || self_ext
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
use consts::{
    ASCII_END, ASCII_START, ASCIIALL_RANGE_LEN, ASCIIEXT_START,
};

impl AsciiShift for u8 {
    fn ascii_lshift(self, shift: Self) -> Self {
        let rel_self = (self - ASCII_START) as u16;
        let rel_shift = (shift - ASCII_START) as u16;
        let sum = rel_self + rel_shift;

        (sum % ASCIIALL_RANGE_LEN as u16) as u8 + ASCII_START
    }
    fn ascii_rshift(self, shift: Self) -> Self {
        let rel_self = (self - ASCII_START) as u16;
        let rel_shift = (shift - ASCII_START) as u16;
        let mid_sum = ASCIIALL_RANGE_LEN as u16 - rel_shift;
        let abs_sum = rel_self + mid_sum;

        (abs_sum % ASCIIALL_RANGE_LEN as u16) as u8 + ASCII_START
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
            .filter(IsAsciiExtended::is_ascii_all)
            .zip(chars)
            .map(shift)
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
        ($val:expr) => {{
            eprintln!("{: <7}: {: <10}", stringify!($val), format!("{:?}", $val));
            $val
        }};
    }

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

        let inner = b"FIrst seCOnd thiRD".to_vec();
        let shift = |(l, r)| {
            dprint!(# "INPUT");
            AsciiShift::ascii_lshift(dprint!(l as char) as u8, dprintln!(r as char) as u8)
        };

        let result = vigenere.cipher_inner(inner.iter().cloned(), shift);

        let shift = |(l, r)| {
            dprint!(# "OUTPUT");
            AsciiShift::ascii_rshift(dprint!(l as char) as u8, dprintln!(r as char) as u8)
        };

        let deciphered =
            vigenere.cipher_inner(result, shift).collect::<Vec<_>>();
        assert_eq!(String::from_utf8(deciphered), String::from_utf8(inner));
    }
}
