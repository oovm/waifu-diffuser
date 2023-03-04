use super::*;
use diagnostic_quick::QResult;
use sevenz_rust::{Error, Password, SevenZArchiveEntry, SevenZReader, SevenZWriter};
use std::fs::{create_dir_all, File};

impl DiffuserModel {
    pub fn load<P: AsRef<Path>>(file: P) -> QResult<Self> {
        let path = file.as_ref();
        let buffer = load_part(&path, "meta.json")?;
        let kind: ModelKind = serde_json5::from_slice(&buffer).unwrap();
        Ok(DiffuserModel { kind, path: path.canonicalize()? })
    }
    pub fn save_meta<P: AsRef<Path>>(&self, path: P) -> QResult<usize> {
        let path = path.as_ref();
        if let Some(s) = path.parent() {
            create_dir_all(s)?
        }
        let buffer = serde_json5::to_string(&self.kind).unwrap();
        let mut writer = create_writer(path).unwrap();
        let mut entry = SevenZArchiveEntry::default();
        entry.name = "meta.json".to_string();
        entry.has_stream = true;
        writer.push_archive_entry(entry, Some(buffer.as_bytes())).unwrap();
        writer.finish().unwrap();
        Ok(buffer.len())
    }
    pub fn save_model<P: AsRef<Path>>(&self, path: P, buffer: &[u8]) -> QResult<usize> {
        let path = path.as_ref();
        let mut writer = create_writer(path).unwrap();
        let mut entry = SevenZArchiveEntry::default();
        entry.name = "model.safetensors".to_string();
        entry.has_stream = true;
        writer.push_archive_entry(entry, Some(buffer)).unwrap();
        writer.finish().unwrap();
        Ok(0)
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

fn create_writer(path: &Path) -> Result<SevenZWriter<File>, Error> {
    if path.exists() {
        match File::open(path) {
            Ok(o) => SevenZWriter::new(o),
            Err(e) => Err(Error::io(e)),
        }
    }
    else {
        SevenZWriter::create(path)
    }
}

#[test]
fn test_writer() {
    let model =
        DiffuserModel { kind: ModelKind::Clip(Box::new(ClipModel { name: "official".to_string() })), path: Default::default() };
    model.save_meta("test.diffuser").unwrap();
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
