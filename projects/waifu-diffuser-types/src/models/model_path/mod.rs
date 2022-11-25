use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use serde::Serialize;
use url::{ParseError, Url};

mod der;

#[derive(Debug, Serialize)]
pub struct ResourcePath {
    pub network: Url,
    pub relative: PathBuf,
}

impl Display for ResourcePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for ResourcePath {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;
        println!("url: {:?}", url);
        Ok(Self { network: url, relative: Default::default() })
    }
}

#[test]
fn test() {
    let s = "https://api.github.com/a?local=a/b/c";
    println!("{:?}", ResourcePath::from_str(s))
}
