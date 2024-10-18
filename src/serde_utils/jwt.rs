use crate::SECRET;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Serializer};

pub fn serialize<V: Serialize, S: Serializer>(v: &V, s: S) -> Result<S::Ok, S::Error> {
    let header = &Header::default();
    let secret = SECRET.get().unwrap();
    let key = EncodingKey::from_secret(secret);
    let string = encode(header, &v, &key).unwrap();
    s.serialize_str(&string)
}
