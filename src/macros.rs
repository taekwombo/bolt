// TODO: Documentation with examples
macro_rules! structure_access {
    ($map_access:ident, $structure:ident, no_sig_key) => {{
        check!(__key, $map_access, $crate::constants::STRUCTURE_FIELDS_KEY);

        let __fields = $map_access
            .next_value::<<$structure as $crate::value::structure::BoltStructure>::Fields>(
        )?;
        check!(__key, $map_access);

        __fields
    }};
    ($map_access:ident, $structure:ident) => {{
        check!(__key, $map_access, $crate::constants::STRUCTURE_SIG_KEY);
        check!(__sig, $map_access, $structure::SIG);
        check!(__key, $map_access, $crate::constants::STRUCTURE_FIELDS_KEY);

        let __fields = $map_access
            .next_value::<<$structure as $crate::value::structure::BoltStructure>::Fields>(
        )?;
        check!(__key, $map_access);

        __fields
    }};
}

macro_rules! check {
    (__key, $map_access:ident, $expected:path) => {{
        match $map_access.next_key::<&str>()? {
            Some(__key) if __key == $expected => (),
            Some(__key) => unexpected_key_access!(__key, $expected),
            None => unexpected_key_access!($expected),
        }
    }};
    (__key, $map_access:ident) => {{
        if let Some(__key) = $map_access.next_key::<&str>()? {
            unexpected_key_access!(__key, "to be None");
        }
    }};
    (__sig, $map_access:ident, $expected:path) => {{
        let __signature = $map_access.next_value::<u8>()?;
        if __signature != $expected {
            return Err(<V::Error as ::serde::de::Error>::custom(format!(
                "Expected {:#04x} signature. Got {:#04x} instead.",
                $expected, __signature,
            )));
        }
    }};
}

macro_rules! unexpected_key_access {
    ($key:ident, $expected:expr) => {
        return Err(<V::Error as ::serde::de::Error>::custom(format!(
            "Expected key '{}', got '{}' instead.",
            $expected, $key,
        )));
    };
    ($expected:path) => {
        return Err(<V::Error as ::serde::de::Error>::custom(format!(
            "Expected key '{}', got None instead.",
            $expected,
        )));
    };
}

macro_rules! serialize_length {
    ($sig:path, $len:path) => {
        (($sig as usize) << 56) + ($len as usize)
    };
}

macro_rules! value_map {
    ($($key:literal => $value:expr,)*) => {
        {
            let mut __map = ::std::collections::HashMap::new();
            __map.insert(String::from("signature"), $crate::value::Value::I64(<Self as $crate::value::structure::BoltStructure>::SIG as i64));
            $(__map.insert(String::from($key), $value);)*
            $crate::value::Value::Map(__map)
        }
    }
}
