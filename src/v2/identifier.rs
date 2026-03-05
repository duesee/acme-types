#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// Defines the identifier object in the Order and NewAuthorization resources
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Identifier {
    /// The type of identifier.
    #[cfg_attr(feature = "json", serde(rename = "type"))]
    pub type_: IdentifierType,
    /// The identifier itself.
    pub value: String,
}

/// Order and authorization identifier types
///
/// The "ACME Identifier Types" registry lists the types of identifiers
/// that can be present in ACME authorization objects.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum IdentifierType {
    /// RFC 8555.
    #[cfg_attr(feature = "json", serde(rename = "dns"))]
    Dns,
}
