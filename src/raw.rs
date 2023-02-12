use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};


#[cfg(windows)]
pub fn in_out() -> (impl BufRead, impl Write) {
    use std::os::windows::prelude::{AsRawHandle, FromRawHandle};
    unsafe {
        let stdin = File::from_raw_handle(stdin().as_raw_handle());
        let stdout = File::from_raw_handle(stdout().as_raw_handle());
        (BufReader::new(stdin), BufWriter::new(stdout))
    }
}

#[cfg(unix)]
pub fn in_out() -> (impl BufRead, impl Write) {
    use std::os::unix::prelude::{AsRawFd, FromRawFd};
    unsafe {
        let stdin = File::from_raw_fd(stdin().as_raw_fd());
        let stdout = File::from_raw_fd(stdout().as_raw_fd());
        (BufReader::new(stdin), BufWriter::new(stdout))
    }
}