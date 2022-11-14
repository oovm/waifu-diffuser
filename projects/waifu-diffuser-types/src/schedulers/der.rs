use std::fmt::Formatter;

use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, Visitor};

use crate::schedulers::{DDIMScheduler, DiffuserScheduler, DiffuserSchedulerKind};

struct SchedulerDeserializeVisitor {}

impl<'de> Deserialize<'de> for DiffuserScheduler {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const VARIANTS: &'static [&'static str] = &["Euler", "DDIM"];
        let __tagged = match (Deserializer::deserialize_any(deserializer, _serde::__private::de::TaggedContentVisitor::<__Field>::new("type", "internally tagged enum DiffuserScheduler"))) {
            Ok(__val) => __val,
            Err(__err) => { return _serde::__private::Err(__err); }
        };
        match __tagged.tag {
            __Field::__field0 => _serde::__private::Result::map(<Box<EulerScheduler> as _serde::Deserialize>::deserialize(_serde::__private::de::ContentDeserializer::<__D::Error>::new(__tagged.content)), DiffuserScheduler::Euler),
            __Field::__field1 => _serde::__private::Result::map(<Box<DDIMScheduler> as _serde::Deserialize>::deserialize(_serde::__private::de::ContentDeserializer::<__D::Error>::new(__tagged.content)), DiffuserScheduler::DDIM),
        }
    }
}

impl<'de> Deserialize<'de> for DiffuserSchedulerKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str()
    }
}

impl<'de> Visitor<'de> for SchedulerDeserializeVisitor {
    type Value = DiffuserScheduler;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Except one of `ddim`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        let scheduler = if v.eq_ignore_ascii_case("ddim") {
            DiffuserScheduler::DDIM(Box::new(DDIMScheduler::default()))
        } else {
            Err(E::custom("Unknown scheduler type"))?;
        };
        Ok(scheduler)
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        while let Some(key) = map.next_key::<&str>() {
            match key {}
        }
        Ok()
    }
}


struct DiffuserSchedulerKindVisitor;

impl<'de> Visitor<'de> for DiffuserSchedulerKindVisitor {
    type Value = DiffuserSchedulerKind;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Except one of `Euler`, `DDIM`")
    }
    fn visit_u64<E>(self, _v: u64) -> Result<Self::Value, E> where E: Error {
        todo!()
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        if v.eq_ignore_ascii_case("ddim") {
            Ok(DiffuserSchedulerKind::DDIM)
        } else {
            Err(E::custom("Unknown scheduler type"))?;
        }
    }
    fn visit_bytes<E>(self, _v: &[u8]) -> Result<Self::Value, E> where E: Error {
        todo!()
    }
}

impl<'de> _serde::Deserialize<'de> for __Field {
    #[inline]
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>, { Deserializer::deserialize_identifier(__deserializer, DiffuserSchedulerKindVisitor) }
}
