use core::mem;

use io::Write;
use Result;

/// Creates a helper that will write all data from the provided buffer into the provided writer.
///
/// Once the buffer has been written the helper will return the writer + buffer,
///
/// In the case of an error from the writer the helper will discard both writer and buffer and
/// return the error.
///
/// ```
/// # #[macro_use]
/// # extern crate nb;
/// # fn main () {
/// use nb::io::write_all;
///
/// let mut buffer = [0; 3];
/// {
///     let mut writing = write_all(&mut buffer[..], [5, 6]);
///     block!(writing.poll()).ok().unwrap();
/// }
/// assert_eq!(buffer, [5, 6, 0]);
/// # }
/// ```
pub fn write_all<W: Write, B: AsRef<[u8]>>(writer: W, buffer: B) -> WriteAll<W, B> {
    WriteAll {
        state: State::Writing {
            writer,
            buffer,
            position: 0
        },
    }
}

/// TODO
pub struct WriteAll<W: Write, B: AsRef<[u8]>> {
    state: State<W, B>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error<E> {
    WriteZero,
    Other(E)
}

enum State<W: Write, B: AsRef<[u8]>> {
    Writing {
        writer: W,
        buffer: B,
        position: usize,
    },
    Empty,
}

impl<W: Write, B: AsRef<[u8]>> WriteAll<W, B> {
    /// Poll the associated writer with some bytes from buffer.
    ///
    /// If buffer has been written then will return the writer + buffer,
    ///
    /// In the case of an error from the writer will discard both writer and buffer and return the
    /// error.
    ///
    /// Otherwise will return `nb::Error::WouldBlock`.
    pub fn poll(&mut self) -> Result<(W, B), Error<W::Error>> {
        if let State::Writing { ref mut writer, ref mut buffer, ref mut position } = self.state {
            let buffer = buffer.as_ref();
            while *position < buffer.len() {
                let amount = writer.write(&buffer[*position..]).map_err(|e| e.map(Error::Other))?;
                *position += amount;
                if amount == 0 {
                    Err(Error::WriteZero)?;
                }
            }
        } else {
            panic!("wait a WriteAll after it's done");
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Writing { writer, buffer, .. } => Ok((writer, buffer)),
            State::Empty => panic!(),
        }
    }
}
