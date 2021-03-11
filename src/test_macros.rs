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
macro_rules! assert_ser_de {
    ($($value:expr),* $(,)*) => {
        $(
            assert_eq!($value, $crate::from_bytes(&$crate::to_bytes(&$value).unwrap()).unwrap());
        )*
    }
}

#[macro_export]
macro_rules! assert_ser {
    (ok $var:ident $bytes:expr) => {
        if $var.is_err() {
            eprintln!("{:?}", $var);
        }
        assert!($var.is_ok());
        assert_eq!($var.unwrap(), $bytes);
    };
    (err $var:ident $error:expr) => {
        assert!($var.is_err());
        assert_eq!($var.unwrap_err(), $error);
    };
    ($($ok_err:tt {$($value:expr => $expected:expr),* $(,)*})+) => {
        $(
            $(
               let __v = $crate::to_bytes(&$value);
               assert_ser!($ok_err __v $expected);
            )*
        )+
    }
}

#[macro_export]
macro_rules! assert_de {
    (ok $var:ident $value:expr) => {
        if $var.is_err() {
            eprintln!("{:?}", $var);
        }
        assert!($var.is_ok());
        assert_eq!($var.unwrap(), $value);
    };
    (err $var:ident $value:expr) => {
        assert!($var.is_err());
        assert_eq!($var.unwrap_err(), $value);
    };
    ($($ok_err:tt with $method:ident into $t:ty {$($source:expr => $value:expr),* $(,)*})+) => {
        $(
            $(
                let __v = $crate::$method::<$t>($source);
                assert_de!($ok_err __v $value);
            )*
        )+
    };
}
