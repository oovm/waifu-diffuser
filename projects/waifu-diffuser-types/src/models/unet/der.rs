use std::fmt::Formatter;

use serde::{
    de::{Error, MapAccess, Visitor},
    Deserializer,
};

use super::*;

struct UnetVisitor<'i> {
    place: &'i mut UNetModel,
}

impl<'de> Deserialize<'de> for UNetModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = String::new();
        let path = ResourcePath::new("https://example.com", "").unwrap();
        let mut out = UNetModel::new(id, path);
        deserializer.deserialize_map(UnetVisitor { place: &mut out })?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(UnetVisitor { place })?;
        Ok(())
    }
}

impl<'i, 'de> Visitor<'de> for UnetVisitor<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Except a `UNetModel` object")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "version" => {
                    self.place.version = map.next_value()?;
                }
                "net" | "unet" => {
                    self.place.net = map.next_value()?;
                }
                "vae_encoder" | "vae-encoder" => {
                    self.place.vae_encoder = map.next_value()?;
                }
                "vae_decoder" | "vae-decoder" => {
                    self.place.vae_decoder = map.next_value()?;
                }
                "preview" => {
                    self.place.preview = map.next_value()?;
                }
                _ => {
                    println!("Unknown key: {}", key);
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }
        if self.place.net.local.to_string_lossy().eq("") {
            Err(Error::missing_field("net"))?
        }
        Ok(())
    }
}
