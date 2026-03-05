#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

pub use super::Identifier as NewAuthorization;

#[cfg(feature = "json")]
impl NewAuthorization {
    /// Deserializes a NewAuthorization object from a JSON str
    pub fn from_str(s: &str) -> Result<NewAuthorization, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes a NewAuthorization object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Defines an ACME authorization resource
///
/// For more information, refer to [RFC 8555 § 7.1.4](https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.4)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Authorization {
    /// Authorization identifier
    pub identifier: super::Identifier,
    /// Authorization status
    pub status: AuthorizationStatus,
    /// Authorization expiration time
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub expires: Option<String>,
    /// Authorization challenge objects
    pub challenges: Vec<Challenge>,
    /// Present and true for authorizations for a domain name containing a wildcard
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub wildcard: Option<bool>,
}

#[cfg(feature = "json")]
impl Authorization {
    /// Deserializes an Authorization object from a JSON str
    pub fn from_str(s: &str) -> Result<Authorization, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes an Authorization object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Defines an update to an Authorization resource.
///
/// This can be used when the ACME client wishes to relinquish authorization to issue certificates for an identifier.
///
/// For more information, refer to [RFC 8555 § 7.5.2](https://datatracker.ietf.org/doc/html/rfc8555#section-7.5.2)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct AuthorizationUpdate {
    /// Authorization status
    pub status: AuthorizationStatus,
}

#[cfg(feature = "json")]
impl AuthorizationUpdate {
    /// Deserializes an AuthorizationUpdate object from a JSON str
    pub fn from_str(s: &str) -> Result<AuthorizationUpdate, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes an AuthorizationUpdate object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Defines an ACME authorization challenge object
///
/// For more information, refer to [RFC 8555 § 7.1.5](https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.5)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Challenge {
    /// URL to respond to challenge
    pub url: String,
    /// Challenge type
    #[cfg_attr(feature = "json", serde(rename = "type"))]
    pub type_: ChallengeType,
    /// Current status of the challenge
    pub status: ChallengeStatus,
    /// Optional challenge token - this may or may not be present depending on the challenge type
    ///
    /// For more information, refer to [RFC 8555 § 8.1](https://datatracker.ietf.org/doc/html/rfc8555#section-8.1)
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub token: Option<String>,
    /// Time at which the challenge was validated
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub validated: Option<String>,
    /// Error(s) encountered during challenge validation
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub error: Option<super::Error>,
}

#[cfg(feature = "json")]
impl Challenge {
    /// Deserializes a Challenge object from a JSON str
    pub fn from_str(s: &str) -> Result<Challenge, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes a Challenge object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Challenge resource status values
///
/// For more information, refer to [RFC 8555 § 7.1.6](https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.6)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum ChallengeStatus {
    /// Challenge objects are created in the "pending" state.
    #[cfg_attr(feature = "json", serde(rename = "pending"))]
    Pending,
    /// They transition to the "processing" state when the client responds to the challenge
    /// (see Section 7.5.1) and the server begins attempting to validate that the client has
    /// completed the challenge. Likewise, client requests for retries do not cause a state change.
    #[cfg_attr(feature = "json", serde(rename = "processing"))]
    Processing,
    /// If validation is successful, the challenge moves to the "valid" state.
    #[cfg_attr(feature = "json", serde(rename = "valid"))]
    Valid,
    /// If there is an error, the challenge moves to the "invalid" state.
    #[cfg_attr(feature = "json", serde(rename = "invalid"))]
    Invalid,
}

/// Challenge resource type values
///
/// For more information, refer to [RFC 8555 § 8](https://datatracker.ietf.org/doc/html/rfc8555#section-8)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum ChallengeType {
    /// For more information about http-01 challenges, refer to [RFC 8555 § 8.3](https://datatracker.ietf.org/doc/html/rfc8555#section-8.3)
    #[cfg_attr(feature = "json", serde(rename = "http-01"))]
    Http01,
    /// For more information about dns-01 challenges, refer to [RFC 8555 § 8.4](https://datatracker.ietf.org/doc/html/rfc8555#section-8.4)
    #[cfg_attr(feature = "json", serde(rename = "dns-01"))]
    Dns01,
}

/// Authorization resource status values
///
/// For more information, refer to [RFC 8555 § 7.1.6](https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.6)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum AuthorizationStatus {
    /// Authorization objects are created in the "pending" state.
    #[cfg_attr(feature = "json", serde(rename = "pending"))]
    Pending,
    /// If one of the challenges listed in the authorization transitions to the "valid"
    /// state, then the authorization also changes to the "valid" state.
    #[cfg_attr(feature = "json", serde(rename = "valid"))]
    Valid,
    /// If the client attempts to fulfill a challenge and fails, or if there is
    /// an error while the authorization is still pending, then the authorization
    /// transitions to the "invalid" state.
    #[cfg_attr(feature = "json", serde(rename = "invalid"))]
    Invalid,
    /// Once the authorization is in the "valid" state, it can expire ("expired"),
    #[cfg_attr(feature = "json", serde(rename = "expired"))]
    Expired,
    /// be deactivated by the client ("deactivated", see Section 7.5.2),
    #[cfg_attr(feature = "json", serde(rename = "deactivated"))]
    Deactivated,
    /// or revoked by the server ("revoked").
    #[cfg_attr(feature = "json", serde(rename = "revoked"))]
    Revoked,
}
