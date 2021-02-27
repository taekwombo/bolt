pub(crate) fn map_assertion_err (e: crate::error::Error) {
    eprintln!("{}", e);
}

#[macro_export]
macro_rules! bytes {
    ($($slice:expr),* $(,)*) => {
        {
            let mut arr: Vec<u8> = Vec::new(); 
            $(arr.extend_from_slice(&$slice);)*
            arr
        }
    }
}

#[macro_export]
macro_rules! assert_ser {
    ($($value:expr => $bytes:expr),* $(,)*) => {
        $(
            assert_eq!($crate::to_bytes(&$value).map_err($crate::macros::map_assertion_err).unwrap(), $bytes);
        )*
    }
}

#[macro_export]
macro_rules! assert_ser_err {
    ($($value:expr),* $(,)*) => {
        $(
            assert!($crate::to_bytes(&value).is_err());
        )*
    }
}

#[macro_export]
macro_rules! assert_de {
    ($($bytes:expr => $value:expr),* $(,)*) => {
        $(
            assert_eq!($crate::from_bytes(&$bytes).map_err($crate::macros::map_assertion_err).unwrap(), $value);
        )*
    }
}

#[macro_export]
macro_rules! assert_de_err {
    ($($value:expr),* $(,)*) => {
        $(
            assert!($crate::from_bytes(&$value).is_err());
        )*
    }
}

