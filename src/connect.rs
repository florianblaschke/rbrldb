use std::str::from_utf8;

use anyhow::{Result, anyhow};
use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn read(&mut self) -> Result<Option<BytesMut>> {
        loop {
            if let Ok(line) = self.parse() {
                return Ok(Some(line));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(anyhow!("connection reset by peer"));
                }
            }
        }
    }

    fn parse(&mut self) -> Result<BytesMut> {
        if !self.buffer.is_empty() {
            for i in 0..self.buffer.len() - 1 {
                if self.buffer[i] == b'$' {
                    let slice = &self.buffer[i..];
                    if let Some(pos) = slice.iter().position(|&b| b == b';') {
                        let num_bytes = get_skip_bytes(&slice[1..pos])?;
                        let skip_ahead = i + pos + 1 + num_bytes;

                        if self.buffer.len() < skip_ahead {
                            return Err(anyhow!("incomplete"));
                        }

                        let line = self.buffer.split_to(skip_ahead);
                        self.buffer.advance(2);
                        return Ok(line);
                    }
                }

                if self.buffer[i] == b'\r' && self.buffer[i + 1] == b'\n' {
                    let line = self.buffer.split_to(i);
                    self.buffer.advance(2);
                    return Ok(line);
                }
            }
        }

        Err(anyhow!("incomplete"))
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<()> {
        self.stream.write_all(data).await?;
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.stream.flush().await?;
        Ok(())
    }

    pub fn has_empty_buffer(&self) -> bool {
        self.buffer.is_empty()
    }
}

fn get_skip_bytes(slice: &[u8]) -> Result<usize> {
    Ok(from_utf8(slice)
        .map_err(|e| anyhow!("invalid utf8: {e}"))?
        .parse::<usize>()
        .map_err(|e| anyhow!("invalid number:{e}"))?)
}
