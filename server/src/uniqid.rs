use rocket::request::FromParam;
use uuid::Uuid;

pub struct UniqId(Uuid);

static ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
static BASE: u128 = 62; // len of ALPHABET

impl<'r> FromParam<'r> for UniqId {
    type Error = String;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        let mut quad: u128 = 0;

        for needle in param.bytes() {
            let value: u128 = ALPHABET
                .iter()
                .position(|c| *c == needle)
                .ok_or("Invalid input".to_string())?
                .try_into()
                .unwrap();
            quad = quad.checked_mul(BASE).ok_or("Invalid input".to_string())?;
            quad = quad.checked_add(value).ok_or("Invalid input".to_string())?;
        }

        let uuid = Uuid::from_u128(quad);

        Ok(UniqId(uuid))
    }
}

impl Into<Uuid> for UniqId {
    fn into(self) -> Uuid {
        self.0
    }
}
