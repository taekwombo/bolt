use super::*;
use packstream_serde::constants::marker::*;
use packstream_serde::packstream::*;

#[test]
fn empty() {
    ser_de::<Empty>(&[TINY_LIST]);

    de_ser(Empty);

    de_err::<Empty>(&[TINY_LIST + 1, 0]);
    de_err::<Empty>(&[TINY_LIST + 1]);
}

#[test]
fn single() {
    ser_de::<Single<u8>>(&[TINY_LIST + 1, 0]);

    de_ser(Single(100));

    de_err::<Single<u8>>(&[TINY_LIST]);
    de_err::<Single<u8>>(&[TINY_LIST + 2, 0, 0]);
}
