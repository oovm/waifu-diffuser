use super::*;
use diagnostic_quick::{QError, QResult};
use sevenz_rust::{Error, Password, SevenZArchiveEntry, SevenZReader, SevenZWriter};
use std::{fs::File, io::Read};

impl DiffuserModel {
    pub fn load<P: AsRef<Path>>(file: P) -> QResult<Self> {
        let path = file.as_ref().canonicalize()?;
        let buffer = load_part(&path, "meta.json")?;
        let kind: ModelKind = serde_json5::from_slice(&buffer).unwrap();
        Ok(DiffuserModel { kind, path })
    }
    pub fn save_meta<P: AsRef<Path>>(&self, path: P) -> QResult<usize> {
        let path = path.as_ref().canonicalize()?;
        let buffer = serde_json5::to_string(&self.kind).unwrap();
        if path.exists() {
            if overwrite { SevenZWriter::new(File::open(path)?) } else { Err(QError::runtime_error("File already exists"))? }
        }
        else {
            SevenZWriter::create(path).unwrap()
        }

        let mut writer = SevenZWriter::create(path).unwrap();
    }
}

#[test]
fn test_load() {
    let model = DiffuserModel::load("waifu-diffuser-types.7z");
    println!("{:?}", model);
}

fn load_part(path: &Path, file_name: &str) -> QResult<Vec<u8>> {
    let mut reader = SevenZReader::open(path, Password::empty()).unwrap();
    let mut buffer = Vec::new();
    reader
        .for_each_entries(|entry, read| {
            if entry.is_directory {
                return Ok(true);
            }
            println!("{:?}", entry);
            if !entry.name.eq_ignore_ascii_case(file_name) {
                return Ok(true);
            }
            match read.read_to_end(&mut buffer) {
                Ok(_) => Ok(false),
                Err(e) => Err(Error::io(e)),
            }
        })
        .unwrap();
    Ok(buffer)
}

#[test]
fn test_writer() {
    let mut writer = SevenZWriter::create("test.7z").unwrap();
    // let entry = SevenZArchiveEntry::
    let buffer = b"[]".as_slice();

    let mut entry = SevenZArchiveEntry::default();
    entry.name = "meta.json".to_string();

    writer.push_archive_entry(entry, Some(buffer)).unwrap();
    writer.finish().unwrap()
}

#[test]
fn test_writer2() {
    let mut writer = SevenZWriter::new(File::open("test.7z").unwrap()).unwrap();
    let buffer = b"{}".as_slice();
    let mut entry = SevenZArchiveEntry::default();
    entry.name = "meta.json".to_string();
    entry.has_stream = true;
    writer.push_archive_entry(entry, Some(buffer)).unwrap();
    writer.finish().unwrap()
}
