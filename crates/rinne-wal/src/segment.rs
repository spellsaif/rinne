use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use std::path::PathBuf;

use rinne_core::Event;

use crate::{Record, WalError};

pub struct Segment {
    file: File,
    path: PathBuf,
}

impl Segment {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, WalError> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new().create(true).append(true).open(&path)?;

        Ok(Self { file, path })
    }

    pub fn append(&mut self, event: &Event) -> Result<(), WalError> {
        let record = Record::from_event(&event)?;

        self.file.write_all(record.as_bytes())?;

        self.file.flush()?;

        Ok(())
    }

    pub fn recover(&self) -> Result<Vec<Event>, WalError> {
        let mut file = File::open(&self.path)?;

        let mut events = Vec::new();

        loop {
            // Read record length
            let mut len_bytes = [0u8; 4];

            match file.read_exact(&mut len_bytes) {
                Ok(_) => {}

                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }

                Err(err) => {
                    return Err(err.into());
                }
            }

            let len = u32::from_be_bytes(len_bytes) as usize;

            // Read payload
            let mut payload = vec![0u8; len];

            match file.read_exact(&mut payload) {
                Ok(_) => {}

                Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // Partial record at end of file.
                    // Ignore it.
                    break;
                }

                Err(err) => {
                    return Err(err.into());
                }
            }

            let event = crate::decode(&payload)?;

            events.push(event);
        }

        Ok(events)
    }
}
