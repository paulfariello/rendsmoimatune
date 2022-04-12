use rocket::request::FromParam;
use uuid::Uuid;

pub struct UniqId(Uuid);

static ALPHABET: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
static BASE: u128 = 64; // len of ALPHABET

impl<'r> FromParam<'r> for UniqId {
    type Error = String;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let mut quad: u128 = 0;

        for needle in param.bytes().rev() {
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
