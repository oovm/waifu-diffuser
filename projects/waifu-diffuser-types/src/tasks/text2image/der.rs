use std::fmt::Formatter;

use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};

use super::*;

struct Text2ImageVisitor<'i> {
    place: &'i mut Text2ImageTask,
}

impl<'de> Deserialize<'de> for Text2ImageTask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = Text2ImageTask::default();
        deserializer.deserialize_map(Text2ImageVisitor { place: &mut out })?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Text2ImageVisitor { place })?;
        Ok(())
    }
}

impl<'i, 'de> Visitor<'de> for Text2ImageVisitor<'i> {
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
                // "id" => {
                //     self.place.task_id = map.next_value()?;
                // }
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
        Ok(())
    }
}
