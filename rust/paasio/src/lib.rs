use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reads: usize,
    bytes_through: usize,
    wrapped: R,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reads: 0,
            bytes_through: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }
}

pub struct WriteStats<W> {
    writes: usize,
    bytes_through: usize,
    wrapped: W,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writes: 0,
            bytes_through: 0,
            wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        // let r = self.wrapped.write(buf);
        // match r {
        //     Ok(n) => {
        //         self.bytes_through += n;
        //         Ok(n)
        //     }
        //     Err(e) => Err(e),
        // }
        let r = self.wrapped.write(buf)?;
        self.bytes_through += r;
        Ok(r)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
