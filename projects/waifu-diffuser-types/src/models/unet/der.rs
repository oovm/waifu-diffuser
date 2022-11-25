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
        let url = Url::from_directory_path(".").unwrap();
        let mut out = UNetModel {
            version: Version::new(1, 5, 0),
            net_path: "".to_string(),
            net_url: url.clone(),
            vae_encoder_path: "".to_string(),
            vae_encoder_url: url.clone(),
            vae_decoder_path: "".to_string(),
            vae_decoder_url: url.clone(),
        };
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
        formatter.write_str("Except a `Text2ImageTask` object")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "id" => {
                    self.place.id = map.next_value()?;
                }
                "text" => {
                    // self.place.text = map.next_value()?;
                }
                "font" => {
                    // self.place.font = map.next_value()?;
                }
                "font_size" => {
                    // self.place.font_size = map.next_value()?;
                }
                "font_color" => {
                    // self.place.font_color = map.next_value()?;
                }
                "background_color" => {
                    // self.place.background_color = map.next_value()?;
                }
                "w" | "width" => {
                    self.place.width = map.next_value()?;
                }
                "h" | "height" => {
                    self.place.height = map.next_value()?;
                }
                "output" => {
                    // self.place.output = map.next_value()?;
                }
                _ => {
                    println!("Unknown key: {}", key);
                    let _: serde_json::Value = map.next_value()?;
                }
            }
        }
        if self.place.id.is_empty() {
            Err(Error::missing_field("id"))?
        }
        Ok(())
    }
}
