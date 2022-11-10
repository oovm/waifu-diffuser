use super::*;
use sevenz_rust::{Error, Password, SevenZArchiveEntry, SevenZReader, SevenZWriter};
use std::{fs::File, io::Read};

impl DiffuserModel {}


#[test]
fn test_load() {
    load_meta("waifu-diffuser-types.7z")
}


fn load_meta(path: &Path) -> QResult {
    let mut reader = SevenZReader::open(, Password::empty()).unwrap();
    let file = "";
    let mut buffer = Vec::new();
    reader
        .for_each_entries(|entry, read| {
            if entry.is_directory {
                return Ok(true);
            }
            println!("{:?}", entry);
            if !entry.name.eq_ignore_ascii_case(file) {
                return Ok(true);
            }
            match read.read_to_end(&mut buffer) {
                Ok(_) => Ok(false),
                Err(e) => Err(Error::io(e)),
            }
        })
        .unwrap();
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
    writer.push_archive_entry(entry, Some(buffer)).unwrap();
    writer.finish().unwrap()
}
