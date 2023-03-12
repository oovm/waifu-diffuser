use std::{
    fs::File,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

mod unet;

#[test]
fn ready() {
    println!("it works!")
}

fn here<P: AsRef<Path>>(sub: P) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join(sub)
}

fn save_json<T: Serialize>(data: T, path: &Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(file, formatter);
    data.serialize(&mut ser).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
