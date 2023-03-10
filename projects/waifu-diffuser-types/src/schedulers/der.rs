#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;   #[allow(unused_macros)] macro_rules! try { ( $   __expr   :   expr   ) =>   { match   $   __expr   { _serde   ::   __private   ::   Ok   ( __val   ) =>   __val   ,   _serde   ::   __private   ::   Err   ( __err   ) =>   { return   _serde   ::   __private   ::   Err   ( __err   ) ;   } } } } #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DiffuserScheduler {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>, {
            #[allow(non_camel_case_types)]
            enum __Field { __field0 }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result { _serde::__private::Formatter::write_str(__formatter, "variant identifier") }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E> where __E: _serde::de::Error, {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(_serde::de::Unexpected::Unsigned(__value), &"variant index 0 <= i < 1")),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E> where __E: _serde::de::Error, {
                    match __value {
                        "DDIM" => _serde::__private::Ok(__Field::__field0),
                        _ => { _serde::__private::Err(_serde::de::Error::unknown_variant(__value, VARIANTS)) }
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E> where __E: _serde::de::Error, {
                    match __value {
                        b"DDIM" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_variant(__value, VARIANTS))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>, { _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor) }
            }
            const VARIANTS: &'static [&'static str] = &["DDIM"];
            let __tagged = match (_serde::Deserializer::deserialize_any(__deserializer, _serde::__private::de::TaggedContentVisitor::<__Field>::new("type", "internally tagged enum DiffuserScheduler"))) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
            };
            match __tagged.tag { __Field::__field0 => _serde::__private::Result::map(<Box<DDIMScheduler> as _serde::Deserialize>::deserialize(_serde::__private::de::ContentDeserializer::<__D::Error>::new(__tagged.content)), DiffuserScheduler::DDIM), }
        }
    }
};