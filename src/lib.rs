#![doc(test(attr(deny(warnings))))]

//! Joachim Henke's basE91 encoding implementation for Rust
//! http://base91.sourceforge.net

#[rustfmt::skip]
const ENTAB: [u8; 91] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M',
    b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm',
    b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'!', b'#', b'$',
    b'%', b'&', b'(', b')', b'*', b'+', b',', b'.', b'/', b':', b';', b'<', b'=',
    b'>', b'?', b'@', b'[', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~', b'"'
];

#[rustfmt::skip]
const DETAB: [u8; 256] = [
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 62, 90, 63, 64, 65, 66, 91, 67, 68, 69, 70, 71, 91, 72, 73,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 74, 75, 76, 77, 78, 79,
    80,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 81, 91, 82, 83, 84,
    85, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 86, 87, 88, 89, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91,
    91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91, 91
];

pub struct Encoder<I> {
    data: I,
    secondary: u8,
    has_secondary: bool,
    rem: u32,
    shift: u32,
}

impl<I> Iterator for Encoder<I>
where
    I: Iterator<Item = u8>,
{
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<u8> {
        let mut x = self;
        if x.has_secondary {
            x.has_secondary = false;
            return Some(x.secondary);
        }

        while let Some(b) = x.data.next() {
            x.rem |= (b as u32) << x.shift;
            x.shift += 8;

            if x.shift > 13 {
                let mut key = x.rem & 8191;
                if key > 88 {
                    x.rem >>= 13;
                    x.shift -= 13;
                } else {
                    key = x.rem & 16383;
                    x.rem >>= 14;
                    x.shift -= 14;
                }

                x.secondary = ENTAB[(key / 91) as usize];
                x.has_secondary = true;
                return Some(ENTAB[(key % 91) as usize]);
            }
        }

        if x.shift > 0 {
            let r = Some(ENTAB[(x.rem % 91) as usize]);
            if x.shift > 7 || x.rem > 90 {
                x.has_secondary = true;
                x.secondary = ENTAB[(x.rem / 91) as usize];
            }
            x.shift = 0;
            r
        } else {
            None
        }
    }
}

pub fn iter_encode<I>(data: I) -> Encoder<I> {
    Encoder {
        data,
        secondary: 0,
        has_secondary: false,
        rem: 0,
        shift: 0,
    }
}

pub fn iter_encode_old<I, O>(data: I, mut out: O)
where
    I: Iterator<Item = u8>,
    O: FnMut(u8),
{
    let mut key: u32;
    let mut rem: u32 = 0;
    let mut shift: u32 = 0;

    for b in data {
        rem |= (b as u32) << shift;
        shift += 8;

        if shift > 13 {
            key = rem & 8191;

            if key > 88 {
                rem >>= 13;
                shift -= 13;
            } else {
                key = rem & 16383;
                rem >>= 14;
                shift -= 14;
            }

            out(ENTAB[(key % 91) as usize]);
            out(ENTAB[(key / 91) as usize]);
        }
    }

    if shift > 0 {
        out(ENTAB[(rem % 91) as usize]);
        if shift > 7 || rem > 90 {
            out(ENTAB[(rem / 91) as usize]);
        }
    }
}

pub fn iter_decode<I, O>(data: I, mut out: O)
where
    I: Iterator<Item = u8>,
    O: FnMut(u8),
{
    let mut buf: i32 = -1;
    let mut key: i32;

    let mut rem: i32 = 0;
    let mut shift: i32 = 0;

    for b in data.map(|b| b as usize) {
        key = DETAB[b] as i32;

        if key == 91 {
            continue;
        }

        if buf == -1 {
            buf = key;
        } else {
            buf += key * 91;
            rem |= buf << shift;
            shift += if (buf & 8191) > 88 { 13 } else { 14 };

            while {
                out(rem as u8);
                rem >>= 8;
                shift -= 8;

                shift > 7
            } {}

            buf = -1;
        }
    }

    if buf != -1 {
        out((rem | buf << shift) as u8);
    }
}

pub fn slice_encode(value: &[u8]) -> Vec<u8> {
    iter_encode(value.iter().map(|x| *x)).collect()
}

pub fn slice_decode(value: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(value.len());

    iter_decode(value.iter().map(|v| *v), |v| result.push(v));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_pairs() -> Vec<(&'static str, &'static str)> {
        let data = vec![
            ("test", "fPNKd"),
            ("vest", "hPNKd"),
            (
                "5Fq99ztBNtv+NsWSdNS04dnyiC81Qf4dsbz6Y5elKaR+KVsAWoiK0SdBiVg2hC/FXpX0Zozw8Hd4",
                "qRqgWoRZ!L0/|msb}%dHM3;BQJX%1Q$XowN0=kHTcR5<Q81jMgz1qelja%$gNQva~1;1C:Zp>I.E2*Df))Xxc>Gq_JDzbC"
            )
        ];

        data
    }

    #[test]
    fn test_encode() {
        for pair in get_pairs() {
            assert_eq!(
                &String::from_utf8_lossy(&slice_encode(pair.0.as_bytes())[..]),
                pair.1
            );
        }
    }

    #[test]
    fn test_decode() {
        for pair in get_pairs() {
            assert_eq!(
                &String::from_utf8_lossy(&slice_decode(pair.1.as_bytes())[..]),
                pair.0
            );
        }
    }

    #[test]
    fn test_integrity() {
        use rand::*;
        const LEN: usize = 1024;
        let mut rng = thread_rng();
        let mut buf = [0u8; LEN];

        for _ in 0..10000 {
            for i in 0..LEN {
                buf[i] = rng.gen();
            }

            let encoded = slice_encode(&buf);
            let decoded = slice_decode(&encoded);

            assert_eq!(&decoded[..], &buf[..]);
        }
    }

    #[test]
    fn all_bytes() {
        let buf = (0..=255).chain((0..=255).rev()).collect::<Vec<u8>>();
        let encoded = slice_encode(&buf);
        let decoded = slice_decode(&encoded);
        assert_eq!(decoded, buf);
    }
}
