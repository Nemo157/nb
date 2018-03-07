use futures::{Poll, task};
use futures_io::{CoreAsyncRead, CoreAsyncWrite, CoreIoError};

use io::{Read, Write};

pub struct AsAsync<T>(T);

#[derive(Debug, Copy, Clone)]
pub enum IoError<T> {
    WriteZero(&'static str),
    UnexpectedEof(&'static str),
    Other(T),
}

pub trait AsAsyncReadExt {
    /// TODO
    fn as_async(self) -> AsAsync<Self> where Self: Sized {
        AsAsync(self)
    }
}

pub trait AsAsyncWriteExt {
    /// TODO
    fn as_async(self) -> AsAsync<Self> where Self: Sized {
        AsAsync(self)
    }
}

impl<R: Read + Sized> AsAsyncReadExt for R {
}

impl<W: Write + Sized> AsAsyncWriteExt for W {
}

impl<T> CoreIoError for IoError<T> {
    fn write_zero(msg: &'static str) -> Self {
        IoError::WriteZero(msg)
    }

    fn unexpected_eof(msg: &'static str) -> Self {
        IoError::UnexpectedEof(msg)
    }
}

impl<T> From<T> for IoError<T> {
    fn from(err: T) -> Self {
        IoError::Other(err)
    }
}

impl<T: Read> CoreAsyncRead for AsAsync<T> {
    type Error = IoError<T::Error>;

    fn poll_read(&mut self, _cx: &mut task::Context, buf: &mut [u8])
        -> Poll<usize, Self::Error>
    {
        try_nb!(self.0.read(buf))
    }
}

impl<T: Write> CoreAsyncWrite for AsAsync<T> {
    type Error = IoError<T::Error>;

    fn poll_write(&mut self, _cx: &mut task::Context, buf: &[u8])
        -> Poll<usize, Self::Error>
    {
        try_nb!(self.0.write(buf))
    }

    fn poll_flush(&mut self, _cx: &mut task::Context)
        -> Poll<(), Self::Error>
    {
        try_nb!(self.0.flush())
    }

    fn poll_close(&mut self, _cx: &mut task::Context)
        -> Poll<(), Self::Error>
    {
        try_nb!(self.0.close())
    }
}
