//! Models for sending permission overwrites to Discord.

use crate::{
    guild::Permissions,
    id::{marker::GenericMarker, Id},
};
use serde::{Deserialize, Serialize};

/// Permission overwrite data for a role or member.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PermissionOverwrite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Permissions>,
    pub id: Id<GenericMarker>,
    #[serde(rename = "type")]
    pub kind: PermissionOverwriteType,
}

/// Type of a permission overwrite target.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum PermissionOverwriteType {
    /// Permission overwrite targets an individual member.
    Member,
    /// Permission overwrite targets an individual role.
    Role,
    Unknown(u8),
}

impl From<u8> for PermissionOverwriteType {
    fn from(value: u8) -> Self {
        match value {
            0 => PermissionOverwriteType::Role,
            1 => PermissionOverwriteType::Member,
            unknown => PermissionOverwriteType::Unknown(unknown)
        }
    }
}

impl From<PermissionOverwriteType> for u8 {
    fn from(value: PermissionOverwriteType) -> Self {
        match value {
            PermissionOverwriteType::Member => 1,
            PermissionOverwriteType::Role => 0,
            PermissionOverwriteType::Unknown(unknown) => unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PermissionOverwrite, PermissionOverwriteType, Permissions};
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(PermissionOverwrite: allow, deny, kind);
    assert_impl_all!(
        PermissionOverwrite: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        PermissionOverwriteType: Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );


    #[test]
    fn test_overwrite() {
        let value = PermissionOverwrite {
            allow: Some(Permissions::CREATE_INVITE),
            deny: Some(Permissions::KICK_MEMBERS),
            id: Id::new(12_345_678),
            kind: PermissionOverwriteType::Member,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PermissionOverwrite",
                    len: 4,
                },
                Token::Str("allow"),
                Token::Some,
                Token::Str("1"),
                Token::Str("deny"),
                Token::Some,
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("12345678"),
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_blank_overwrite() {
        // Test integer deser used in guild templates.
        let raw = r#"{
  "allow": "1",
  "deny": "2",
  "id": 1,
  "type": 1
}"#;

        let value = PermissionOverwrite {
            allow: Some(Permissions::CREATE_INVITE),
            deny: Some(Permissions::KICK_MEMBERS),
            id: Id::new(1),
            kind: PermissionOverwriteType::Member,
        };

        let deserialized = serde_json::from_str::<PermissionOverwrite>(raw).unwrap();

        assert_eq!(deserialized, value);

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PermissionOverwrite",
                    len: 4,
                },
                Token::Str("allow"),
                Token::Some,
                Token::Str("1"),
                Token::Str("deny"),
                Token::Some,
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_overwrite_type_name() {
        serde_test::assert_tokens(&PermissionOverwriteType::Member, &[Token::U8(1)]);
        serde_test::assert_tokens(&PermissionOverwriteType::Role, &[Token::U8(0)]);
        serde_test::assert_tokens(&PermissionOverwriteType::Unknown(99), &[Token::U8(99)]);
    }
}
