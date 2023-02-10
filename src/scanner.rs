use std::io::BufRead;

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitAsciiWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self { reader, buf_str: Vec::new(), buf_iter: "".split_ascii_whitespace() }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            unsafe { self.buf_str.set_len(0); }
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}