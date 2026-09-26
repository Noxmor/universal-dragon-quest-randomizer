use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};

pub struct RomReader {
    file: File,
    len: u64,
}

impl RomReader {
    #[allow(dead_code)]
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let len = file.metadata()?.len();

        Ok(Self { file, len })
    }

    #[allow(dead_code)]
    pub fn len(&self) -> u64 {
        self.len
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[allow(dead_code)]
    pub fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> io::Result<()> {
        let end = offset
            .checked_add(buf.len() as u64)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "read offset overflow"))?;

        if end > self.len {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "read extends beyond end of ROM",
            ));
        }

        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(buf)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn read<const N: usize>(&mut self, offset: u64) -> io::Result<[u8; N]> {
        let mut buf = [0u8; N];
        self.read_at(offset, &mut buf)?;
        Ok(buf)
    }

    #[allow(dead_code)]
    pub fn read_vec(&mut self, offset: u64, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        self.read_at(offset, &mut buf)?;
        Ok(buf)
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> File {
        self.file
    }
}
