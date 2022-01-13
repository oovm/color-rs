use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Text {
    // intern string
    color_map: Vec<String>,
    // same as Vec<char> with color bits
    characters: Vec<u32>,
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;   #[allow(unused_macros)] macro_rules! try { ( $   __expr   :   expr   ) =>   { match   $   __expr   { _serde   ::   __private   ::   Ok   ( __val   ) =>   __val   ,   _serde   ::   __private   ::   Err   ( __err   ) =>   { return   _serde   ::   __private   ::   Err   ( __err   ) ;   } } } } #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Text {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>, {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __field1, __ignore }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result { _serde::__private::Formatter::write_str(__formatter, "field identifier") }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E> where __E: _serde::de::Error, {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E> where __E: _serde::de::Error, {
                    match __value {
                        "color_map" => _serde::__private::Ok(__Field::__field0),
                        "characters" => _serde::__private::Ok(__Field::__field1),
                        _ => { _serde::__private::Ok(__Field::__ignore) }
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E> where __E: _serde::de::Error, {
                    match __value {
                        b"color_map" => _serde::__private::Ok(__Field::__field0),
                        b"characters" => _serde::__private::Ok(__Field::__field1),
                        _ => { _serde::__private::Ok(__Field::__ignore) }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>, { _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor) }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Text>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Text;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result { _serde::__private::Formatter::write_str(__formatter, "struct Text") }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error> where __A: _serde::de::SeqAccess<'de>, {
                    let __field0 = match match (_serde::de::SeqAccess::next_element::<Vec<String>>(&mut __seq)) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => { return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct Text with 2 elements")); }
                    };
                    let __field1 = match match (_serde::de::SeqAccess::next_element::<Vec<u32>>(&mut __seq)) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => { return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct Text with 2 elements")); }
                    };
                    _serde::__private::Ok(Text { color_map: __field0, characters: __field1 })
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error> where __A: _serde::de::MapAccess<'de>, {
                    let mut __field0: _serde::__private::Option<Vec<String>> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = match (_serde::de::MapAccess::next_key::<__Field>(&mut __map)) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                    } {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) { return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("color_map")); }
                                __field0 = _serde::__private::Some(match (_serde::de::MapAccess::next_value::<Vec<String>>(&mut __map)) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                                });
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) { return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("characters")); }
                                __field1 = _serde::__private::Some(match (_serde::de::MapAccess::next_value::<Vec<u32>>(&mut __map)) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                                });
                            }
                            _ => {
                                let _ = match (_serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => match (_serde::__private::de::missing_field("color_map")) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                        },
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => match (_serde::__private::de::missing_field("characters")) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => { return _serde::__private::Err(__err); }
                        },
                    };
                    _serde::__private::Ok(Text { color_map: __field0, characters: __field1 })
                }
            }
            const FIELDS: &'static [&'static str] = &["color_map", "characters"];
            _serde::Deserializer::deserialize_struct(__deserializer, "Text", FIELDS, __Visitor { marker: _serde::__private::PhantomData::<Text>, lifetime: _serde::__private::PhantomData })
        }
    }
};