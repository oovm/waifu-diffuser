use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use url::{ParseError, Url};

#[derive(Debug)]
pub struct ModelPath {
    pub network_path: Url,
    pub relative_path: PathBuf,
}

impl Display for ModelPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for ModelPath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;
        println!("url: {:?}", url);
        Ok(Self { network_path: url, relative_path: Default::default() })
    }
}

#[test]
fn test() {
    let s = "https://api.github.com?local=./a";
    println!("{:?}", ModelPath::from_str(s))
}
