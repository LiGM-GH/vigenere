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

const ALPHA_RANGE_LEN: u8 = b'Z' - b'A' + 1;

impl AsciiShift for u8 {
    fn ascii_lshift(self, shift: Self) -> Self {
        ((self - b'A') + (shift - b'A')) % ALPHA_RANGE_LEN + b'A'
    }
    fn ascii_rshift(self, shift: Self) -> Self {
        ((self - b'A' + ALPHA_RANGE_LEN) - (shift - b'A')) % ALPHA_RANGE_LEN
            + b'A'
    }
}

impl Vigenere {
    pub fn new(key: String) -> Option<Self> {
        if key.chars().all(|val| val.is_ascii_alphabetic()) && !key.is_empty() {
            let key = key.to_ascii_uppercase();

            Some(Self { key })
        } else {
            None
        }
    }

    pub fn decipher<I: Iterator<Item = u8>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = u8> + use<'_, I> {
        let chars = self.key.chars().map(|ch| ch as u8).cycle();
        let shift = |(l, r): (u8, u8)| l.ascii_rshift(r);

        inner
            .filter(|ch| ch.is_ascii_alphabetic())
            .map(|ch| ch.to_ascii_uppercase())
            .zip(chars)
            .map(shift)
    }

    pub fn cipher<I: Iterator<Item = u8>>(
        &self,
        inner: I,
    ) -> impl Iterator<Item = u8> + use<'_, I> {
        let chars = self.key.chars().map(|ch| ch as u8).cycle();
        let shift = |(l, r): (u8, u8)| l.ascii_lshift(r);

        inner
            .filter(|ch| ch.is_ascii_alphabetic())
            .map(|ch| ch.to_ascii_uppercase())
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
    fn full_ascii_tabula_recta_ascii_lshift() {
        assert_eq!(b'A'.ascii_lshift(b'A'), b'A');
        assert_eq!(b'B'.ascii_lshift(b'A'), b'B');
        assert_eq!(b'C'.ascii_lshift(b'A'), b'C');
        assert_eq!(b'D'.ascii_lshift(b'A'), b'D');
        assert_eq!(b'E'.ascii_lshift(b'A'), b'E');
        assert_eq!(b'F'.ascii_lshift(b'A'), b'F');
        assert_eq!(b'G'.ascii_lshift(b'A'), b'G');
        assert_eq!(b'H'.ascii_lshift(b'A'), b'H');
        assert_eq!(b'I'.ascii_lshift(b'A'), b'I');
        assert_eq!(b'J'.ascii_lshift(b'A'), b'J');
        assert_eq!(b'K'.ascii_lshift(b'A'), b'K');
        assert_eq!(b'L'.ascii_lshift(b'A'), b'L');
        assert_eq!(b'M'.ascii_lshift(b'A'), b'M');
        assert_eq!(b'N'.ascii_lshift(b'A'), b'N');
        assert_eq!(b'O'.ascii_lshift(b'A'), b'O');
        assert_eq!(b'P'.ascii_lshift(b'A'), b'P');
        assert_eq!(b'Q'.ascii_lshift(b'A'), b'Q');
        assert_eq!(b'R'.ascii_lshift(b'A'), b'R');
        assert_eq!(b'S'.ascii_lshift(b'A'), b'S');
        assert_eq!(b'T'.ascii_lshift(b'A'), b'T');
        assert_eq!(b'U'.ascii_lshift(b'A'), b'U');
        assert_eq!(b'V'.ascii_lshift(b'A'), b'V');
        assert_eq!(b'W'.ascii_lshift(b'A'), b'W');
        assert_eq!(b'X'.ascii_lshift(b'A'), b'X');
        assert_eq!(b'Y'.ascii_lshift(b'A'), b'Y');
        assert_eq!(b'Z'.ascii_lshift(b'A'), b'Z');

        assert_eq!(b'A'.ascii_lshift(b'B'), b'B');
        assert_eq!(b'B'.ascii_lshift(b'B'), b'C');
        assert_eq!(b'C'.ascii_lshift(b'B'), b'D');
        assert_eq!(b'D'.ascii_lshift(b'B'), b'E');
        assert_eq!(b'E'.ascii_lshift(b'B'), b'F');
        assert_eq!(b'F'.ascii_lshift(b'B'), b'G');
        assert_eq!(b'G'.ascii_lshift(b'B'), b'H');
        assert_eq!(b'H'.ascii_lshift(b'B'), b'I');
        assert_eq!(b'I'.ascii_lshift(b'B'), b'J');
        assert_eq!(b'J'.ascii_lshift(b'B'), b'K');
        assert_eq!(b'K'.ascii_lshift(b'B'), b'L');
        assert_eq!(b'L'.ascii_lshift(b'B'), b'M');
        assert_eq!(b'M'.ascii_lshift(b'B'), b'N');
        assert_eq!(b'N'.ascii_lshift(b'B'), b'O');
        assert_eq!(b'O'.ascii_lshift(b'B'), b'P');
        assert_eq!(b'P'.ascii_lshift(b'B'), b'Q');
        assert_eq!(b'Q'.ascii_lshift(b'B'), b'R');
        assert_eq!(b'R'.ascii_lshift(b'B'), b'S');
        assert_eq!(b'S'.ascii_lshift(b'B'), b'T');
        assert_eq!(b'T'.ascii_lshift(b'B'), b'U');
        assert_eq!(b'U'.ascii_lshift(b'B'), b'V');
        assert_eq!(b'V'.ascii_lshift(b'B'), b'W');
        assert_eq!(b'W'.ascii_lshift(b'B'), b'X');
        assert_eq!(b'X'.ascii_lshift(b'B'), b'Y');
        assert_eq!(b'Y'.ascii_lshift(b'B'), b'Z');
        assert_eq!(b'Z'.ascii_lshift(b'B'), b'A');

        assert_eq!(b'A'.ascii_lshift(b'C'), b'C');
        assert_eq!(b'B'.ascii_lshift(b'C'), b'D');
        assert_eq!(b'C'.ascii_lshift(b'C'), b'E');
        assert_eq!(b'D'.ascii_lshift(b'C'), b'F');
        assert_eq!(b'E'.ascii_lshift(b'C'), b'G');
        assert_eq!(b'F'.ascii_lshift(b'C'), b'H');
        assert_eq!(b'G'.ascii_lshift(b'C'), b'I');
        assert_eq!(b'H'.ascii_lshift(b'C'), b'J');
        assert_eq!(b'I'.ascii_lshift(b'C'), b'K');
        assert_eq!(b'J'.ascii_lshift(b'C'), b'L');
        assert_eq!(b'K'.ascii_lshift(b'C'), b'M');
        assert_eq!(b'L'.ascii_lshift(b'C'), b'N');
        assert_eq!(b'M'.ascii_lshift(b'C'), b'O');
        assert_eq!(b'N'.ascii_lshift(b'C'), b'P');
        assert_eq!(b'O'.ascii_lshift(b'C'), b'Q');
        assert_eq!(b'P'.ascii_lshift(b'C'), b'R');
        assert_eq!(b'Q'.ascii_lshift(b'C'), b'S');
        assert_eq!(b'R'.ascii_lshift(b'C'), b'T');
        assert_eq!(b'S'.ascii_lshift(b'C'), b'U');
        assert_eq!(b'T'.ascii_lshift(b'C'), b'V');
        assert_eq!(b'U'.ascii_lshift(b'C'), b'W');
        assert_eq!(b'V'.ascii_lshift(b'C'), b'X');
        assert_eq!(b'W'.ascii_lshift(b'C'), b'Y');
        assert_eq!(b'X'.ascii_lshift(b'C'), b'Z');
        assert_eq!(b'Y'.ascii_lshift(b'C'), b'A');
        assert_eq!(b'Z'.ascii_lshift(b'C'), b'B');

        assert_eq!(b'A'.ascii_lshift(b'D'), b'D');
        assert_eq!(b'B'.ascii_lshift(b'D'), b'E');
        assert_eq!(b'C'.ascii_lshift(b'D'), b'F');
        assert_eq!(b'D'.ascii_lshift(b'D'), b'G');
        assert_eq!(b'E'.ascii_lshift(b'D'), b'H');
        assert_eq!(b'F'.ascii_lshift(b'D'), b'I');
        assert_eq!(b'G'.ascii_lshift(b'D'), b'J');
        assert_eq!(b'H'.ascii_lshift(b'D'), b'K');
        assert_eq!(b'I'.ascii_lshift(b'D'), b'L');
        assert_eq!(b'J'.ascii_lshift(b'D'), b'M');
        assert_eq!(b'K'.ascii_lshift(b'D'), b'N');
        assert_eq!(b'L'.ascii_lshift(b'D'), b'O');
        assert_eq!(b'M'.ascii_lshift(b'D'), b'P');
        assert_eq!(b'N'.ascii_lshift(b'D'), b'Q');
        assert_eq!(b'O'.ascii_lshift(b'D'), b'R');
        assert_eq!(b'P'.ascii_lshift(b'D'), b'S');
        assert_eq!(b'Q'.ascii_lshift(b'D'), b'T');
        assert_eq!(b'R'.ascii_lshift(b'D'), b'U');
        assert_eq!(b'S'.ascii_lshift(b'D'), b'V');
        assert_eq!(b'T'.ascii_lshift(b'D'), b'W');
        assert_eq!(b'U'.ascii_lshift(b'D'), b'X');
        assert_eq!(b'V'.ascii_lshift(b'D'), b'Y');
        assert_eq!(b'W'.ascii_lshift(b'D'), b'Z');
        assert_eq!(b'X'.ascii_lshift(b'D'), b'A');
        assert_eq!(b'Y'.ascii_lshift(b'D'), b'B');
        assert_eq!(b'Z'.ascii_lshift(b'D'), b'C');

        assert_eq!(b'A'.ascii_lshift(b'E'), b'E');
        assert_eq!(b'B'.ascii_lshift(b'E'), b'F');
        assert_eq!(b'C'.ascii_lshift(b'E'), b'G');
        assert_eq!(b'D'.ascii_lshift(b'E'), b'H');
        assert_eq!(b'E'.ascii_lshift(b'E'), b'I');
        assert_eq!(b'F'.ascii_lshift(b'E'), b'J');
        assert_eq!(b'G'.ascii_lshift(b'E'), b'K');
        assert_eq!(b'H'.ascii_lshift(b'E'), b'L');
        assert_eq!(b'I'.ascii_lshift(b'E'), b'M');
        assert_eq!(b'J'.ascii_lshift(b'E'), b'N');
        assert_eq!(b'K'.ascii_lshift(b'E'), b'O');
        assert_eq!(b'L'.ascii_lshift(b'E'), b'P');
        assert_eq!(b'M'.ascii_lshift(b'E'), b'Q');
        assert_eq!(b'N'.ascii_lshift(b'E'), b'R');
        assert_eq!(b'O'.ascii_lshift(b'E'), b'S');
        assert_eq!(b'P'.ascii_lshift(b'E'), b'T');
        assert_eq!(b'Q'.ascii_lshift(b'E'), b'U');
        assert_eq!(b'R'.ascii_lshift(b'E'), b'V');
        assert_eq!(b'S'.ascii_lshift(b'E'), b'W');
        assert_eq!(b'T'.ascii_lshift(b'E'), b'X');
        assert_eq!(b'U'.ascii_lshift(b'E'), b'Y');
        assert_eq!(b'V'.ascii_lshift(b'E'), b'Z');
        assert_eq!(b'W'.ascii_lshift(b'E'), b'A');
        assert_eq!(b'X'.ascii_lshift(b'E'), b'B');
        assert_eq!(b'Y'.ascii_lshift(b'E'), b'C');
        assert_eq!(b'Z'.ascii_lshift(b'E'), b'D');

        assert_eq!(b'A'.ascii_lshift(b'F'), b'F');
        assert_eq!(b'B'.ascii_lshift(b'F'), b'G');
        assert_eq!(b'C'.ascii_lshift(b'F'), b'H');
        assert_eq!(b'D'.ascii_lshift(b'F'), b'I');
        assert_eq!(b'E'.ascii_lshift(b'F'), b'J');
        assert_eq!(b'F'.ascii_lshift(b'F'), b'K');
        assert_eq!(b'G'.ascii_lshift(b'F'), b'L');
        assert_eq!(b'H'.ascii_lshift(b'F'), b'M');
        assert_eq!(b'I'.ascii_lshift(b'F'), b'N');
        assert_eq!(b'J'.ascii_lshift(b'F'), b'O');
        assert_eq!(b'K'.ascii_lshift(b'F'), b'P');
        assert_eq!(b'L'.ascii_lshift(b'F'), b'Q');
        assert_eq!(b'M'.ascii_lshift(b'F'), b'R');
        assert_eq!(b'N'.ascii_lshift(b'F'), b'S');
        assert_eq!(b'O'.ascii_lshift(b'F'), b'T');
        assert_eq!(b'P'.ascii_lshift(b'F'), b'U');
        assert_eq!(b'Q'.ascii_lshift(b'F'), b'V');
        assert_eq!(b'R'.ascii_lshift(b'F'), b'W');
        assert_eq!(b'S'.ascii_lshift(b'F'), b'X');
        assert_eq!(b'T'.ascii_lshift(b'F'), b'Y');
        assert_eq!(b'U'.ascii_lshift(b'F'), b'Z');
        assert_eq!(b'V'.ascii_lshift(b'F'), b'A');
        assert_eq!(b'W'.ascii_lshift(b'F'), b'B');
        assert_eq!(b'X'.ascii_lshift(b'F'), b'C');
        assert_eq!(b'Y'.ascii_lshift(b'F'), b'D');
        assert_eq!(b'Z'.ascii_lshift(b'F'), b'E');

        assert_eq!(b'A'.ascii_lshift(b'G'), b'G');
        assert_eq!(b'B'.ascii_lshift(b'G'), b'H');
        assert_eq!(b'C'.ascii_lshift(b'G'), b'I');
        assert_eq!(b'D'.ascii_lshift(b'G'), b'J');
        assert_eq!(b'E'.ascii_lshift(b'G'), b'K');
        assert_eq!(b'F'.ascii_lshift(b'G'), b'L');
        assert_eq!(b'G'.ascii_lshift(b'G'), b'M');
        assert_eq!(b'H'.ascii_lshift(b'G'), b'N');
        assert_eq!(b'I'.ascii_lshift(b'G'), b'O');
        assert_eq!(b'J'.ascii_lshift(b'G'), b'P');
        assert_eq!(b'K'.ascii_lshift(b'G'), b'Q');
        assert_eq!(b'L'.ascii_lshift(b'G'), b'R');
        assert_eq!(b'M'.ascii_lshift(b'G'), b'S');
        assert_eq!(b'N'.ascii_lshift(b'G'), b'T');
        assert_eq!(b'O'.ascii_lshift(b'G'), b'U');
        assert_eq!(b'P'.ascii_lshift(b'G'), b'V');
        assert_eq!(b'Q'.ascii_lshift(b'G'), b'W');
        assert_eq!(b'R'.ascii_lshift(b'G'), b'X');
        assert_eq!(b'S'.ascii_lshift(b'G'), b'Y');
        assert_eq!(b'T'.ascii_lshift(b'G'), b'Z');
        assert_eq!(b'U'.ascii_lshift(b'G'), b'A');
        assert_eq!(b'V'.ascii_lshift(b'G'), b'B');
        assert_eq!(b'W'.ascii_lshift(b'G'), b'C');
        assert_eq!(b'X'.ascii_lshift(b'G'), b'D');
        assert_eq!(b'Y'.ascii_lshift(b'G'), b'E');
        assert_eq!(b'Z'.ascii_lshift(b'G'), b'F');

        assert_eq!(b'A'.ascii_lshift(b'H'), b'H');
        assert_eq!(b'B'.ascii_lshift(b'H'), b'I');
        assert_eq!(b'C'.ascii_lshift(b'H'), b'J');
        assert_eq!(b'D'.ascii_lshift(b'H'), b'K');
        assert_eq!(b'E'.ascii_lshift(b'H'), b'L');
        assert_eq!(b'F'.ascii_lshift(b'H'), b'M');
        assert_eq!(b'G'.ascii_lshift(b'H'), b'N');
        assert_eq!(b'H'.ascii_lshift(b'H'), b'O');
        assert_eq!(b'I'.ascii_lshift(b'H'), b'P');
        assert_eq!(b'J'.ascii_lshift(b'H'), b'Q');
        assert_eq!(b'K'.ascii_lshift(b'H'), b'R');
        assert_eq!(b'L'.ascii_lshift(b'H'), b'S');
        assert_eq!(b'M'.ascii_lshift(b'H'), b'T');
        assert_eq!(b'N'.ascii_lshift(b'H'), b'U');
        assert_eq!(b'O'.ascii_lshift(b'H'), b'V');
        assert_eq!(b'P'.ascii_lshift(b'H'), b'W');
        assert_eq!(b'Q'.ascii_lshift(b'H'), b'X');
        assert_eq!(b'R'.ascii_lshift(b'H'), b'Y');
        assert_eq!(b'S'.ascii_lshift(b'H'), b'Z');
        assert_eq!(b'T'.ascii_lshift(b'H'), b'A');
        assert_eq!(b'U'.ascii_lshift(b'H'), b'B');
        assert_eq!(b'V'.ascii_lshift(b'H'), b'C');
        assert_eq!(b'W'.ascii_lshift(b'H'), b'D');
        assert_eq!(b'X'.ascii_lshift(b'H'), b'E');
        assert_eq!(b'Y'.ascii_lshift(b'H'), b'F');
        assert_eq!(b'Z'.ascii_lshift(b'H'), b'G');

        assert_eq!(b'A'.ascii_lshift(b'I'), b'I');
        assert_eq!(b'B'.ascii_lshift(b'I'), b'J');
        assert_eq!(b'C'.ascii_lshift(b'I'), b'K');
        assert_eq!(b'D'.ascii_lshift(b'I'), b'L');
        assert_eq!(b'E'.ascii_lshift(b'I'), b'M');
        assert_eq!(b'F'.ascii_lshift(b'I'), b'N');
        assert_eq!(b'G'.ascii_lshift(b'I'), b'O');
        assert_eq!(b'H'.ascii_lshift(b'I'), b'P');
        assert_eq!(b'I'.ascii_lshift(b'I'), b'Q');
        assert_eq!(b'J'.ascii_lshift(b'I'), b'R');
        assert_eq!(b'K'.ascii_lshift(b'I'), b'S');
        assert_eq!(b'L'.ascii_lshift(b'I'), b'T');
        assert_eq!(b'M'.ascii_lshift(b'I'), b'U');
        assert_eq!(b'N'.ascii_lshift(b'I'), b'V');
        assert_eq!(b'O'.ascii_lshift(b'I'), b'W');
        assert_eq!(b'P'.ascii_lshift(b'I'), b'X');
        assert_eq!(b'Q'.ascii_lshift(b'I'), b'Y');
        assert_eq!(b'R'.ascii_lshift(b'I'), b'Z');
        assert_eq!(b'S'.ascii_lshift(b'I'), b'A');
        assert_eq!(b'T'.ascii_lshift(b'I'), b'B');
        assert_eq!(b'U'.ascii_lshift(b'I'), b'C');
        assert_eq!(b'V'.ascii_lshift(b'I'), b'D');
        assert_eq!(b'W'.ascii_lshift(b'I'), b'E');
        assert_eq!(b'X'.ascii_lshift(b'I'), b'F');
        assert_eq!(b'Y'.ascii_lshift(b'I'), b'G');
        assert_eq!(b'Z'.ascii_lshift(b'I'), b'H');

        assert_eq!(b'A'.ascii_lshift(b'J'), b'J');
        assert_eq!(b'B'.ascii_lshift(b'J'), b'K');
        assert_eq!(b'C'.ascii_lshift(b'J'), b'L');
        assert_eq!(b'D'.ascii_lshift(b'J'), b'M');
        assert_eq!(b'E'.ascii_lshift(b'J'), b'N');
        assert_eq!(b'F'.ascii_lshift(b'J'), b'O');
        assert_eq!(b'G'.ascii_lshift(b'J'), b'P');
        assert_eq!(b'H'.ascii_lshift(b'J'), b'Q');
        assert_eq!(b'I'.ascii_lshift(b'J'), b'R');
        assert_eq!(b'J'.ascii_lshift(b'J'), b'S');
        assert_eq!(b'K'.ascii_lshift(b'J'), b'T');
        assert_eq!(b'L'.ascii_lshift(b'J'), b'U');
        assert_eq!(b'M'.ascii_lshift(b'J'), b'V');
        assert_eq!(b'N'.ascii_lshift(b'J'), b'W');
        assert_eq!(b'O'.ascii_lshift(b'J'), b'X');
        assert_eq!(b'P'.ascii_lshift(b'J'), b'Y');
        assert_eq!(b'Q'.ascii_lshift(b'J'), b'Z');
        assert_eq!(b'R'.ascii_lshift(b'J'), b'A');
        assert_eq!(b'S'.ascii_lshift(b'J'), b'B');
        assert_eq!(b'T'.ascii_lshift(b'J'), b'C');
        assert_eq!(b'U'.ascii_lshift(b'J'), b'D');
        assert_eq!(b'V'.ascii_lshift(b'J'), b'E');
        assert_eq!(b'W'.ascii_lshift(b'J'), b'F');
        assert_eq!(b'X'.ascii_lshift(b'J'), b'G');
        assert_eq!(b'Y'.ascii_lshift(b'J'), b'H');
        assert_eq!(b'Z'.ascii_lshift(b'J'), b'I');

        assert_eq!(b'A'.ascii_lshift(b'K'), b'K');
        assert_eq!(b'B'.ascii_lshift(b'K'), b'L');
        assert_eq!(b'C'.ascii_lshift(b'K'), b'M');
        assert_eq!(b'D'.ascii_lshift(b'K'), b'N');
        assert_eq!(b'E'.ascii_lshift(b'K'), b'O');
        assert_eq!(b'F'.ascii_lshift(b'K'), b'P');
        assert_eq!(b'G'.ascii_lshift(b'K'), b'Q');
        assert_eq!(b'H'.ascii_lshift(b'K'), b'R');
        assert_eq!(b'I'.ascii_lshift(b'K'), b'S');
        assert_eq!(b'J'.ascii_lshift(b'K'), b'T');
        assert_eq!(b'K'.ascii_lshift(b'K'), b'U');
        assert_eq!(b'L'.ascii_lshift(b'K'), b'V');
        assert_eq!(b'M'.ascii_lshift(b'K'), b'W');
        assert_eq!(b'N'.ascii_lshift(b'K'), b'X');
        assert_eq!(b'O'.ascii_lshift(b'K'), b'Y');
        assert_eq!(b'P'.ascii_lshift(b'K'), b'Z');
        assert_eq!(b'Q'.ascii_lshift(b'K'), b'A');
        assert_eq!(b'R'.ascii_lshift(b'K'), b'B');
        assert_eq!(b'S'.ascii_lshift(b'K'), b'C');
        assert_eq!(b'T'.ascii_lshift(b'K'), b'D');
        assert_eq!(b'U'.ascii_lshift(b'K'), b'E');
        assert_eq!(b'V'.ascii_lshift(b'K'), b'F');
        assert_eq!(b'W'.ascii_lshift(b'K'), b'G');
        assert_eq!(b'X'.ascii_lshift(b'K'), b'H');
        assert_eq!(b'Y'.ascii_lshift(b'K'), b'I');
        assert_eq!(b'Z'.ascii_lshift(b'K'), b'J');

        assert_eq!(b'A'.ascii_lshift(b'L'), b'L');
        assert_eq!(b'B'.ascii_lshift(b'L'), b'M');
        assert_eq!(b'C'.ascii_lshift(b'L'), b'N');
        assert_eq!(b'D'.ascii_lshift(b'L'), b'O');
        assert_eq!(b'E'.ascii_lshift(b'L'), b'P');
        assert_eq!(b'F'.ascii_lshift(b'L'), b'Q');
        assert_eq!(b'G'.ascii_lshift(b'L'), b'R');
        assert_eq!(b'H'.ascii_lshift(b'L'), b'S');
        assert_eq!(b'I'.ascii_lshift(b'L'), b'T');
        assert_eq!(b'J'.ascii_lshift(b'L'), b'U');
        assert_eq!(b'K'.ascii_lshift(b'L'), b'V');
        assert_eq!(b'L'.ascii_lshift(b'L'), b'W');
        assert_eq!(b'M'.ascii_lshift(b'L'), b'X');
        assert_eq!(b'N'.ascii_lshift(b'L'), b'Y');
        assert_eq!(b'O'.ascii_lshift(b'L'), b'Z');
        assert_eq!(b'P'.ascii_lshift(b'L'), b'A');
        assert_eq!(b'Q'.ascii_lshift(b'L'), b'B');
        assert_eq!(b'R'.ascii_lshift(b'L'), b'C');
        assert_eq!(b'S'.ascii_lshift(b'L'), b'D');
        assert_eq!(b'T'.ascii_lshift(b'L'), b'E');
        assert_eq!(b'U'.ascii_lshift(b'L'), b'F');
        assert_eq!(b'V'.ascii_lshift(b'L'), b'G');
        assert_eq!(b'W'.ascii_lshift(b'L'), b'H');
        assert_eq!(b'X'.ascii_lshift(b'L'), b'I');
        assert_eq!(b'Y'.ascii_lshift(b'L'), b'J');
        assert_eq!(b'Z'.ascii_lshift(b'L'), b'K');

        assert_eq!(b'A'.ascii_lshift(b'M'), b'M');
        assert_eq!(b'B'.ascii_lshift(b'M'), b'N');
        assert_eq!(b'C'.ascii_lshift(b'M'), b'O');
        assert_eq!(b'D'.ascii_lshift(b'M'), b'P');
        assert_eq!(b'E'.ascii_lshift(b'M'), b'Q');
        assert_eq!(b'F'.ascii_lshift(b'M'), b'R');
        assert_eq!(b'G'.ascii_lshift(b'M'), b'S');
        assert_eq!(b'H'.ascii_lshift(b'M'), b'T');
        assert_eq!(b'I'.ascii_lshift(b'M'), b'U');
        assert_eq!(b'J'.ascii_lshift(b'M'), b'V');
        assert_eq!(b'K'.ascii_lshift(b'M'), b'W');
        assert_eq!(b'L'.ascii_lshift(b'M'), b'X');
        assert_eq!(b'M'.ascii_lshift(b'M'), b'Y');
        assert_eq!(b'N'.ascii_lshift(b'M'), b'Z');
        assert_eq!(b'O'.ascii_lshift(b'M'), b'A');
        assert_eq!(b'P'.ascii_lshift(b'M'), b'B');
        assert_eq!(b'Q'.ascii_lshift(b'M'), b'C');
        assert_eq!(b'R'.ascii_lshift(b'M'), b'D');
        assert_eq!(b'S'.ascii_lshift(b'M'), b'E');
        assert_eq!(b'T'.ascii_lshift(b'M'), b'F');
        assert_eq!(b'U'.ascii_lshift(b'M'), b'G');
        assert_eq!(b'V'.ascii_lshift(b'M'), b'H');
        assert_eq!(b'W'.ascii_lshift(b'M'), b'I');
        assert_eq!(b'X'.ascii_lshift(b'M'), b'J');
        assert_eq!(b'Y'.ascii_lshift(b'M'), b'K');
        assert_eq!(b'Z'.ascii_lshift(b'M'), b'L');

        assert_eq!(b'A'.ascii_lshift(b'N'), b'N');
        assert_eq!(b'B'.ascii_lshift(b'N'), b'O');
        assert_eq!(b'C'.ascii_lshift(b'N'), b'P');
        assert_eq!(b'D'.ascii_lshift(b'N'), b'Q');
        assert_eq!(b'E'.ascii_lshift(b'N'), b'R');
        assert_eq!(b'F'.ascii_lshift(b'N'), b'S');
        assert_eq!(b'G'.ascii_lshift(b'N'), b'T');
        assert_eq!(b'H'.ascii_lshift(b'N'), b'U');
        assert_eq!(b'I'.ascii_lshift(b'N'), b'V');
        assert_eq!(b'J'.ascii_lshift(b'N'), b'W');
        assert_eq!(b'K'.ascii_lshift(b'N'), b'X');
        assert_eq!(b'L'.ascii_lshift(b'N'), b'Y');
        assert_eq!(b'M'.ascii_lshift(b'N'), b'Z');
        assert_eq!(b'N'.ascii_lshift(b'N'), b'A');
        assert_eq!(b'O'.ascii_lshift(b'N'), b'B');
        assert_eq!(b'P'.ascii_lshift(b'N'), b'C');
        assert_eq!(b'Q'.ascii_lshift(b'N'), b'D');
        assert_eq!(b'R'.ascii_lshift(b'N'), b'E');
        assert_eq!(b'S'.ascii_lshift(b'N'), b'F');
        assert_eq!(b'T'.ascii_lshift(b'N'), b'G');
        assert_eq!(b'U'.ascii_lshift(b'N'), b'H');
        assert_eq!(b'V'.ascii_lshift(b'N'), b'I');
        assert_eq!(b'W'.ascii_lshift(b'N'), b'J');
        assert_eq!(b'X'.ascii_lshift(b'N'), b'K');
        assert_eq!(b'Y'.ascii_lshift(b'N'), b'L');
        assert_eq!(b'Z'.ascii_lshift(b'N'), b'M');

        assert_eq!(b'A'.ascii_lshift(b'O'), b'O');
        assert_eq!(b'B'.ascii_lshift(b'O'), b'P');
        assert_eq!(b'C'.ascii_lshift(b'O'), b'Q');
        assert_eq!(b'D'.ascii_lshift(b'O'), b'R');
        assert_eq!(b'E'.ascii_lshift(b'O'), b'S');
        assert_eq!(b'F'.ascii_lshift(b'O'), b'T');
        assert_eq!(b'G'.ascii_lshift(b'O'), b'U');
        assert_eq!(b'H'.ascii_lshift(b'O'), b'V');
        assert_eq!(b'I'.ascii_lshift(b'O'), b'W');
        assert_eq!(b'J'.ascii_lshift(b'O'), b'X');
        assert_eq!(b'K'.ascii_lshift(b'O'), b'Y');
        assert_eq!(b'L'.ascii_lshift(b'O'), b'Z');
        assert_eq!(b'M'.ascii_lshift(b'O'), b'A');
        assert_eq!(b'N'.ascii_lshift(b'O'), b'B');
        assert_eq!(b'O'.ascii_lshift(b'O'), b'C');
        assert_eq!(b'P'.ascii_lshift(b'O'), b'D');
        assert_eq!(b'Q'.ascii_lshift(b'O'), b'E');
        assert_eq!(b'R'.ascii_lshift(b'O'), b'F');
        assert_eq!(b'S'.ascii_lshift(b'O'), b'G');
        assert_eq!(b'T'.ascii_lshift(b'O'), b'H');
        assert_eq!(b'U'.ascii_lshift(b'O'), b'I');
        assert_eq!(b'V'.ascii_lshift(b'O'), b'J');
        assert_eq!(b'W'.ascii_lshift(b'O'), b'K');
        assert_eq!(b'X'.ascii_lshift(b'O'), b'L');
        assert_eq!(b'Y'.ascii_lshift(b'O'), b'M');
        assert_eq!(b'Z'.ascii_lshift(b'O'), b'N');

        assert_eq!(b'A'.ascii_lshift(b'P'), b'P');
        assert_eq!(b'B'.ascii_lshift(b'P'), b'Q');
        assert_eq!(b'C'.ascii_lshift(b'P'), b'R');
        assert_eq!(b'D'.ascii_lshift(b'P'), b'S');
        assert_eq!(b'E'.ascii_lshift(b'P'), b'T');
        assert_eq!(b'F'.ascii_lshift(b'P'), b'U');
        assert_eq!(b'G'.ascii_lshift(b'P'), b'V');
        assert_eq!(b'H'.ascii_lshift(b'P'), b'W');
        assert_eq!(b'I'.ascii_lshift(b'P'), b'X');
        assert_eq!(b'J'.ascii_lshift(b'P'), b'Y');
        assert_eq!(b'K'.ascii_lshift(b'P'), b'Z');
        assert_eq!(b'L'.ascii_lshift(b'P'), b'A');
        assert_eq!(b'M'.ascii_lshift(b'P'), b'B');
        assert_eq!(b'N'.ascii_lshift(b'P'), b'C');
        assert_eq!(b'O'.ascii_lshift(b'P'), b'D');
        assert_eq!(b'P'.ascii_lshift(b'P'), b'E');
        assert_eq!(b'Q'.ascii_lshift(b'P'), b'F');
        assert_eq!(b'R'.ascii_lshift(b'P'), b'G');
        assert_eq!(b'S'.ascii_lshift(b'P'), b'H');
        assert_eq!(b'T'.ascii_lshift(b'P'), b'I');
        assert_eq!(b'U'.ascii_lshift(b'P'), b'J');
        assert_eq!(b'V'.ascii_lshift(b'P'), b'K');
        assert_eq!(b'W'.ascii_lshift(b'P'), b'L');
        assert_eq!(b'X'.ascii_lshift(b'P'), b'M');
        assert_eq!(b'Y'.ascii_lshift(b'P'), b'N');
        assert_eq!(b'Z'.ascii_lshift(b'P'), b'O');

        assert_eq!(b'A'.ascii_lshift(b'Q'), b'Q');
        assert_eq!(b'B'.ascii_lshift(b'Q'), b'R');
        assert_eq!(b'C'.ascii_lshift(b'Q'), b'S');
        assert_eq!(b'D'.ascii_lshift(b'Q'), b'T');
        assert_eq!(b'E'.ascii_lshift(b'Q'), b'U');
        assert_eq!(b'F'.ascii_lshift(b'Q'), b'V');
        assert_eq!(b'G'.ascii_lshift(b'Q'), b'W');
        assert_eq!(b'H'.ascii_lshift(b'Q'), b'X');
        assert_eq!(b'I'.ascii_lshift(b'Q'), b'Y');
        assert_eq!(b'J'.ascii_lshift(b'Q'), b'Z');
        assert_eq!(b'K'.ascii_lshift(b'Q'), b'A');
        assert_eq!(b'L'.ascii_lshift(b'Q'), b'B');
        assert_eq!(b'M'.ascii_lshift(b'Q'), b'C');
        assert_eq!(b'N'.ascii_lshift(b'Q'), b'D');
        assert_eq!(b'O'.ascii_lshift(b'Q'), b'E');
        assert_eq!(b'P'.ascii_lshift(b'Q'), b'F');
        assert_eq!(b'Q'.ascii_lshift(b'Q'), b'G');
        assert_eq!(b'R'.ascii_lshift(b'Q'), b'H');
        assert_eq!(b'S'.ascii_lshift(b'Q'), b'I');
        assert_eq!(b'T'.ascii_lshift(b'Q'), b'J');
        assert_eq!(b'U'.ascii_lshift(b'Q'), b'K');
        assert_eq!(b'V'.ascii_lshift(b'Q'), b'L');
        assert_eq!(b'W'.ascii_lshift(b'Q'), b'M');
        assert_eq!(b'X'.ascii_lshift(b'Q'), b'N');
        assert_eq!(b'Y'.ascii_lshift(b'Q'), b'O');
        assert_eq!(b'Z'.ascii_lshift(b'Q'), b'P');

        assert_eq!(b'A'.ascii_lshift(b'R'), b'R');
        assert_eq!(b'B'.ascii_lshift(b'R'), b'S');
        assert_eq!(b'C'.ascii_lshift(b'R'), b'T');
        assert_eq!(b'D'.ascii_lshift(b'R'), b'U');
        assert_eq!(b'E'.ascii_lshift(b'R'), b'V');
        assert_eq!(b'F'.ascii_lshift(b'R'), b'W');
        assert_eq!(b'G'.ascii_lshift(b'R'), b'X');
        assert_eq!(b'H'.ascii_lshift(b'R'), b'Y');
        assert_eq!(b'I'.ascii_lshift(b'R'), b'Z');
        assert_eq!(b'J'.ascii_lshift(b'R'), b'A');
        assert_eq!(b'K'.ascii_lshift(b'R'), b'B');
        assert_eq!(b'L'.ascii_lshift(b'R'), b'C');
        assert_eq!(b'M'.ascii_lshift(b'R'), b'D');
        assert_eq!(b'N'.ascii_lshift(b'R'), b'E');
        assert_eq!(b'O'.ascii_lshift(b'R'), b'F');
        assert_eq!(b'P'.ascii_lshift(b'R'), b'G');
        assert_eq!(b'Q'.ascii_lshift(b'R'), b'H');
        assert_eq!(b'R'.ascii_lshift(b'R'), b'I');
        assert_eq!(b'S'.ascii_lshift(b'R'), b'J');
        assert_eq!(b'T'.ascii_lshift(b'R'), b'K');
        assert_eq!(b'U'.ascii_lshift(b'R'), b'L');
        assert_eq!(b'V'.ascii_lshift(b'R'), b'M');
        assert_eq!(b'W'.ascii_lshift(b'R'), b'N');
        assert_eq!(b'X'.ascii_lshift(b'R'), b'O');
        assert_eq!(b'Y'.ascii_lshift(b'R'), b'P');
        assert_eq!(b'Z'.ascii_lshift(b'R'), b'Q');

        assert_eq!(b'A'.ascii_lshift(b'S'), b'S');
        assert_eq!(b'B'.ascii_lshift(b'S'), b'T');
        assert_eq!(b'C'.ascii_lshift(b'S'), b'U');
        assert_eq!(b'D'.ascii_lshift(b'S'), b'V');
        assert_eq!(b'E'.ascii_lshift(b'S'), b'W');
        assert_eq!(b'F'.ascii_lshift(b'S'), b'X');
        assert_eq!(b'G'.ascii_lshift(b'S'), b'Y');
        assert_eq!(b'H'.ascii_lshift(b'S'), b'Z');
        assert_eq!(b'I'.ascii_lshift(b'S'), b'A');
        assert_eq!(b'J'.ascii_lshift(b'S'), b'B');
        assert_eq!(b'K'.ascii_lshift(b'S'), b'C');
        assert_eq!(b'L'.ascii_lshift(b'S'), b'D');
        assert_eq!(b'M'.ascii_lshift(b'S'), b'E');
        assert_eq!(b'N'.ascii_lshift(b'S'), b'F');
        assert_eq!(b'O'.ascii_lshift(b'S'), b'G');
        assert_eq!(b'P'.ascii_lshift(b'S'), b'H');
        assert_eq!(b'Q'.ascii_lshift(b'S'), b'I');
        assert_eq!(b'R'.ascii_lshift(b'S'), b'J');
        assert_eq!(b'S'.ascii_lshift(b'S'), b'K');
        assert_eq!(b'T'.ascii_lshift(b'S'), b'L');
        assert_eq!(b'U'.ascii_lshift(b'S'), b'M');
        assert_eq!(b'V'.ascii_lshift(b'S'), b'N');
        assert_eq!(b'W'.ascii_lshift(b'S'), b'O');
        assert_eq!(b'X'.ascii_lshift(b'S'), b'P');
        assert_eq!(b'Y'.ascii_lshift(b'S'), b'Q');
        assert_eq!(b'Z'.ascii_lshift(b'S'), b'R');

        assert_eq!(b'A'.ascii_lshift(b'T'), b'T');
        assert_eq!(b'B'.ascii_lshift(b'T'), b'U');
        assert_eq!(b'C'.ascii_lshift(b'T'), b'V');
        assert_eq!(b'D'.ascii_lshift(b'T'), b'W');
        assert_eq!(b'E'.ascii_lshift(b'T'), b'X');
        assert_eq!(b'F'.ascii_lshift(b'T'), b'Y');
        assert_eq!(b'G'.ascii_lshift(b'T'), b'Z');
        assert_eq!(b'H'.ascii_lshift(b'T'), b'A');
        assert_eq!(b'I'.ascii_lshift(b'T'), b'B');
        assert_eq!(b'J'.ascii_lshift(b'T'), b'C');
        assert_eq!(b'K'.ascii_lshift(b'T'), b'D');
        assert_eq!(b'L'.ascii_lshift(b'T'), b'E');
        assert_eq!(b'M'.ascii_lshift(b'T'), b'F');
        assert_eq!(b'N'.ascii_lshift(b'T'), b'G');
        assert_eq!(b'O'.ascii_lshift(b'T'), b'H');
        assert_eq!(b'P'.ascii_lshift(b'T'), b'I');
        assert_eq!(b'Q'.ascii_lshift(b'T'), b'J');
        assert_eq!(b'R'.ascii_lshift(b'T'), b'K');
        assert_eq!(b'S'.ascii_lshift(b'T'), b'L');
        assert_eq!(b'T'.ascii_lshift(b'T'), b'M');
        assert_eq!(b'U'.ascii_lshift(b'T'), b'N');
        assert_eq!(b'V'.ascii_lshift(b'T'), b'O');
        assert_eq!(b'W'.ascii_lshift(b'T'), b'P');
        assert_eq!(b'X'.ascii_lshift(b'T'), b'Q');
        assert_eq!(b'Y'.ascii_lshift(b'T'), b'R');
        assert_eq!(b'Z'.ascii_lshift(b'T'), b'S');

        assert_eq!(b'A'.ascii_lshift(b'U'), b'U');
        assert_eq!(b'B'.ascii_lshift(b'U'), b'V');
        assert_eq!(b'C'.ascii_lshift(b'U'), b'W');
        assert_eq!(b'D'.ascii_lshift(b'U'), b'X');
        assert_eq!(b'E'.ascii_lshift(b'U'), b'Y');
        assert_eq!(b'F'.ascii_lshift(b'U'), b'Z');
        assert_eq!(b'G'.ascii_lshift(b'U'), b'A');
        assert_eq!(b'H'.ascii_lshift(b'U'), b'B');
        assert_eq!(b'I'.ascii_lshift(b'U'), b'C');
        assert_eq!(b'J'.ascii_lshift(b'U'), b'D');
        assert_eq!(b'K'.ascii_lshift(b'U'), b'E');
        assert_eq!(b'L'.ascii_lshift(b'U'), b'F');
        assert_eq!(b'M'.ascii_lshift(b'U'), b'G');
        assert_eq!(b'N'.ascii_lshift(b'U'), b'H');
        assert_eq!(b'O'.ascii_lshift(b'U'), b'I');
        assert_eq!(b'P'.ascii_lshift(b'U'), b'J');
        assert_eq!(b'Q'.ascii_lshift(b'U'), b'K');
        assert_eq!(b'R'.ascii_lshift(b'U'), b'L');
        assert_eq!(b'S'.ascii_lshift(b'U'), b'M');
        assert_eq!(b'T'.ascii_lshift(b'U'), b'N');
        assert_eq!(b'U'.ascii_lshift(b'U'), b'O');
        assert_eq!(b'V'.ascii_lshift(b'U'), b'P');
        assert_eq!(b'W'.ascii_lshift(b'U'), b'Q');
        assert_eq!(b'X'.ascii_lshift(b'U'), b'R');
        assert_eq!(b'Y'.ascii_lshift(b'U'), b'S');
        assert_eq!(b'Z'.ascii_lshift(b'U'), b'T');

        assert_eq!(b'A'.ascii_lshift(b'V'), b'V');
        assert_eq!(b'B'.ascii_lshift(b'V'), b'W');
        assert_eq!(b'C'.ascii_lshift(b'V'), b'X');
        assert_eq!(b'D'.ascii_lshift(b'V'), b'Y');
        assert_eq!(b'E'.ascii_lshift(b'V'), b'Z');
        assert_eq!(b'F'.ascii_lshift(b'V'), b'A');
        assert_eq!(b'G'.ascii_lshift(b'V'), b'B');
        assert_eq!(b'H'.ascii_lshift(b'V'), b'C');
        assert_eq!(b'I'.ascii_lshift(b'V'), b'D');
        assert_eq!(b'J'.ascii_lshift(b'V'), b'E');
        assert_eq!(b'K'.ascii_lshift(b'V'), b'F');
        assert_eq!(b'L'.ascii_lshift(b'V'), b'G');
        assert_eq!(b'M'.ascii_lshift(b'V'), b'H');
        assert_eq!(b'N'.ascii_lshift(b'V'), b'I');
        assert_eq!(b'O'.ascii_lshift(b'V'), b'J');
        assert_eq!(b'P'.ascii_lshift(b'V'), b'K');
        assert_eq!(b'Q'.ascii_lshift(b'V'), b'L');
        assert_eq!(b'R'.ascii_lshift(b'V'), b'M');
        assert_eq!(b'S'.ascii_lshift(b'V'), b'N');
        assert_eq!(b'T'.ascii_lshift(b'V'), b'O');
        assert_eq!(b'U'.ascii_lshift(b'V'), b'P');
        assert_eq!(b'V'.ascii_lshift(b'V'), b'Q');
        assert_eq!(b'W'.ascii_lshift(b'V'), b'R');
        assert_eq!(b'X'.ascii_lshift(b'V'), b'S');
        assert_eq!(b'Y'.ascii_lshift(b'V'), b'T');
        assert_eq!(b'Z'.ascii_lshift(b'V'), b'U');

        assert_eq!(b'A'.ascii_lshift(b'W'), b'W');
        assert_eq!(b'B'.ascii_lshift(b'W'), b'X');
        assert_eq!(b'C'.ascii_lshift(b'W'), b'Y');
        assert_eq!(b'D'.ascii_lshift(b'W'), b'Z');
        assert_eq!(b'E'.ascii_lshift(b'W'), b'A');
        assert_eq!(b'F'.ascii_lshift(b'W'), b'B');
        assert_eq!(b'G'.ascii_lshift(b'W'), b'C');
        assert_eq!(b'H'.ascii_lshift(b'W'), b'D');
        assert_eq!(b'I'.ascii_lshift(b'W'), b'E');
        assert_eq!(b'J'.ascii_lshift(b'W'), b'F');
        assert_eq!(b'K'.ascii_lshift(b'W'), b'G');
        assert_eq!(b'L'.ascii_lshift(b'W'), b'H');
        assert_eq!(b'M'.ascii_lshift(b'W'), b'I');
        assert_eq!(b'N'.ascii_lshift(b'W'), b'J');
        assert_eq!(b'O'.ascii_lshift(b'W'), b'K');
        assert_eq!(b'P'.ascii_lshift(b'W'), b'L');
        assert_eq!(b'Q'.ascii_lshift(b'W'), b'M');
        assert_eq!(b'R'.ascii_lshift(b'W'), b'N');
        assert_eq!(b'S'.ascii_lshift(b'W'), b'O');
        assert_eq!(b'T'.ascii_lshift(b'W'), b'P');
        assert_eq!(b'U'.ascii_lshift(b'W'), b'Q');
        assert_eq!(b'V'.ascii_lshift(b'W'), b'R');
        assert_eq!(b'W'.ascii_lshift(b'W'), b'S');
        assert_eq!(b'X'.ascii_lshift(b'W'), b'T');
        assert_eq!(b'Y'.ascii_lshift(b'W'), b'U');
        assert_eq!(b'Z'.ascii_lshift(b'W'), b'V');

        assert_eq!(b'A'.ascii_lshift(b'X'), b'X');
        assert_eq!(b'B'.ascii_lshift(b'X'), b'Y');
        assert_eq!(b'C'.ascii_lshift(b'X'), b'Z');
        assert_eq!(b'D'.ascii_lshift(b'X'), b'A');
        assert_eq!(b'E'.ascii_lshift(b'X'), b'B');
        assert_eq!(b'F'.ascii_lshift(b'X'), b'C');
        assert_eq!(b'G'.ascii_lshift(b'X'), b'D');
        assert_eq!(b'H'.ascii_lshift(b'X'), b'E');
        assert_eq!(b'I'.ascii_lshift(b'X'), b'F');
        assert_eq!(b'J'.ascii_lshift(b'X'), b'G');
        assert_eq!(b'K'.ascii_lshift(b'X'), b'H');
        assert_eq!(b'L'.ascii_lshift(b'X'), b'I');
        assert_eq!(b'M'.ascii_lshift(b'X'), b'J');
        assert_eq!(b'N'.ascii_lshift(b'X'), b'K');
        assert_eq!(b'O'.ascii_lshift(b'X'), b'L');
        assert_eq!(b'P'.ascii_lshift(b'X'), b'M');
        assert_eq!(b'Q'.ascii_lshift(b'X'), b'N');
        assert_eq!(b'R'.ascii_lshift(b'X'), b'O');
        assert_eq!(b'S'.ascii_lshift(b'X'), b'P');
        assert_eq!(b'T'.ascii_lshift(b'X'), b'Q');
        assert_eq!(b'U'.ascii_lshift(b'X'), b'R');
        assert_eq!(b'V'.ascii_lshift(b'X'), b'S');
        assert_eq!(b'W'.ascii_lshift(b'X'), b'T');
        assert_eq!(b'X'.ascii_lshift(b'X'), b'U');
        assert_eq!(b'Y'.ascii_lshift(b'X'), b'V');
        assert_eq!(b'Z'.ascii_lshift(b'X'), b'W');

        assert_eq!(b'A'.ascii_lshift(b'Y'), b'Y');
        assert_eq!(b'B'.ascii_lshift(b'Y'), b'Z');
        assert_eq!(b'C'.ascii_lshift(b'Y'), b'A');
        assert_eq!(b'D'.ascii_lshift(b'Y'), b'B');
        assert_eq!(b'E'.ascii_lshift(b'Y'), b'C');
        assert_eq!(b'F'.ascii_lshift(b'Y'), b'D');
        assert_eq!(b'G'.ascii_lshift(b'Y'), b'E');
        assert_eq!(b'H'.ascii_lshift(b'Y'), b'F');
        assert_eq!(b'I'.ascii_lshift(b'Y'), b'G');
        assert_eq!(b'J'.ascii_lshift(b'Y'), b'H');
        assert_eq!(b'K'.ascii_lshift(b'Y'), b'I');
        assert_eq!(b'L'.ascii_lshift(b'Y'), b'J');
        assert_eq!(b'M'.ascii_lshift(b'Y'), b'K');
        assert_eq!(b'N'.ascii_lshift(b'Y'), b'L');
        assert_eq!(b'O'.ascii_lshift(b'Y'), b'M');
        assert_eq!(b'P'.ascii_lshift(b'Y'), b'N');
        assert_eq!(b'Q'.ascii_lshift(b'Y'), b'O');
        assert_eq!(b'R'.ascii_lshift(b'Y'), b'P');
        assert_eq!(b'S'.ascii_lshift(b'Y'), b'Q');
        assert_eq!(b'T'.ascii_lshift(b'Y'), b'R');
        assert_eq!(b'U'.ascii_lshift(b'Y'), b'S');
        assert_eq!(b'V'.ascii_lshift(b'Y'), b'T');
        assert_eq!(b'W'.ascii_lshift(b'Y'), b'U');
        assert_eq!(b'X'.ascii_lshift(b'Y'), b'V');
        assert_eq!(b'Y'.ascii_lshift(b'Y'), b'W');
        assert_eq!(b'Z'.ascii_lshift(b'Y'), b'X');

        assert_eq!(b'A'.ascii_lshift(b'Z'), b'Z');
        assert_eq!(b'B'.ascii_lshift(b'Z'), b'A');
        assert_eq!(b'C'.ascii_lshift(b'Z'), b'B');
        assert_eq!(b'D'.ascii_lshift(b'Z'), b'C');
        assert_eq!(b'E'.ascii_lshift(b'Z'), b'D');
        assert_eq!(b'F'.ascii_lshift(b'Z'), b'E');
        assert_eq!(b'G'.ascii_lshift(b'Z'), b'F');
        assert_eq!(b'H'.ascii_lshift(b'Z'), b'G');
        assert_eq!(b'I'.ascii_lshift(b'Z'), b'H');
        assert_eq!(b'J'.ascii_lshift(b'Z'), b'I');
        assert_eq!(b'K'.ascii_lshift(b'Z'), b'J');
        assert_eq!(b'L'.ascii_lshift(b'Z'), b'K');
        assert_eq!(b'M'.ascii_lshift(b'Z'), b'L');
        assert_eq!(b'N'.ascii_lshift(b'Z'), b'M');
        assert_eq!(b'O'.ascii_lshift(b'Z'), b'N');
        assert_eq!(b'P'.ascii_lshift(b'Z'), b'O');
        assert_eq!(b'Q'.ascii_lshift(b'Z'), b'P');
        assert_eq!(b'R'.ascii_lshift(b'Z'), b'Q');
        assert_eq!(b'S'.ascii_lshift(b'Z'), b'R');
        assert_eq!(b'T'.ascii_lshift(b'Z'), b'S');
        assert_eq!(b'U'.ascii_lshift(b'Z'), b'T');
        assert_eq!(b'V'.ascii_lshift(b'Z'), b'U');
        assert_eq!(b'W'.ascii_lshift(b'Z'), b'V');
        assert_eq!(b'X'.ascii_lshift(b'Z'), b'W');
        assert_eq!(b'Y'.ascii_lshift(b'Z'), b'X');
        assert_eq!(b'Z'.ascii_lshift(b'Z'), b'Y');
    }
}
