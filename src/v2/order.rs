#[cfg(feature = "json")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Defines a new ACME order object
///
/// For more information, refer to [RFC 8555 § 7.4](https://datatracker.ietf.org/doc/html/rfc8555#section-7.4)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct NewOrder {
    /// Array of requested identifiers
    pub identifiers: Vec<super::Identifier>,
    /// Requested value for certificate's notBefore value
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "json", serde(rename = "notBefore"))]
    pub not_before: Option<String>,
    /// Requested value for certificate's notAfter value
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "json", serde(rename = "notAfter"))]
    pub not_after: Option<String>,
}

#[cfg(feature = "json")]
impl NewOrder {
    /// Deserializes a NewOrder object from a JSON str
    pub fn from_str(s: &str) -> Result<NewOrder, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes a NewOrder object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Defines a new ACME order resource
///
/// For more information, refer to [RFC 8555 § 9.7.2](https://datatracker.ietf.org/doc/html/rfc8555#section-9.7.2)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Order {
    /// The status of this order.
    pub status: OrderStatus,
    /// The timestamp after which the server will consider this order invalid,
    /// encoded in the format specified in [RFC3339]. This field is REQUIRED for
    /// objects with "pending" or "valid" in the status field.
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub expires: Option<String>,
    /// An array of identifier objects that the order pertains to.
    pub identifiers: Vec<super::Identifier>,
    /// The requested value of the notBefore
    /// field in the certificate, in the date format defined in [RFC3339].
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "json", serde(rename = "notBefore"))]
    pub not_before: Option<String>,
    /// The requested value of the notAfter
    /// field in the certificate, in the date format defined in [RFC3339].
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "json", serde(rename = "notAfter"))]
    pub not_after: Option<String>,
    /// The error that occurred while processing the order, if any.
    /// This field is structured as a problem document [RFC7807].
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub error: Option<String>,
    /// For pending orders, the authorizations that the client needs to complete before the
    /// requested certificate can be issued (see Section 7.5), including
    /// unexpired authorizations that the client has completed in the past
    /// for identifiers specified in the order.  The authorizations
    /// required are dictated by server policy; there may not be a 1:1
    /// relationship between the order identifiers and the authorizations
    /// required.  For final orders (in the "valid" or "invalid" state),
    /// the authorizations that were completed.  Each entry is a URL from
    /// which an authorization can be fetched with a POST-as-GET request.
    pub authorizations: Vec<String>,
    /// A URL that a CSR must be POSTed to once all of the order's authorizations
    /// are satisfied to finalize the order. The result of a successful finalization
    /// will be the population of the certificate URL for the order.
    pub finalize: String,
    /// A URL for the certificate that has been issued in response to this order.
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub certificate: Option<String>,
}

#[cfg(feature = "json")]
impl Order {
    /// Deserializes an Order object from a JSON str
    pub fn from_str(s: &str) -> Result<Order, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes an Order object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Defines an ACME order finalize object
///
/// For more information, refer to [RFC 8555 § 7.4](https://datatracker.ietf.org/doc/html/rfc8555#section-7.4)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct OrderFinalize {
    /// CSR for the requested certificate (base64url-encoding of the DER-encoded CSR)
    #[cfg_attr(feature = "json", serde(rename = "csr"))]
    pub certificate_signing_request: String,
}

#[cfg(feature = "json")]
impl OrderFinalize {
    /// Deserializes an OrderFinalize object from a JSON str
    pub fn from_str(s: &str) -> Result<OrderFinalize, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes an OrderFinalize object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Order resource status values
///
/// For more information, refer to [RFC 8555 § 7.1.6](https://datatracker.ietf.org/doc/html/rfc8555#section-7.1.6)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum OrderStatus {
    /// Order objects are created in the "pending" state.
    #[cfg_attr(feature = "json", serde(rename = "pending"))]
    Pending,
    /// Once all of the authorizations listed in the order object are in the "valid" state,
    /// the order transitions to the "ready" state.
    #[cfg_attr(feature = "json", serde(rename = "ready"))]
    Ready,
    /// The order moves to the "processing" state after the client submits a request to the order's
    /// "finalize" URL and the CA begins the issuance process for the certificate.
    #[cfg_attr(feature = "json", serde(rename = "processing"))]
    Processing,
    /// Once the certificate is issued, the order enters the "valid" state.
    #[cfg_attr(feature = "json", serde(rename = "valid"))]
    Valid,
    /// If an error occurs at any of these stages, the order
    /// moves to the "invalid" state.  The order also moves to the "invalid"
    /// state if it expires or one of its authorizations enters a final state
    /// other than "valid" ("expired", "revoked", or "deactivated").
    #[cfg_attr(feature = "json", serde(rename = "invalid"))]
    Invalid,
}

/// Defines a certificate revocation request
///
/// For more information, refer to [RFC 8555 § 7.6](https://datatracker.ietf.org/doc/html/rfc8555#section-7.6)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct CertificateRevocation {
    /// base64url-encoding of the DER-encoded certificate to revoke
    pub certificate: String,
    /// Reason for certificate revocation
    ///
    /// For more information, refer to [RFC 5280 § 5.3.1](https://datatracker.ietf.org/doc/html/rfc5280#section-5.3.1)
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(
        feature = "json",
        serde(serialize_with = "certificate_revocation_reason_serialize")
    )]
    #[cfg_attr(
        feature = "json",
        serde(deserialize_with = "certificate_revocation_reason_deserialize")
    )]
    pub reason: Option<CertificateRevocationReason>,
}

#[cfg(feature = "json")]
impl CertificateRevocation {
    /// Deserializes a CertificateRevocation object from a JSON str
    pub fn from_str(s: &str) -> Result<CertificateRevocation, serde_json::error::Error> {
        serde_json::from_str(s)
    }

    /// Serializes a CertificateRevocation object to a JSON String
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        serde_json::to_string(self)
    }
}

/// Certificate revocation reason values
///
/// For more information, refer to [RFC 5280 § 5.3.1](https://datatracker.ietf.org/doc/html/rfc5280#section-5.3.1)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum CertificateRevocationReason {
    Unspecified,
    KeyCompromise,
    CertificateAuthorityCompromise,
    AffiliationChanged,
    Superseded,
    CessationOfOperation,
    CertificateHold,
    RemoveFromCertificateRevocationList,
    PrivilegeWithdrawn,
    AuthorityAttributeCompromise,
    Other(i32),
}

#[cfg(feature = "json")]
fn certificate_revocation_reason_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<CertificateRevocationReason>, D::Error>
where
    D: Deserializer<'de>,
{
    use self::CertificateRevocationReason::*;

    let n = String::deserialize(deserializer)?.parse::<i32>().unwrap();

    Ok(Some(match n {
        0 => Unspecified,
        1 => KeyCompromise,
        2 => CertificateAuthorityCompromise,
        3 => AffiliationChanged,
        4 => Superseded,
        5 => CessationOfOperation,
        6 => CertificateHold,
        8 => RemoveFromCertificateRevocationList,
        9 => PrivilegeWithdrawn,
        10 => AuthorityAttributeCompromise,
        _ => Other(n),
    }))
}

#[cfg(feature = "json")]
fn certificate_revocation_reason_serialize<S>(
    type_: &Option<CertificateRevocationReason>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use self::CertificateRevocationReason::*;

    serializer.serialize_i32(match type_.clone().unwrap() {
        Other(i) => i,
        Unspecified => 0,
        KeyCompromise => 1,
        CertificateAuthorityCompromise => 2,
        AffiliationChanged => 3,
        Superseded => 4,
        CessationOfOperation => 5,
        CertificateHold => 6,
        RemoveFromCertificateRevocationList => 8,
        PrivilegeWithdrawn => 9,
        AuthorityAttributeCompromise => 10,
    })
}
