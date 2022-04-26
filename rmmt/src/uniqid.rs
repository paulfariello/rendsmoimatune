#[cfg(feature = "rocket")]
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniqId(Uuid);

static ALPHABET: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
static BASE: u128 = 64; // len of ALPHABET

impl TryFrom<String> for UniqId {
    type Error = String;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        UniqId::try_from(string.as_str())
    }
}

impl TryFrom<&str> for UniqId {
    type Error = String;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut quad: u128 = 0;

        for needle in string.bytes().rev() {
            let value: u128 = ALPHABET
                .iter()
                .position(|c| *c == needle)
                .ok_or(format!("Invalid input char {}", needle))?
                .try_into()
                .unwrap();
            quad = quad
                .checked_mul(BASE)
                .ok_or(format!("u128 mul overflow: {}", quad))?;
            quad = quad
                .checked_add(value)
                .ok_or(format!("u128 add overflow: {}", quad))?;
        }

        let uuid = Uuid::from_u128(quad.swap_bytes());

        Ok(UniqId(uuid))
    }
}

impl Into<Uuid> for UniqId {
    fn into(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for UniqId {
    fn from(uuid: Uuid) -> Self {
        UniqId(uuid)
    }
}

impl ToString for UniqId {
    fn to_string(&self) -> String {
        let mut raw = vec![];
        let mut quad: u128 = u128::from_le_bytes(*self.0.as_bytes()).swap_bytes();

        while quad > 0 {
            raw.push(ALPHABET[(quad % BASE) as usize]);
            quad = quad / BASE;
        }

        String::from_utf8(raw).unwrap()
    }
}

impl PartialEq<Uuid> for UniqId {
    fn eq(&self, other: &Uuid) -> bool {
        self.0 == *other
    }
}

impl PartialEq<UniqId> for Uuid {
    fn eq(&self, other: &UniqId) -> bool {
        *self == other.0
    }
}


#[cfg(feature = "rocket")]
impl<'r> FromParam<'r> for UniqId {
    type Error = String;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        param.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use std::str::FromStr;

    #[test]
    fn uniq_id_from_str() {
        // Given
        let string = "fvL4hh9nvSJlDxOn9L3foC";

        // When
        let uniq_id = UniqId::from_param(&string);

        // Then
        assert!(uniq_id.is_ok());
        assert_eq!(uniq_id.unwrap().0, Uuid::from_str("a87f72fd-9cec-4394-94af-9fd861e0bbdf").unwrap());
    }

    #[test]
    fn uniq_id_from_uuid() {
        // Given
        let uuid = Uuid::from_str("a87f72fd-9cec-4394-94af-9fd861e0bbdf").unwrap();

        // When
        let uniq_id: UniqId = uuid.clone().into();

        // Then
        assert_eq!(uniq_id.0, uuid);
    }

    #[test]
    fn uniq_id_to_string() {
        // Given
        let uuid = Uuid::from_str("a87f72fd-9cec-4394-94af-9fd861e0bbdf").unwrap();
        let uniq_id: UniqId = uuid.clone().into();

        // When
        let string = uniq_id.to_string();

        // Then
        assert_eq!(&string, "fvL4hh9nvSJlDxOn9L3foC");
    }
}
