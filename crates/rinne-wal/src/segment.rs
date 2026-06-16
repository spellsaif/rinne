use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use rinne_core::Event;

use crate::{Record, WalError};

pub struct Segment {
    file: File,
}

impl Segment {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, WalError> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self { file })
    }

    pub fn append(&mut self, event: &Event) -> Result<(), WalError> {
        let record = Record::from_event(&event)?;

        self.file.write_all(record.as_bytes())?;

        self.file.flush()?;

        Ok(())
    }
}
