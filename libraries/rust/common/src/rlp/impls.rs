// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bytes::{Bytes, BytesMut};
use core::{
    iter::{empty, once},
    mem, str,
};

use crate::rlp::{
    error::DecoderError,
    rlpin::Rlp,
    stream::RlpStream,
    traits::{Decodable, Encodable},
};

pub fn decode_usize(bytes: &[u8]) -> Result<usize, DecoderError> {
    match bytes.len() {
        l if l <= mem::size_of::<usize>() => {
            if bytes[0] == 0 {
                return Err(DecoderError::RlpInvalidIndirection);
            }
            let mut res = 0usize;
            for (i, byte) in bytes.iter().enumerate().take(l) {
                let shift = (l - 1 - i) * 8;
                res += (*byte as usize) << shift;
            }
            Ok(res)
        }
        _ => Err(DecoderError::RlpIsTooBig),
    }
}

impl<T: Encodable + ?Sized> Encodable for Box<T> {
    fn rlp_append(&self, s: &mut RlpStream) {
        Encodable::rlp_append(&**self, s)
    }
}

impl<T: Decodable> Decodable for Box<T> {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        T::decode(rlp).map(Box::new)
    }
}

impl Encodable for bool {
    fn rlp_append(&self, s: &mut RlpStream) {
        let as_uint = u8::from(*self);
        Encodable::rlp_append(&as_uint, s);
    }
}

impl Decodable for bool {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let as_uint = <u8 as Decodable>::decode(rlp)?;
        match as_uint {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecoderError::Custom("invalid boolean value")),
        }
    }
}

impl<'a> Encodable for &'a [u8] {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(self);
    }
}

impl Encodable for Vec<u8> {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(self);
    }
}

impl Decodable for Vec<u8> {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| Ok(bytes.to_vec()))
    }
}

impl Encodable for Bytes {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(self);
    }
}

impl Decodable for Bytes {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| Ok(Bytes::copy_from_slice(bytes)))
    }
}

impl Encodable for BytesMut {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(self);
    }
}

impl Decodable for BytesMut {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| Ok(bytes.into()))
    }
}

impl<T> Encodable for Option<T>
where
    T: Encodable,
{
    fn rlp_append(&self, s: &mut RlpStream) {
        match *self {
            None => {
                s.begin_list(0);
            }
            Some(ref value) => {
                s.begin_list(1);
                s.append(value);
            }
        }
    }
}

impl<T> Decodable for Option<T>
where
    T: Decodable,
{
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let items = rlp.item_count()?;
        match items {
            1 => rlp.val_at(0).map(Some),
            0 => Ok(None),
            _ => Err(DecoderError::RlpIncorrectListLen),
        }
    }
}

impl Encodable for u8 {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_iter(once(*self));
    }
}

impl Decodable for u8 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| match bytes.len() {
            1 => Ok(bytes[0]),
            0 => Ok(0),
            _ => Err(DecoderError::RlpIsTooBig),
        })
    }
}

macro_rules! impl_encodable_for_u {
    ($name: ident) => {
        impl Encodable for $name {
            fn rlp_append(&self, s: &mut RlpStream) {
                let bytes = |mut v: $name| -> Vec<u8> {
                    if v == 0 {
                        vec![0]
                    } else {
                        let mut buffer: Vec<u8> = vec![0_u8; 16];
                        for i in (0..=15).rev() {
                            let b: u8 = (v & 0xff) as u8;
                            buffer[i] = b;
                            v >>= 8;
                            if v == 0 && (b & 0x80) == 0 {
                                return buffer[i..].to_vec();
                            }
                        }
                        buffer
                    }
                }(*self);
                s.encoder().encode_value(&bytes);
            }
        }
    };
}

macro_rules! impl_decodable_for_u {
    ($name: ident) => {
        impl Decodable for $name {
            fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
                rlp.decoder().decode_value(|bytes| match bytes.len() {
                    0 | 1 => u8::decode(rlp).map(|v| v as $name),
                    l if l <= mem::size_of::<$name>() => {
                        let mut res = 0 as $name;
                        for (i, byte) in bytes.iter().enumerate().take(l) {
                            let shift = (l - 1 - i) * 8;
                            res += (*byte as $name) << shift;
                        }
                        Ok(res)
                    }
                    _ => Err(DecoderError::RlpIsTooBig),
                })
            }
        }
    };
}

impl_encodable_for_u!(u16);
impl_encodable_for_u!(u32);
impl_encodable_for_u!(u64);
impl_encodable_for_u!(u128);

impl_decodable_for_u!(u16);
impl_decodable_for_u!(u32);
impl_decodable_for_u!(u64);
impl_decodable_for_u!(u128);

impl Encodable for usize {
    fn rlp_append(&self, s: &mut RlpStream) {
        (*self as u64).rlp_append(s);
    }
}

impl Decodable for usize {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        u64::decode(rlp).map(|value| value as usize)
    }
}

impl<'a> Encodable for &'a str {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(self.as_bytes());
    }
}

impl Encodable for String {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(self.as_bytes());
    }
}

impl Decodable for String {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
            match str::from_utf8(bytes) {
                Ok(s) => Ok(s.to_owned()),
                // consider better error type here
                Err(_err) => Err(DecoderError::RlpExpectedToBeData),
            }
        })
    }
}

impl Encodable for i8 {
    fn rlp_append(&self, s: &mut RlpStream) {
        Encodable::rlp_append(&(*self as u8), s);
    }
}

impl Decodable for i8 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder()
            .decode_value(|bytes| match bytes.len() as u32 {
                len if len == u8::BITS / 8 => Ok(bytes[0] as i8),
                _ => Err(DecoderError::RlpInvalidLength),
            })
    }
}
