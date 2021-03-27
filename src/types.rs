macro_rules! access_check {
    ($map_access:ident, { $( $kind:tt($( $args:tt )?), )* }) => {
        $(
            access_check!($map_access $kind $($args)?);
        )*
    };

    ($map_access:ident signature $expecting:ident) => {
        {
            let __signature = $map_access.next_value::<u8>()?;
            if __signature != $expecting {
                return Err(
                    V::Error::custom(format!(
                            "Expected {:#04x} signature. Got {:#04x} instead.",
                            $expecting,
                            __signature,
                    ))
                );
            }
        }
    };

    ($map_access:ident key $expecting:ident) => {
        {
            let __key = $map_access.next_key::<&str>()?;
            match __key {
                Some(__key) => {
                    if __key != $expecting {
                        return Err(
                            V::Error::custom(format!(
                                    "Expected key {}. Got key {} insted.",
                                    $expecting,
                                    __key,
                            ))
                        );
                    }
                },
                None => return Err(V::Error::custom(format!("Expected key {} to exist", $expecting)))
            }
        }
    };

    ($map_access:ident fields) => {
        {
            let __fields = $map_access.next_value::<Vec<()>>()?;
            if !__fields.is_empty() {
                return Err(V::Error::custom(format!(
                    "Unexpected elements in structure fields"
                )));
            }
        }
    };

    ($map_access:ident key) => {
        {
            let __key = $map_access.next_key::<&str>()?;
            if __key.is_some() {
                return Err(
                    V::Error::custom(format!(
                            "Unexpected key {:?}. Expected structure key to be None",
                            __key
                    ))
                );
            }
        }
    };
}

macro_rules! unexpected_key_access {
    ($key:ident) => {
        Err(V::Error::custom(format!(
            "Expected STRUCTURE_SIG_KEY. Got {} instead.",
            $key
        )))
    };
    () => {
        Err(V::Error::custom(
            "Expected STRUCTURE_SIG_KEY. Got None instead.",
        ))
    };
}

macro_rules! serialize_length {
    ($signature:ident, $length:ident) => {
        (($signature as usize) << 56) + ($length as usize)
    };
}

mod graph;
mod message;
