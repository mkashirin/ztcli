#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
#[allow(unused_imports)]
use show_option::prelude::*;
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use show_option::ShowOption;

    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    /// ApiToken
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "token": {
    ///      "description": "API Token. Minimum 32 characters. This token is
    /// encrypted in the database and can not be retrieved once set",
    ///      "writeOnly": true,
    ///      "examples": [
    ///        "adsf98ashdkjh3689adsfnj3$ADn"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tokenName": {
    ///      "description": "user specified token name",
    ///      "examples": [
    ///        "my-super-secret-token"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ApiToken {
        /// API Token. Minimum 32 characters. This token is encrypted in the
        /// database and can not be retrieved once set
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
        /// user specified token name
        #[serde(
            rename = "tokenName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub token_name: Option<String>,
    }

    impl From<&ApiToken> for ApiToken {
        fn from(value: &ApiToken) -> Self {
            value.clone()
        }
    }

    /// AuthMethods
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "google": {
    ///      "description": "Google OIDC ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "156162346876134683"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "local": {
    ///      "description": "email address for built-in authentication",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "user@example.com"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "oidc": {
    ///      "description": "Generic OIDC ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AuthMethods {
        /// Google OIDC ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub google: Option<String>,
        /// email address for built-in authentication
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub local: Option<String>,
        /// Generic OIDC ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub oidc: Option<String>,
    }

    impl From<&AuthMethods> for AuthMethods {
        fn from(value: &AuthMethods) -> Self {
            value.clone()
        }
    }

    /// Dns
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "domain": {
    ///      "description": "Search domain to use for DNS records",
    ///      "examples": [
    ///        "some.domain"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "servers": {
    ///      "description": "IP address of unicast DNS service",
    ///      "examples": [
    ///        [
    ///          "10.0.0.3"
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Dns {
        /// Search domain to use for DNS records
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,
        /// IP address of unicast DNS service
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub servers: Option<Vec<String>>,
    }

    impl From<&Dns> for Dns {
        fn from(value: &Dns) -> Self {
            value.clone()
        }
    }

    /// InviteStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "pending",
    ///    "accepted",
    ///    "canceled"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InviteStatus {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "accepted")]
        Accepted,
        #[serde(rename = "canceled")]
        Canceled,
    }

    impl From<&InviteStatus> for InviteStatus {
        fn from(value: &InviteStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for InviteStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Pending => write!(f, "pending"),
                Self::Accepted => write!(f, "accepted"),
                Self::Canceled => write!(f, "canceled"),
            }
        }
    }

    impl std::str::FromStr for InviteStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "pending" => Ok(Self::Pending),
                "accepted" => Ok(Self::Accepted),
                "canceled" => Ok(Self::Canceled),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for InviteStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for InviteStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for InviteStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// IpRange
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "ipRangeEnd": {
    ///      "examples": [
    ///        "10.0.0.255"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ipRangeStart": {
    ///      "examples": [
    ///        "10.0.0.1"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IpRange {
        #[serde(
            rename = "ipRangeEnd",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ip_range_end: Option<String>,
        #[serde(
            rename = "ipRangeStart",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ip_range_start: Option<String>,
    }

    impl From<&IpRange> for IpRange {
        fn from(value: &IpRange) -> Self {
            value.clone()
        }
    }

    /// Ipv4AssignMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "zt": {
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Ipv4AssignMode {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zt: Option<bool>,
    }

    impl From<&Ipv4AssignMode> for Ipv4AssignMode {
        fn from(value: &Ipv4AssignMode) -> Self {
            value.clone()
        }
    }

    /// Ipv6AssignMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "6plane": {
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "rfc4193": {
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "zt": {
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Ipv6AssignMode {
        #[serde(
            rename = "6plane",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub _6plane: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rfc4193: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zt: Option<bool>,
    }

    impl From<&Ipv6AssignMode> for Ipv6AssignMode {
        fn from(value: &Ipv6AssignMode) -> Self {
            value.clone()
        }
    }

    /// Member
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "clientVersion": {
    ///      "description": "ZeroTier version the member is running",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "1.6.3"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "clock": {
    ///      "readOnly": true,
    ///      "examples": [
    ///        1612993759070
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "config": {
    ///      "$ref": "#/components/schemas/MemberConfig"
    ///    },
    ///    "controllerId": {
    ///      "deprecated": true,
    ///      "readOnly": true,
    ///      "examples": [
    ///        "8056c2e21c"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "description": {
    ///      "description": "User defined description of the member",
    ///      "examples": [
    ///        "My super awesome cray that I got ZeroTier to run on"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "hidden": {
    ///      "description": "Whether or not the member is hidden in the UI",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "concatenation of network ID and member ID",
    ///      "deprecated": true,
    ///      "readOnly": true,
    ///      "examples": [
    ///        "8056c2e21c000001-abcdef0123"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lastOnline": {
    ///      "description": "Last seen time of the member (milliseconds since
    /// epoch).  Note: This data is considered ephemeral and may be reset to 0
    /// at any time without warning.",
    ///      "deprecated": true,
    ///      "readOnly": true,
    ///      "examples": [
    ///        1612993673254
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "lastSeen": {
    ///      "description": "Time the member last checked in with the network
    /// controller in milliseconds since epoch. Note: This data is considered
    /// ephemeral and may be reset to 0 at any time without warning.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1612993673254
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "name": {
    ///      "description": "User defined name of the member",
    ///      "examples": [
    ///        "my-cray-supercomputer"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "networkId": {
    ///      "readOnly": true,
    ///      "examples": [
    ///        "8056c2e21c000001"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "nodeId": {
    ///      "description": "ZeroTier ID of the member",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "abcdef01234"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "physicalAddress": {
    ///      "description": "IP address the member last spoke to the controller
    /// via (milliseconds since epoch).  Note: This data is considered ephemeral
    /// and may be reset to 0 at any time without warning.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "8.8.8.8"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "protocolVersion": {
    ///      "description": "ZeroTier protocol version",
    ///      "readOnly": true,
    ///      "examples": [
    ///        12
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "supportsRulesEngine": {
    ///      "description": "Whether or not the client version is new enough to
    /// support the rules engine (1.4.0+)",
    ///      "readOnly": true,
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Member {
        /// ZeroTier version the member is running
        #[serde(
            rename = "clientVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clock: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<MemberConfig>,
        #[serde(
            rename = "controllerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub controller_id: Option<String>,
        /// User defined description of the member
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// Whether or not the member is hidden in the UI
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hidden: Option<bool>,
        /// Concatenation of network ID and member ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// Last seen time of the member (milliseconds since epoch). Note:
        /// This data is considered ephemeral and may be reset to 0 at
        /// any time without warning.
        #[serde(
            rename = "lastOnline",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_online: Option<i64>,
        /// Time the member last checked in with the network controller in
        /// milliseconds since epoch. Note: This data is considered ephemeral
        /// and may be reset to 0 at any time without warning.
        #[serde(
            rename = "lastSeen",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_seen: Option<i64>,
        /// User defined name of the member
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "networkId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub network_id: Option<String>,
        /// ZeroTier ID of the member
        #[serde(
            rename = "nodeId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_id: Option<String>,
        /// IP address the member last spoke to the controller via
        /// (milliseconds since epoch). Note: This data is considered
        /// ephemeral and may be reset to 0 at any time without
        /// warning.
        #[serde(
            rename = "physicalAddress",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub physical_address: Option<String>,
        /// ZeroTier protocol version
        #[serde(
            rename = "protocolVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub protocol_version: Option<i64>,
        /// Whether or not the client version is new enough to support the
        /// rules engine (1.4.0+)
        #[serde(
            rename = "supportsRulesEngine",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supports_rules_engine: Option<bool>,
    }

    impl From<&Member> for Member {
        fn from(value: &Member) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Member {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut name =
                self.name.show_or(::serde_json::Value::Null).to_string();
            if name.to_string() == "".to_string() {
                name = "unnamed".to_string()
            };
            let node_id = self.node_id.show_or(::serde_json::Value::Null);
            let hidden = self.hidden.show_or(::serde_json::Value::Null);
            let physical_address =
                self.physical_address.show_or(::serde_json::Value::Null);

            let config = self.config.as_ref().unwrap();
            let authorized =
                config.authorized.show_or(::serde_json::Value::Null);
            let ip_assignments = config
                .ip_assignments
                .as_ref()
                .unwrap()
                .iter()
                .map(|val| format!("{}", val))
                .collect::<Vec<String>>()
                .join(", ");

            write!(
                f,
                "Member {name} (ID: {node_id})
  * Authorized: {authorized}
  * Hidden: {hidden}
  * IP assignments: [{ip_assignments}]
  * Physical address: {physical_address}"
            )
        }
    }

    /// MemberConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "activeBridge": {
    ///      "description": "Allow the member to be a bridge on the network",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "authorized": {
    ///      "description": "Is the member authorized on the network",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "capabilities": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "integer"
    ///      }
    ///    },
    ///    "creationTime": {
    ///      "description": "Time the member was created or first tried to join
    /// the network",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1599853509872
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "id": {
    ///      "description": "ID of the member node.  This is the 10 digit
    /// identifier that identifies a ZeroTier node.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "abcdef01234"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "identity": {
    ///      "description": "Public Key of the member's Identity",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "abcdef0123:0:abcdef0123abcdef0123abcdef0123abcdef0123abcdef0123abcdef0123abcdef0123"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "ipAssignments": {
    ///      "description": "List of assigned IP addresses",
    ///      "examples": [
    ///        [
    ///          "10.0.0.3"
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "lastAuthorizedTime": {
    ///      "description": "Time the member was authorized on the network",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1599853637989
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "lastDeauthorizedTime": {
    ///      "description": "Time the member was deauthorized on the network",
    ///      "readOnly": true,
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "noAutoAssignIps": {
    ///      "description": "Exempt this member from the IP auto assignment pool
    /// on a Network",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "revision": {
    ///      "description": "Member record revision count",
    ///      "readOnly": true,
    ///      "examples": [
    ///        123
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "ssoExempt": {
    ///      "description": "Allow the member to be authorized without
    /// OIDC/SSO",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "tags": {
    ///      "description": "Array of 2 member tuples of tag [ID, tag value]",
    ///      "examples": [
    ///        [
    ///          [
    ///            123,
    ///            456
    ///          ]
    ///        ]
    ///      ],
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "anyOf": [
    ///            {
    ///              "type": "integer"
    ///            },
    ///            {
    ///              "type": "boolean"
    ///            }
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    "vMajor": {
    ///      "description": "Major version of the client",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "vMinor": {
    ///      "description": "Minor version of the client",
    ///      "readOnly": true,
    ///      "examples": [
    ///        6
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "vProto": {
    ///      "description": "Protocol version of the client",
    ///      "readOnly": true,
    ///      "examples": [
    ///        12
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "vRev": {
    ///      "description": "Revision number of the client",
    ///      "readOnly": true,
    ///      "examples": [
    ///        3
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MemberConfig {
        /// Allow the member to be a bridge on the network
        #[serde(
            rename = "activeBridge",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub active_bridge: Option<bool>,
        /// Is the member authorized on the network
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authorized: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub capabilities: Option<Vec<i64>>,
        /// Time the member was created or first tried to join the network
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub creation_time: Option<i64>,
        /// ID of the member node. This is the 10 digit identifier that
        /// identifies a ZeroTier node.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// Public Key of the member's Identity
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub identity: Option<String>,
        /// List of assigned IP addresses
        #[serde(
            rename = "ipAssignments",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ip_assignments: Option<Vec<String>>,
        ///Time the member was authorized on the network
        #[serde(
            rename = "lastAuthorizedTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_authorized_time: Option<i64>,
        /// Time the member was deauthorized on the network
        #[serde(
            rename = "lastDeauthorizedTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_deauthorized_time: Option<i64>,
        /// Exempt this member from the IP auto assignment pool on a Network
        #[serde(
            rename = "noAutoAssignIps",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_auto_assign_ips: Option<bool>,
        /// Member record revision count
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub revision: Option<i64>,
        /// Allow the member to be authorized without OIDC/SSO
        #[serde(
            rename = "ssoExempt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_exempt: Option<bool>,
        /// Array of 2 member tuples of tag [ID, tag value]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<Vec<MemberConfigTagsItemItem>>>,
        /// Major version of the client
        #[serde(
            rename = "vMajor",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v_major: Option<i64>,
        /// Minor version of the client
        #[serde(
            rename = "vMinor",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v_minor: Option<i64>,
        /// Protocol version of the client
        #[serde(
            rename = "vProto",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v_proto: Option<i64>,
        /// Revision number of the client
        #[serde(
            rename = "vRev",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v_rev: Option<i64>,
    }

    impl From<&MemberConfig> for MemberConfig {
        fn from(value: &MemberConfig) -> Self {
            value.clone()
        }
    }

    /// MemberConfigTagsItemItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "type": "integer"
    ///    },
    ///    {
    ///      "type": "boolean"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum MemberConfigTagsItemItem {
        Variant0(i64),
        Variant1(bool),
    }

    impl From<&MemberConfigTagsItemItem> for MemberConfigTagsItemItem {
        fn from(value: &MemberConfigTagsItemItem) -> Self {
            value.clone()
        }
    }

    impl std::str::FromStr for MemberConfigTagsItemItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Variant0(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Variant1(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl std::convert::TryFrom<&str> for MemberConfigTagsItemItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for MemberConfigTagsItemItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for MemberConfigTagsItemItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for MemberConfigTagsItemItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }

    impl From<i64> for MemberConfigTagsItemItem {
        fn from(value: i64) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<bool> for MemberConfigTagsItemItem {
        fn from(value: bool) -> Self {
            Self::Variant1(value)
        }
    }

    /// Network object
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "description": "Network object",
    ///  "type": "object",
    ///  "properties": {
    ///    "authorizedMemberCount": {
    ///      "readOnly": true,
    ///      "examples": [
    ///        200
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "capabilitiesByName": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ]
    ///    },
    ///    "clock": {
    ///      "readOnly": true,
    ///      "examples": [
    ///        12345
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "config": {
    ///      "$ref": "#/components/schemas/NetworkConfig"
    ///    },
    ///    "description": {
    ///      "examples": [
    ///        "Some descriptive text about my network."
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "readOnly": true,
    ///      "examples": [
    ///        "8056c2e21c000001"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "onlineMemberCount": {
    ///      "description": "Note: May be 0 on endpoints returning lists of
    /// Networks",
    ///      "readOnly": true,
    ///      "examples": [
    ///        123
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "ownerId": {
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/PermissionsMap"
    ///    },
    ///    "rulesSource": {
    ///      "examples": [
    ///        "accept;"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "tagsByName": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ]
    ///    },
    ///    "totalMemberCount": {
    ///      "readOnly": true,
    ///      "examples": [
    ///        250
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Network {
        #[serde(
            rename = "authorizedMemberCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorized_member_count: Option<i64>,
        #[serde(
            rename = "capabilitiesByName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub capabilities_by_name:
            Option<::serde_json::Map<String, ::serde_json::Value>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clock: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<NetworkConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// Note: May be 0 on endpoints returning lists of Networks
        #[serde(
            rename = "onlineMemberCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub online_member_count: Option<i64>,
        #[serde(
            rename = "ownerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub owner_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub permissions: Option<PermissionsMap>,
        #[serde(
            rename = "rulesSource",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rules_source: Option<String>,
        #[serde(
            rename = "tagsByName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tags_by_name:
            Option<::serde_json::Map<String, ::serde_json::Value>>,
        #[serde(
            rename = "totalMemberCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub total_member_count: Option<i64>,
    }

    impl From<&Network> for Network {
        fn from(value: &Network) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Network {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            let id = self.id.show_or(::serde_json::Value::Null);
            let total_member_count =
                self.total_member_count.show_or(::serde_json::Value::Null);
            let online_member_count =
                self.online_member_count.show_or(::serde_json::Value::Null);
            let authorized_member_count = self
                .authorized_member_count
                .show_or(::serde_json::Value::Null);

            let config = self.config.as_ref().unwrap();
            let name = config.name.show_or(::serde_json::Value::Null);
            let private = config.private.show_or(::serde_json::Value::Null);

            write!(
                f,
                "Network {name} (ID: {id})
  * Private: {private}
  * Total member count: {total_member_count}
  * Online member count: {online_member_count}
  * Authorized member count: {authorized_member_count}"
            )
        }
    }

    /// NetworkConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "capabilities": {
    ///      "description": "Array of network capabilities",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "object"
    ///      }
    ///    },
    ///    "creationTime": {
    ///      "description": "Time the network was created",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1442292672978
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "dns": {
    ///      "$ref": "#/components/schemas/DNS"
    ///    },
    ///    "enableBroadcast": {
    ///      "description": "Enable broadcast packets on the network",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "Network ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "8056c2e21c000001"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "ipAssignmentPools": {
    ///      "description": "Range of IP addresses for the auto assign pool",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/IPRange"
    ///      }
    ///    },
    ///    "lastModified": {
    ///      "description": "Time the network was last modified",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1588184318235
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int64"
    ///    },
    ///    "mtu": {
    ///      "description": "MTU to set on the client virtual network adapter",
    ///      "examples": [
    ///        2800
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "multicastLimit": {
    ///      "description": "Maximum number of recipients per multicast or
    /// broadcast. Warning - Setting this to 0 will disable IPv4 communication
    /// on your network!",
    ///      "examples": [
    ///        32
    ///      ],
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "examples": [
    ///        "My ZeroTier Network"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "private": {
    ///      "description": "Whether or not the network is private.  If false,
    /// members will *NOT* need to be authorized to join.",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "routes": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/Route"
    ///      }
    ///    },
    ///    "rules": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "object"
    ///      }
    ///    },
    ///    "ssoConfig": {
    ///      "$ref": "#/components/schemas/NetworkSSOConfig"
    ///    },
    ///    "tags": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "object"
    ///      }
    ///    },
    ///    "v4AssignMode": {
    ///      "$ref": "#/components/schemas/IPV4AssignMode"
    ///    },
    ///    "v6AssignMode": {
    ///      "$ref": "#/components/schemas/IPV6AssignMode"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkConfig {
        /// Array of network capabilities
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub capabilities:
            Option<Vec<::serde_json::Map<String, ::serde_json::Value>>>,
        /// Time the network was created
        #[serde(
            rename = "creationTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub creation_time: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dns: Option<Dns>,
        /// Enable broadcast packets on the network
        #[serde(
            rename = "enableBroadcast",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_broadcast: Option<bool>,
        /// Network ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// Range of IP addresses for the auto assign pool
        #[serde(
            rename = "ipAssignmentPools",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ip_assignment_pools: Option<Vec<IpRange>>,
        /// Time the network was last modified
        #[serde(
            rename = "lastModified",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_modified: Option<i64>,
        /// MTU to set on the client virtual network adapter
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mtu: Option<i64>,
        /// Maximum number of recipients per multicast or broadcast. Warning -
        /// Setting this to 0 will disable IPv4 communication on your network!
        #[serde(
            rename = "multicastLimit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub multicast_limit: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Whether or not the network is private. If false, members will
        /// *NOT* need to be authorized to join.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub routes: Option<Vec<Route>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rules: Option<Vec<::serde_json::Map<String, ::serde_json::Value>>>,
        #[serde(
            rename = "ssoConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_config: Option<NetworkSsoConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<::serde_json::Map<String, ::serde_json::Value>>>,
        #[serde(
            rename = "v4AssignMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v4_assign_mode: Option<Ipv4AssignMode>,
        #[serde(
            rename = "v6AssignMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v6_assign_mode: Option<Ipv6AssignMode>,
    }

    impl From<&NetworkConfig> for NetworkConfig {
        fn from(value: &NetworkConfig) -> Self {
            value.clone()
        }
    }

    /// NetworkSsoConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "allowList": {
    ///      "description": "List of email addresses or group memberships that
    /// may SSO auth onto the network",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authorizationEndpoint": {
    ///      "description": "Authorization URL endpoint",
    ///      "readOnly": true,
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "description": "SSO client ID.  Client ID must be already
    /// configured in the Org",
    ///      "examples": [
    ///        "some-client-id"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "description": "SSO enabled/disabled on network",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "issuer": {
    ///      "description": "URL of the OIDC issuer",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "https://example.com/oidc"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "mode": {
    ///      "description": "SSO mode.  One of: `default`, `email`, `group`",
    ///      "examples": [
    ///        "default"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "provider": {
    ///      "description": "Provider type",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "keycloak"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkSsoConfig {
        /// List of email addresses or group memberships that may SSO auth onto
        /// the network
        #[serde(
            rename = "allowList",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_list: Option<Vec<String>>,
        /// Authorization URL endpoint
        #[serde(
            rename = "authorizationEndpoint",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_endpoint: Option<String>,
        /// SSO client ID.  Client ID must be already configured in the Org
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_id: Option<String>,
        /// SSO enabled/disabled on network
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        /// URL of the OIDC issuer
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        /// SSO mode.  One of: `default`, `email`, `group`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
        /// Provider type
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }

    impl From<&NetworkSsoConfig> for NetworkSsoConfig {
        fn from(value: &NetworkSsoConfig) -> Self {
            value.clone()
        }
    }

    /// OrgSsoConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "enabled": {
    ///      "description": "Enabled flag for SSO",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "issuers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SsoIssuer"
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrgSsoConfig {
        ///Enabled flag for SSO
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issuers: Vec<SsoIssuer>,
    }

    impl From<&OrgSsoConfig> for OrgSsoConfig {
        fn from(value: &OrgSsoConfig) -> Self {
            value.clone()
        }
    }

    /// Organization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "description": "Organization ID",
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "members": {
    ///      "description": "List of organization members",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrganizationMember"
    ///      }
    ///    },
    ///    "ownerEmail": {
    ///      "description": "Organization owner's email address",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "user@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ownerId": {
    ///      "description": "User ID of the organization owner",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ssoConfig": {
    ///      "$ref": "#/components/schemas/OrgSsoConfig"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Organization {
        /// Organization ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// List of organization members
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub members: Vec<OrganizationMember>,
        /// Organization owner's email address
        #[serde(
            rename = "ownerEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub owner_email: Option<String>,
        /// User ID of the organization owner
        #[serde(
            rename = "ownerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub owner_id: Option<String>,
        #[serde(
            rename = "ssoConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_config: Option<OrgSsoConfig>,
    }

    impl From<&Organization> for Organization {
        fn from(value: &Organization) -> Self {
            value.clone()
        }
    }

    /// OrganizationInvitation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "creation_time": {
    ///      "description": "Creation time of the invite",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1613067920454
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "email": {
    ///      "description": "Email address of invitee",
    ///      "examples": [
    ///        "joe@user.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Invitation ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "orgId": {
    ///      "description": "Organization ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "ownerEmail": {
    ///      "description": "Organization owner email address",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "user@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "description": "Invitation status",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "pending"
    ///      ],
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/InviteStatus"
    ///        }
    ///      ]
    ///    },
    ///    "update_time": {
    ///      "description": "Last updated time of the invitation",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1613067920454
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationInvitation {
        /// Creation time of the invite
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub creation_time: Option<i64>,
        /// Email address of invitee
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        /// Invitation ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// Organization ID
        #[serde(
            rename = "orgId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub org_id: Option<String>,
        /// Organization owner email address
        #[serde(
            rename = "ownerEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub owner_email: Option<String>,
        /// Invitation status
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<InviteStatus>,
        /// Last updated time of the invitation
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub update_time: Option<i64>,
    }

    impl From<&OrganizationInvitation> for OrganizationInvitation {
        fn from(value: &OrganizationInvitation) -> Self {
            value.clone()
        }
    }

    /// OrganizationMember
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "email": {
    ///      "description": "Organization member email address",
    ///      "readOnly": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "name": {
    ///      "description": "Organization member display name",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "Joe User"
    ///      ],
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "orgId": {
    ///      "description": "Organization ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "userId": {
    ///      "description": "User ID",
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrganizationMember {
        /// Organization member email address
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        /// Organization member display name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Organization ID
        #[serde(
            rename = "orgId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub org_id: Option<String>,
        /// User ID
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub user_id: Option<String>,
    }

    impl From<&OrganizationMember> for OrganizationMember {
        fn from(value: &OrganizationMember) -> Self {
            value.clone()
        }
    }

    /// Permissions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "a": {
    ///      "description": "Authorize permission",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "d": {
    ///      "description": "Delete permission",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "m": {
    ///      "description": "Modify network settings permission",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "r": {
    ///      "description": "Read network settings permission",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Permissions {
        /// Authorize permission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub a: Option<bool>,
        /// Delete permission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub d: Option<bool>,
        /// Modify network settings permission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub m: Option<bool>,
        /// Read network settings permission
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub r: Option<bool>,
    }

    impl From<&Permissions> for Permissions {
        fn from(value: &Permissions) -> Self {
            value.clone()
        }
    }

    /// PermissionsMap
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "examples": [
    ///    {
    ///      "00000000-0000-0000-0000-000000000000": {
    ///        "a": true,
    ///        "d": true,
    ///        "m": true,
    ///        "r": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "$ref": "#/components/schemas/Permissions"
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PermissionsMap(
        pub ::std::collections::HashMap<String, Permissions>,
    );
    impl ::std::ops::Deref for PermissionsMap {
        type Target = ::std::collections::HashMap<String, Permissions>;
        fn deref(&self) -> &::std::collections::HashMap<String, Permissions> {
            &self.0
        }
    }

    impl From<PermissionsMap> for ::std::collections::HashMap<String, Permissions> {
        fn from(value: PermissionsMap) -> Self {
            value.0
        }
    }

    impl From<&PermissionsMap> for PermissionsMap {
        fn from(value: &PermissionsMap) -> Self {
            value.clone()
        }
    }

    impl From<::std::collections::HashMap<String, Permissions>> for PermissionsMap {
        fn from(
            value: ::std::collections::HashMap<String, Permissions>,
        ) -> Self {
            Self(value)
        }
    }

    /// RandomToken
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "clock": {
    ///      "description": "Current time on server",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1613067920454
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "hex": {
    ///      "description": "hex encoded random bytes of the token",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "16924f3ff478526cffb1b89b1040b33c8dbd3c09e07f39691f615769121c0d76"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "token": {
    ///      "description": "Random 32 character token",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "wwrb66uUh18Fqc38rd8jMd5RFJzRsCn4"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RandomToken {
        /// Current time on server
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clock: Option<i64>,
        /// hex encoded random bytes of the token
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hex: Option<String>,
        /// Random 32 character token
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    impl From<&RandomToken> for RandomToken {
        fn from(value: &RandomToken) -> Self {
            value.clone()
        }
    }

    /// Route
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "target": {
    ///      "examples": [
    ///        "10.0.0.0/24"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "via": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Route {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub via: Option<String>,
    }

    impl From<&Route> for Route {
        fn from(value: &Route) -> Self {
            value.clone()
        }
    }

    /// SsoIssuer
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "authorization_endpoint": {
    ///      "description": "authorization endpoint",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "https://example.com/oidc/auth/endpoint"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "description": "OIDC Client ID",
    ///      "examples": [
    ///        "oidc-client-id"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "issuer": {
    ///      "description": "OIDC Issuer URL",
    ///      "examples": [
    ///        "https://example.com/oidc/auth"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "provider": {
    ///      "description": "OIDC Provider (one of: default, authelia, auth0,
    /// azure, keycloak, okta, onelogin)",
    ///      "examples": [
    ///        "keycloak"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SsoIssuer {
        /// Authorization endpoint
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authorization_endpoint: Option<String>,
        /// OIDC Client ID
        #[serde(
            rename = "clientId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_id: Option<String>,
        ///OIDC Issuer URL
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        /// OIDC Provider (one of: default, authelia, auth0, azure, keycloak,
        /// okta, onelogin)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
    }

    impl From<&SsoIssuer> for SsoIssuer {
        fn from(value: &SsoIssuer) -> Self {
            value.clone()
        }
    }

    /// Status
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "apiVersion": {
    ///      "examples": [
    ///        "4"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "clock": {
    ///      "description": "Current time on server",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1613067920454
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "id": {
    ///      "examples": [
    ///        "central_status"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "loginMethods": {
    ///      "type": "object",
    ///      "properties": {
    ///        "facebook": {
    ///          "type": "boolean"
    ///        },
    ///        "github": {
    ///          "type": "boolean"
    ///        },
    ///        "google": {
    ///          "type": "boolean"
    ///        },
    ///        "local": {
    ///          "type": "boolean"
    ///        },
    ///        "oidc": {
    ///          "type": "boolean"
    ///        },
    ///        "saml": {
    ///          "type": "boolean"
    ///        },
    ///        "twitter": {
    ///          "type": "boolean"
    ///        }
    ///      }
    ///    },
    ///    "readOnlyMode": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "examples": [
    ///        "CentralStatus"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "uptime": {
    ///      "description": "Uptime on server",
    ///      "readOnly": true,
    ///      "examples": [
    ///        1613067920454
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "user": {
    ///      "$ref": "#/components/schemas/User"
    ///    },
    ///    "version": {
    ///      "examples": [
    ///        "1.6.5"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Status {
        #[serde(
            rename = "apiVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub api_version: Option<String>,
        /// Current time on server
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clock: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "loginMethods",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub login_methods: Option<StatusLoginMethods>,
        #[serde(
            rename = "readOnlyMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub read_only_mode: Option<bool>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub type_: Option<String>,
        /// Uptime on server
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uptime: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<User>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }

    impl From<&Status> for Status {
        fn from(value: &Status) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let user = self.user.as_ref().unwrap();
            let name = user.display_name.show_or(::serde_json::Value::Null);
            let email = user.email.show_or(::serde_json::Value::Null);
            let id = user.id.show_or(::serde_json::Value::Null);

            let api_version =
                self.api_version.show_or(::serde_json::Value::Null);
            let uptime = self.uptime.show_or(::serde_json::Value::Null);

            write!(
                f,
                "User {name} (ID: {id}) <{email}>
  * Client API version: {api_version}
  * Client uptime: {uptime}"
            )
        }
    }

    /// StatusLoginMethods
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "facebook": {
    ///      "type": "boolean"
    ///    },
    ///    "github": {
    ///      "type": "boolean"
    ///    },
    ///    "google": {
    ///      "type": "boolean"
    ///    },
    ///    "local": {
    ///      "type": "boolean"
    ///    },
    ///    "oidc": {
    ///      "type": "boolean"
    ///    },
    ///    "saml": {
    ///      "type": "boolean"
    ///    },
    ///    "twitter": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StatusLoginMethods {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub facebook: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub github: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub google: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub local: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub oidc: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub saml: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub twitter: Option<bool>,
    }

    impl From<&StatusLoginMethods> for StatusLoginMethods {
        fn from(value: &StatusLoginMethods) -> Self {
            value.clone()
        }
    }

    /// User
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "auth": {
    ///      "readOnly": true,
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/AuthMethods"
    ///        }
    ///      ]
    ///    },
    ///    "displayName": {
    ///      "description": "Display Name",
    ///      "examples": [
    ///        "Joe User"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "description": "User email address",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "user@example.com"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "globalPermissions": {
    ///      "readOnly": true,
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Permissions"
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "description": "User ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "orgId": {
    ///      "description": "Organization ID",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "00000000-0000-0000-0000-000000000000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "smsNumber": {
    ///      "description": "SMS number",
    ///      "deprecated": true,
    ///      "examples": [
    ///        "+1-800-555-1212"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tokens": {
    ///      "description": "List of API token names.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        [
    ///          "my-token-id"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct User {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth: Option<AuthMethods>,
        /// Display Name
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        /// User email address
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(
            rename = "globalPermissions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub global_permissions: Option<Permissions>,
        /// User ID
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        /// Organization ID
        #[serde(
            rename = "orgId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub org_id: Option<String>,
        ///SMS number
        #[serde(
            rename = "smsNumber",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sms_number: Option<String>,
        /// List of API token names.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tokens: Vec<String>,
    }

    impl From<&User> for User {
        fn from(value: &User) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
/// Client for ZeroTier Central API
///
/// ZeroTier Central Network Management Portal API.
/// <p>All API requests must have an API token header specified in the
/// <code>Authorization: token xxxxx</code> format. You can generate your
/// API key by logging into <a href="https://my.zerotier.com">ZeroTier Central</a>
/// and creating a token on the Account page.</p> <p>eg.
/// <code>curl -X GET -H "Authorization: token xxxxx" https://api.zerotier.com/api/v1/network</code></p><p>
/// <h3>Rate Limiting</h3>
/// </p> <p>The ZeroTier Central API implements rate limiting. Paid users are
/// limited to 100 requests per second. Free users are limited to 20 requests
/// per second.</p> <p>You can get the OpenAPI spec here as well:
/// <code>https://docs.zerotier.com/api/central/ref-v1.json</code></p>
///
/// Version: v1
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "v1"
    }
}

#[allow(clippy::all)]
impl Client {
    /// Obtain the overall status of the account tied to the API token in use
    ///
    /// Sends a `GET` request to `/status`
    pub async fn get_status<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::Status>, Error<()>> {
        let url = format!("{}/status", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Returns a list of Networks you have access to
    ///
    /// Sends a `GET` request to `/network`
    pub async fn get_network_list<'a>(
        &'a self,
    ) -> Result<ResponseValue<Vec<types::Network>>, Error<()>> {
        let url = format!("{}/network", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Create a new network
    ///
    /// Sends a `POST` request to `/network`
    ///
    /// Arguments:
    /// - `body`: empty JSON object
    pub async fn new_network<'a>(
        &'a self,
        body: &'a ::serde_json::Map<String, ::serde_json::Value>,
    ) -> Result<ResponseValue<types::Network>, Error<()>> {
        let url = format!("{}/network", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get network by ID
    ///
    /// Returns a single network
    ///
    /// Sends a `GET` request to `/network/{networkID}`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network to return
    pub async fn get_network_by_id<'a>(
        &'a self,
        network_id: &'a str,
    ) -> Result<ResponseValue<types::Network>, Error<()>> {
        let url = format!(
            "{}/network/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Update network configuration
    ///
    /// Sends a `POST` request to `/network/{networkID}`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network to change
    /// - `body`: Network object JSON
    pub async fn update_network<'a>(
        &'a self,
        network_id: &'a str,
        body: &'a types::Network,
    ) -> Result<ResponseValue<types::Network>, Error<()>> {
        let url = format!(
            "{}/network/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Delete network
    ///
    /// Sends a `DELETE` request to `/network/{networkID}`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network
    pub async fn delete_network<'a>(
        &'a self,
        network_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/network/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Returns a list of Members on the network
    ///
    /// Sends a `GET` request to `/network/{networkID}/member`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network to return
    pub async fn get_network_member_list<'a>(
        &'a self,
        network_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::Member>>, Error<()>> {
        let url = format!(
            "{}/network/{}/member",
            self.baseurl,
            encode_path(&network_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Return an individual member on a network
    ///
    /// Sends a `GET` request to `/network/{networkID}/member/{memberID}`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network
    /// - `member_id`: ID of the member
    pub async fn get_network_member<'a>(
        &'a self,
        network_id: &'a str,
        member_id: &'a str,
    ) -> Result<ResponseValue<types::Member>, Error<()>> {
        let url = format!(
            "{}/network/{}/member/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
            encode_path(&member_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Modify a network member
    ///
    /// Sends a `POST` request to `/network/{networkID}/member/{memberID}`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network
    /// - `member_id`: ID of the member
    /// - `body`: Member object JSON
    pub async fn update_network_member<'a>(
        &'a self,
        network_id: &'a str,
        member_id: &'a str,
        body: &'a types::Member,
    ) -> Result<ResponseValue<types::Member>, Error<()>> {
        let url = format!(
            "{}/network/{}/member/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
            encode_path(&member_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Delete a network member
    ///
    /// Sends a `DELETE` request to `/network/{networkID}/member/{memberID}`
    ///
    /// Arguments:
    /// - `network_id`: ID of the network
    /// - `member_id`: ID of the member
    pub async fn delete_network_member<'a>(
        &'a self,
        network_id: &'a str,
        member_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/network/{}/member/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
            encode_path(&member_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get user record
    ///
    /// Sends a `GET` request to `/user/{userID}`
    ///
    /// Arguments:
    /// - `user_id`: User ID
    pub async fn get_user_by_id<'a>(
        &'a self,
        user_id: &'a str,
    ) -> Result<ResponseValue<types::User>, Error<()>> {
        let url = format!(
            "{}/user/{}",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Update user record (SMS number or Display Name only)
    ///
    /// Sends a `POST` request to `/user/{userID}`
    ///
    /// Arguments:
    /// - `user_id`: User ID
    /// - `body`: User object JSON
    pub async fn update_user_by_id<'a>(
        &'a self,
        user_id: &'a str,
        body: &'a types::User,
    ) -> Result<ResponseValue<types::User>, Error<()>> {
        let url = format!(
            "{}/user/{}",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Delete user
    ///
    /// Deletes the user and all associated networks. This is not reversible.
    /// Delete at your own risk.
    ///
    /// Sends a `DELETE` request to `/user/{userID}`
    ///
    /// Arguments:
    /// - `user_id`: User ID
    pub async fn delete_user_by_id<'a>(
        &'a self,
        user_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/user/{}",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Add an API token
    ///
    /// Sends a `POST` request to `/user/{userID}/token`
    ///
    /// Arguments:
    /// - `user_id`: User ID
    /// - `body`: APIToken JSON object
    pub async fn add_api_token<'a>(
        &'a self,
        user_id: &'a str,
        body: &'a types::ApiToken,
    ) -> Result<ResponseValue<types::ApiToken>, Error<()>> {
        let url = format!(
            "{}/user/{}/token",
            self.baseurl,
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Delete API Token
    ///
    /// Sends a `DELETE` request to `/user/{userID}/token/{tokenName}`
    ///
    /// Arguments:
    /// - `user_id`: User ID
    /// - `token_name`: Token Name
    pub async fn delete_api_token<'a>(
        &'a self,
        user_id: &'a str,
        token_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/user/{}/token/{}",
            self.baseurl,
            encode_path(&user_id.to_string()),
            encode_path(&token_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get the current user's organization
    ///
    /// Sends a `GET` request to `/org`
    pub async fn get_organization<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::Organization>, Error<()>> {
        let url = format!("{}/org", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get organization by ID
    ///
    /// Sends a `GET` request to `/org/{orgID}`
    ///
    /// Arguments:
    /// - `org_id`: Organization ID
    pub async fn get_organization_by_id<'a>(
        &'a self,
        org_id: &'a str,
    ) -> Result<ResponseValue<types::Organization>, Error<()>> {
        let url = format!(
            "{}/org/{}",
            self.baseurl,
            encode_path(&org_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get list of organization members
    ///
    /// Sends a `GET` request to `/org/{orgID}/user`
    ///
    /// Arguments:
    /// - `org_id`: Organization ID
    pub async fn get_organization_members<'a>(
        &'a self,
        org_id: &'a str,
    ) -> Result<ResponseValue<types::OrganizationMember>, Error<()>> {
        let url = format!(
            "{}/org/{}/user",
            self.baseurl,
            encode_path(&org_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get list of organization invitations
    ///
    /// Sends a `GET` request to `/org-invitation`
    pub async fn get_organization_invitation_list<'a>(
        &'a self,
    ) -> Result<ResponseValue<Vec<types::OrganizationInvitation>>, Error<()>>
    {
        let url = format!("{}/org-invitation", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Invite a user to your organization by email
    ///
    /// Sends a `POST` request to `/org-invitation`
    ///
    /// Arguments:
    /// - `body`: Organization Invitation JSON object
    pub async fn invite_user_by_email<'a>(
        &'a self,
        body: &'a types::OrganizationInvitation,
    ) -> Result<ResponseValue<types::OrganizationInvitation>, Error<()>> {
        let url = format!("{}/org-invitation", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get organization invitation
    ///
    /// Sends a `GET` request to `/org-invitation/{inviteID}`
    ///
    /// Arguments:
    /// - `invite_id`: Invitation ID
    pub async fn get_invitation_by_id<'a>(
        &'a self,
        invite_id: &'a str,
    ) -> Result<ResponseValue<types::OrganizationInvitation>, Error<()>> {
        let url = format!(
            "{}/org-invitation/{}",
            self.baseurl,
            encode_path(&invite_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Accept organization invitation
    ///
    /// Sends a `POST` request to `/org-invitation/{inviteID}`
    ///
    /// Arguments:
    /// - `invite_id`: Invitation ID
    pub async fn accept_invitation<'a>(
        &'a self,
        invite_id: &'a str,
    ) -> Result<ResponseValue<types::OrganizationInvitation>, Error<()>> {
        let url = format!(
            "{}/org-invitation/{}",
            self.baseurl,
            encode_path(&invite_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Decline organization invitation
    ///
    /// Sends a `DELETE` request to `/org-invitation/{inviteID}`
    ///
    /// Arguments:
    /// - `invite_id`: Invitation ID
    pub async fn decline_invitation<'a>(
        &'a self,
        invite_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/org-invitation/{}",
            self.baseurl,
            encode_path(&invite_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            401u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            403u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get a random 32 character token
    ///
    /// Get a random 32 character.  Used by the web UI to generate API keys
    ///
    /// Sends a `GET` request to `/randomToken`
    pub async fn get_random_token<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::RandomToken>, Error<()>> {
        let url = format!("{}/randomToken", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
