use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize,
)]
#[serde(from = "u8", into = "u8")]
pub enum PrivacyLevel {
    GuildOnly,
    Unknown(u8)
}
impl From<u8> for PrivacyLevel {
    fn from(value: u8) -> Self {
        match value {
            2 => PrivacyLevel::GuildOnly,
            unknown => PrivacyLevel::Unknown(unknown),
        }
    }
}

impl From<PrivacyLevel> for u8 {
    fn from(value: PrivacyLevel) -> Self {
        match value {
            PrivacyLevel::GuildOnly => 2,
            PrivacyLevel::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrivacyLevel;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&PrivacyLevel::GuildOnly, &[Token::U8(2)]);
        serde_test::assert_tokens(&PrivacyLevel::Unknown(99), &[Token::U8(99)]);
    }
}
