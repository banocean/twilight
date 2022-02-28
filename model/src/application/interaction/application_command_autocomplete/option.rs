use serde::{Deserialize, Serialize};

/// Data received when a user partially fills in a command option.
///
/// [`value`]: Self::value
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ApplicationCommandAutocompleteDataOption {
    #[serde(default)]
    pub focused: bool,
    #[serde(rename = "type")]
    pub kind: ApplicationCommandAutocompleteDataOptionType,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandAutocompleteDataOption>,
    pub value: Option<String>,
}

/// Type of option data received.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(from = "u8", into = "u8")]
pub enum ApplicationCommandAutocompleteDataOptionType {
    SubCommand,
    SubCommandGroup,
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Number ,
    Attachment ,
    Unknown(u8),
}

impl From<u8> for ApplicationCommandAutocompleteDataOptionType {
    fn from(value: u8) -> Self {
        match value {
            1 => ApplicationCommandAutocompleteDataOptionType::SubCommand,
            2 => ApplicationCommandAutocompleteDataOptionType::SubCommandGroup,
            3 => ApplicationCommandAutocompleteDataOptionType::String,
            4 => ApplicationCommandAutocompleteDataOptionType::Integer,
            5 => ApplicationCommandAutocompleteDataOptionType::Boolean,
            6 => ApplicationCommandAutocompleteDataOptionType::User,
            7 => ApplicationCommandAutocompleteDataOptionType::Channel,
            8 => ApplicationCommandAutocompleteDataOptionType::Role,
            9 => ApplicationCommandAutocompleteDataOptionType::Mentionable,
            10 => ApplicationCommandAutocompleteDataOptionType::Number,
            11 => ApplicationCommandAutocompleteDataOptionType::Attachment,
            unknown => ApplicationCommandAutocompleteDataOptionType::Unknown(unknown),
        }
    }
}

impl From<ApplicationCommandAutocompleteDataOptionType> for u8 {
    fn from(value: ApplicationCommandAutocompleteDataOptionType) -> Self {
        match value {
            ApplicationCommandAutocompleteDataOptionType::SubCommand => 1,
            ApplicationCommandAutocompleteDataOptionType::SubCommandGroup => 2,
            ApplicationCommandAutocompleteDataOptionType::String => 3,
            ApplicationCommandAutocompleteDataOptionType::Integer => 4,
            ApplicationCommandAutocompleteDataOptionType::Boolean => 5,
            ApplicationCommandAutocompleteDataOptionType::User => 6,
            ApplicationCommandAutocompleteDataOptionType::Channel => 7,
            ApplicationCommandAutocompleteDataOptionType::Role => 8,
            ApplicationCommandAutocompleteDataOptionType::Mentionable => 9,
            ApplicationCommandAutocompleteDataOptionType::Number => 10,
            ApplicationCommandAutocompleteDataOptionType::Attachment => 11,
            ApplicationCommandAutocompleteDataOptionType::Unknown(unknown) => unknown,
        }
    }
}
