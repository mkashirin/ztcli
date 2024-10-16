#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {

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

    /// ControllerNetwork
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///   "type": "object",
    ///   "required": [
    ///     "capabilities",
    ///     "creationTime",
    ///     "dns",
    ///     "enableBroadcast",
    ///     "id",
    ///     "ipAssignmentPools",
    ///     "mtu",
    ///     "multicastLimit",
    ///     "name",
    ///     "nwid",
    ///     "objtype",
    ///     "private",
    ///     "revision",
    ///     "routes",
    ///     "rules",
    ///     "tags",
    ///     "v4AssignMode",
    ///     "v6AssignMode"
    ///   ],
    ///   "properties": {
    ///     "capabilities": {
    ///       "$ref": "#/components/schemas/NetworkCapsItem"
    ///     },
    ///     "creationTime": {
    ///       "$ref": "#/components/schemas/uSafeint"
    ///     },
    ///     "dns": {
    ///       "anyOf": [
    ///         {
    ///           "$ref": "#/components/schemas/NetworkDNS"
    ///         },
    ///         {
    ///           "$ref": "#/components/schemas/EmptyArrayItem"
    ///         }
    ///       ]
    ///     },
    ///     "enableBroadcast": {
    ///       "type": "boolean"
    ///     },
    ///     "id": {
    ///       "$ref": "#/components/schemas/ZTNetworkID"
    ///     },
    ///     "ipAssignmentPools": {
    ///       "type": "array",
    ///       "items": {
    ///         "type": "object",
    ///         "required": [
    ///           "ipRangeEnd",
    ///           "ipRangeStart"
    ///         ],
    ///         "properties": {
    ///           "ipRangeEnd": {
    ///             "anyOf": [
    ///               {
    ///                 "$ref": "#/components/schemas/IPv4"
    ///               },
    ///               {
    ///                 "$ref": "#/components/schemas/IPv6"
    ///               }
    ///             ]
    ///           },
    ///           "ipRangeStart": {
    ///             "anyOf": [
    ///               {
    ///                 "$ref": "#/components/schemas/IPv4"
    ///               },
    ///               {
    ///                 "$ref": "#/components/schemas/IPv6"
    ///               }
    ///             ]
    ///           }
    ///         }
    ///       }
    ///     },
    ///     "mtu": {
    ///       "$ref": "#/components/schemas/MTU"
    ///     },
    ///     "multicastLimit": {
    ///       "$ref": "#/components/schemas/uSafeint"
    ///     },
    ///     "name": {
    ///       "type": "string"
    ///     },
    ///     "nwid": {
    ///       "$ref": "#/components/schemas/ZTNetworkID"
    ///     },
    ///     "objtype": {
    ///       "type": "string",
    ///       "enum": [
    ///         "network"
    ///       ]
    ///     },
    ///     "private": {
    ///       "type": "boolean"
    ///     },
    ///     "revision": {
    ///       "$ref": "#/components/schemas/uSafeint"
    ///     },
    ///     "routes": {
    ///       "type": "array",
    ///       "items": {
    ///         "type": "object",
    ///         "required": [
    ///           "target"
    ///         ],
    ///         "properties": {
    ///           "target": {
    ///             "anyOf": [
    ///               {
    ///                 "$ref": "#/components/schemas/IPv4"
    ///               },
    ///               {
    ///                 "$ref": "#/components/schemas/IPv6"
    ///               }
    ///             ]
    ///           },
    ///           "via": {
    ///             "anyOf": [
    ///               {
    ///                 "$ref": "#/components/schemas/IPv4"
    ///               },
    ///               {
    ///                 "$ref": "#/components/schemas/IPv6"
    ///               }
    ///             ]
    ///           }
    ///         }
    ///       }
    ///     },
    ///     "rules": {
    ///       "type": "array",
    ///       "items": {
    ///         "$ref": "#/components/schemas/NetworkRule"
    ///       }
    ///     },
    ///     "tags": {
    ///       "$ref": "#/components/schemas/NetworkTagsItem"
    ///     },
    ///     "v4AssignMode": {
    ///       "type": "object",
    ///       "properties": {
    ///         "zt": {
    ///           "type": "boolean"
    ///         }
    ///       }
    ///     },
    ///     "v6AssignMode": {
    ///       "type": "object",
    ///       "properties": {
    ///         "6plane": {
    ///           "type": "boolean"
    ///         },
    ///         "rfc4193": {
    ///           "type": "boolean"
    ///         },
    ///         "zt": {
    ///           "type": "boolean"
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetwork {
        pub capabilities: NetworkCapsItem,
        #[serde(rename = "creationTime")]
        pub creation_time: USafeint,
        pub dns: ControllerNetworkDns,
        #[serde(rename = "enableBroadcast")]
        pub enable_broadcast: bool,
        pub id: ZtNetworkId,
        #[serde(rename = "ipAssignmentPools")]
        pub ip_assignment_pools: Vec<ControllerNetworkIpAssignmentPoolsItem>,
        pub mtu: Mtu,
        #[serde(rename = "multicastLimit")]
        pub multicast_limit: USafeint,
        pub name: String,
        pub nwid: ZtNetworkId,
        pub objtype: ControllerNetworkObjtype,
        pub private: bool,
        pub revision: USafeint,
        pub routes: Vec<ControllerNetworkRoutesItem>,
        pub rules: Vec<NetworkRule>,
        pub tags: NetworkTagsItem,
        #[serde(rename = "v4AssignMode")]
        pub v4_assign_mode: ControllerNetworkV4AssignMode,
        #[serde(rename = "v6AssignMode")]
        pub v6_assign_mode: ControllerNetworkV6AssignMode,
    }

    impl From<&ControllerNetwork> for ControllerNetwork {
        fn from(value: &ControllerNetwork) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkDns
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/NetworkDNS"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/EmptyArrayItem"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ControllerNetworkDns {
        NetworkDns(NetworkDns),
        EmptyArrayItem(EmptyArrayItem),
    }

    impl From<&ControllerNetworkDns> for ControllerNetworkDns {
        fn from(value: &ControllerNetworkDns) -> Self {
            value.clone()
        }
    }

    impl From<NetworkDns> for ControllerNetworkDns {
        fn from(value: NetworkDns) -> Self {
            Self::NetworkDns(value)
        }
    }

    impl From<EmptyArrayItem> for ControllerNetworkDns {
        fn from(value: EmptyArrayItem) -> Self {
            Self::EmptyArrayItem(value)
        }
    }

    /// ControllerNetworkIdList
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/ZTNetworkID"
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkIdList(pub Vec<ZtNetworkId>);
    impl ::std::ops::Deref for ControllerNetworkIdList {
        type Target = Vec<ZtNetworkId>;
        fn deref(&self) -> &Vec<ZtNetworkId> {
            &self.0
        }
    }

    impl From<ControllerNetworkIdList> for Vec<ZtNetworkId> {
        fn from(value: ControllerNetworkIdList) -> Self {
            value.0
        }
    }

    impl From<&ControllerNetworkIdList> for ControllerNetworkIdList {
        fn from(value: &ControllerNetworkIdList) -> Self {
            value.clone()
        }
    }

    impl From<Vec<ZtNetworkId>> for ControllerNetworkIdList {
        fn from(value: Vec<ZtNetworkId>) -> Self {
            Self(value)
        }
    }

    /// ControllerNetworkIpAssignmentPoolsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "ipRangeEnd",
    ///    "ipRangeStart"
    ///  ],
    ///  "properties": {
    ///    "ipRangeEnd": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    },
    ///    "ipRangeStart": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkIpAssignmentPoolsItem {
        #[serde(rename = "ipRangeEnd")]
        pub ip_range_end: ControllerNetworkIpAssignmentPoolsItemIpRangeEnd,
        #[serde(rename = "ipRangeStart")]
        pub ip_range_start: ControllerNetworkIpAssignmentPoolsItemIpRangeStart,
    }

    impl From<&ControllerNetworkIpAssignmentPoolsItem>
        for ControllerNetworkIpAssignmentPoolsItem
    {
        fn from(value: &ControllerNetworkIpAssignmentPoolsItem) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkIpAssignmentPoolsItemIpRangeEnd
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkIpAssignmentPoolsItemIpRangeEnd {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkIpAssignmentPoolsItemIpRangeEnd>
        for ControllerNetworkIpAssignmentPoolsItemIpRangeEnd
    {
        fn from(
            value: &ControllerNetworkIpAssignmentPoolsItemIpRangeEnd,
        ) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkIpAssignmentPoolsItemIpRangeStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkIpAssignmentPoolsItemIpRangeStart {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkIpAssignmentPoolsItemIpRangeStart>
        for ControllerNetworkIpAssignmentPoolsItemIpRangeStart
    {
        fn from(
            value: &ControllerNetworkIpAssignmentPoolsItemIpRangeStart,
        ) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMember
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "authenticationExpiryTime",
    ///    "capabilities",
    ///    "creationTime",
    ///    "id",
    ///    "lastAuthorizedCredential",
    ///    "lastAuthorizedCredentialType",
    ///    "lastAuthorizedTime",
    ///    "lastDeauthorizedTime",
    ///    "nwid",
    ///    "objtype",
    ///    "remoteTraceLevel",
    ///    "remoteTraceTarget",
    ///    "revision",
    ///    "tags",
    ///    "vMajor",
    ///    "vMinor",
    ///    "vProto",
    ///    "vRev"
    ///  ],
    ///  "properties": {
    ///    "activeBridge": {
    ///      "type": "boolean"
    ///    },
    ///    "address": {
    ///      "$ref": "#/components/schemas/ZTAddress"
    ///    },
    ///    "authenticationExpiryTime": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "authorized": {
    ///      "type": "boolean"
    ///    },
    ///    "capabilities": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/uSafeint"
    ///      }
    ///    },
    ///    "creationTime": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/ZTAddress"
    ///    },
    ///    "identity": {
    ///      "type": "string"
    ///    },
    ///    "ipAssignments": {
    ///      "type": "array",
    ///      "items": {
    ///        "anyOf": [
    ///          {
    ///            "$ref": "#/components/schemas/IPv4"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/IPv6"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "lastAuthorizedCredential": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "lastAuthorizedCredentialType": {
    ///      "type": "string"
    ///    },
    ///    "lastAuthorizedTime": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "lastDeauthorizedTime": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "noAutoAssignIps": {
    ///      "type": "boolean"
    ///    },
    ///    "nwid": {
    ///      "$ref": "#/components/schemas/ZTNetworkID"
    ///    },
    ///    "objtype": {
    ///      "type": "string",
    ///      "enum": [
    ///        "member"
    ///      ]
    ///    },
    ///    "remoteTraceLevel": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "remoteTraceTarget": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "revision": {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    "ssoExempt": {
    ///      "type": "boolean"
    ///    },
    ///    "tags": {},
    ///    "vMajor": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "vMinor": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "vProto": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "vRev": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMember {
        #[serde(
            rename = "activeBridge",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub active_bridge: Option<bool>,
        pub address: ZtAddress,
        #[serde(rename = "authenticationExpiryTime")]
        pub authentication_expiry_time: USafeint,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authorized: Option<bool>,
        pub capabilities: Vec<USafeint>,
        #[serde(rename = "creationTime")]
        pub creation_time: USafeint,
        pub id: ZtAddress,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub identity: Option<String>,
        #[serde(
            rename = "ipAssignments",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ip_assignments: Vec<ControllerNetworkMemberIpAssignmentsItem>,
        #[serde(rename = "lastAuthorizedCredential")]
        pub last_authorized_credential: Option<String>,
        #[serde(rename = "lastAuthorizedCredentialType")]
        pub last_authorized_credential_type: String,
        #[serde(rename = "lastAuthorizedTime")]
        pub last_authorized_time: USafeint,
        #[serde(rename = "lastDeauthorizedTime")]
        pub last_deauthorized_time: USafeint,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "noAutoAssignIps",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_auto_assign_ips: Option<bool>,
        pub nwid: ZtNetworkId,
        pub objtype: ControllerNetworkMemberObjtype,
        #[serde(rename = "remoteTraceLevel")]
        pub remote_trace_level: USafeint,
        #[serde(rename = "remoteTraceTarget")]
        pub remote_trace_target: Option<String>,
        pub revision: VersionDigit,
        #[serde(
            rename = "ssoExempt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_exempt: Option<bool>,
        pub tags: ::serde_json::Value,
        #[serde(rename = "vMajor")]
        pub v_major: ControllerNetworkMemberVMajor,
        #[serde(rename = "vMinor")]
        pub v_minor: ControllerNetworkMemberVMinor,
        #[serde(rename = "vProto")]
        pub v_proto: ControllerNetworkMemberVProto,
        #[serde(rename = "vRev")]
        pub v_rev: ControllerNetworkMemberVRev,
    }

    impl From<&ControllerNetworkMember> for ControllerNetworkMember {
        fn from(value: &ControllerNetworkMember) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMemberIpAssignmentsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberIpAssignmentsItem {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkMemberIpAssignmentsItem>
        for ControllerNetworkMemberIpAssignmentsItem
    {
        fn from(value: &ControllerNetworkMemberIpAssignmentsItem) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMemberList
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "integer",
    ///    "format": "int32"
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberList(
        pub ::std::collections::HashMap<String, i32>,
    );
    impl ::std::ops::Deref for ControllerNetworkMemberList {
        type Target = ::std::collections::HashMap<String, i32>;
        fn deref(&self) -> &::std::collections::HashMap<String, i32> {
            &self.0
        }
    }

    impl From<ControllerNetworkMemberList>
        for ::std::collections::HashMap<String, i32>
    {
        fn from(value: ControllerNetworkMemberList) -> Self {
            value.0
        }
    }

    impl From<&ControllerNetworkMemberList> for ControllerNetworkMemberList {
        fn from(value: &ControllerNetworkMemberList) -> Self {
            value.clone()
        }
    }

    impl From<::std::collections::HashMap<String, i32>>
        for ControllerNetworkMemberList
    {
        fn from(value: ::std::collections::HashMap<String, i32>) -> Self {
            Self(value)
        }
    }

    /// ControllerNetworkMemberListFull
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "meta"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ControllerNetworkMember"
    ///      }
    ///    },
    ///    "meta": {
    ///      "type": "object",
    ///      "required": [
    ///        "authorizedCount",
    ///        "totalCount"
    ///      ],
    ///      "properties": {
    ///        "authorizedCount": {
    ///          "$ref": "#/components/schemas/uSafeint"
    ///        },
    ///        "totalCount": {
    ///          "$ref": "#/components/schemas/uSafeint"
    ///        }
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberListFull {
        pub data: Vec<ControllerNetworkMember>,
        pub meta: ControllerNetworkMemberListFullMeta,
    }

    impl From<&ControllerNetworkMemberListFull>
        for ControllerNetworkMemberListFull
    {
        fn from(value: &ControllerNetworkMemberListFull) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMemberListFullMeta
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "authorizedCount",
    ///    "totalCount"
    ///  ],
    ///  "properties": {
    ///    "authorizedCount": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "totalCount": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberListFullMeta {
        #[serde(rename = "authorizedCount")]
        pub authorized_count: USafeint,
        #[serde(rename = "totalCount")]
        pub total_count: USafeint,
    }

    impl From<&ControllerNetworkMemberListFullMeta>
        for ControllerNetworkMemberListFullMeta
    {
        fn from(value: &ControllerNetworkMemberListFullMeta) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMemberObjtype
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "member"
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
    pub enum ControllerNetworkMemberObjtype {
        #[serde(rename = "member")]
        Member,
    }

    impl From<&ControllerNetworkMemberObjtype> for ControllerNetworkMemberObjtype {
        fn from(value: &ControllerNetworkMemberObjtype) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ControllerNetworkMemberObjtype {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Member => write!(f, "member"),
            }
        }
    }

    impl std::str::FromStr for ControllerNetworkMemberObjtype {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "member" => Ok(Self::Member),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ControllerNetworkMemberObjtype {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ControllerNetworkMemberObjtype {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ControllerNetworkMemberObjtype {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// ControllerNetworkMemberRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "activeBridge": {
    ///      "type": "boolean"
    ///    },
    ///    "authorized": {
    ///      "type": "boolean"
    ///    },
    ///    "ipAssignments": {
    ///      "type": "array",
    ///      "items": {
    ///        "anyOf": [
    ///          {
    ///            "$ref": "#/components/schemas/IPv4"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/IPv6"
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "noAutoAssignIps": {
    ///      "type": "boolean"
    ///    },
    ///    "ssoExempt": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberRequest {
        #[serde(
            rename = "activeBridge",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub active_bridge: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authorized: Option<bool>,
        #[serde(
            rename = "ipAssignments",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ip_assignments:
            Vec<ControllerNetworkMemberRequestIpAssignmentsItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "noAutoAssignIps",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_auto_assign_ips: Option<bool>,
        #[serde(
            rename = "ssoExempt",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_exempt: Option<bool>,
    }

    impl From<&ControllerNetworkMemberRequest> for ControllerNetworkMemberRequest {
        fn from(value: &ControllerNetworkMemberRequest) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMemberRequestIpAssignmentsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberRequestIpAssignmentsItem {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkMemberRequestIpAssignmentsItem>
        for ControllerNetworkMemberRequestIpAssignmentsItem
    {
        fn from(
            value: &ControllerNetworkMemberRequestIpAssignmentsItem,
        ) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkMemberVMajor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ControllerNetworkMemberVMajor {
        Variant0(VersionDigit),
        Variant1(ControllerNetworkMemberVMajorVariant1),
    }

    impl From<&ControllerNetworkMemberVMajor> for ControllerNetworkMemberVMajor {
        fn from(value: &ControllerNetworkMemberVMajor) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for ControllerNetworkMemberVMajor {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<ControllerNetworkMemberVMajorVariant1>
        for ControllerNetworkMemberVMajor
    {
        fn from(value: ControllerNetworkMemberVMajorVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// ControllerNetworkMemberVMajorVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberVMajorVariant1(f64);
    impl ::std::ops::Deref for ControllerNetworkMemberVMajorVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<ControllerNetworkMemberVMajorVariant1> for f64 {
        fn from(value: ControllerNetworkMemberVMajorVariant1) -> Self {
            value.0
        }
    }

    impl From<&ControllerNetworkMemberVMajorVariant1>
        for ControllerNetworkMemberVMajorVariant1
    {
        fn from(value: &ControllerNetworkMemberVMajorVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for ControllerNetworkMemberVMajorVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ControllerNetworkMemberVMajorVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// ControllerNetworkMemberVMinor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ControllerNetworkMemberVMinor {
        Variant0(VersionDigit),
        Variant1(ControllerNetworkMemberVMinorVariant1),
    }

    impl From<&ControllerNetworkMemberVMinor> for ControllerNetworkMemberVMinor {
        fn from(value: &ControllerNetworkMemberVMinor) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for ControllerNetworkMemberVMinor {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<ControllerNetworkMemberVMinorVariant1>
        for ControllerNetworkMemberVMinor
    {
        fn from(value: ControllerNetworkMemberVMinorVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// ControllerNetworkMemberVMinorVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberVMinorVariant1(f64);
    impl ::std::ops::Deref for ControllerNetworkMemberVMinorVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<ControllerNetworkMemberVMinorVariant1> for f64 {
        fn from(value: ControllerNetworkMemberVMinorVariant1) -> Self {
            value.0
        }
    }

    impl From<&ControllerNetworkMemberVMinorVariant1>
        for ControllerNetworkMemberVMinorVariant1
    {
        fn from(value: &ControllerNetworkMemberVMinorVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for ControllerNetworkMemberVMinorVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ControllerNetworkMemberVMinorVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// ControllerNetworkMemberVProto
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ControllerNetworkMemberVProto {
        Variant0(VersionDigit),
        Variant1(ControllerNetworkMemberVProtoVariant1),
    }

    impl From<&ControllerNetworkMemberVProto> for ControllerNetworkMemberVProto {
        fn from(value: &ControllerNetworkMemberVProto) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for ControllerNetworkMemberVProto {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<ControllerNetworkMemberVProtoVariant1>
        for ControllerNetworkMemberVProto
    {
        fn from(value: ControllerNetworkMemberVProtoVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// ControllerNetworkMemberVProtoVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberVProtoVariant1(f64);
    impl ::std::ops::Deref for ControllerNetworkMemberVProtoVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<ControllerNetworkMemberVProtoVariant1> for f64 {
        fn from(value: ControllerNetworkMemberVProtoVariant1) -> Self {
            value.0
        }
    }

    impl From<&ControllerNetworkMemberVProtoVariant1>
        for ControllerNetworkMemberVProtoVariant1
    {
        fn from(value: &ControllerNetworkMemberVProtoVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for ControllerNetworkMemberVProtoVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ControllerNetworkMemberVProtoVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// ControllerNetworkMemberVRev
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ControllerNetworkMemberVRev {
        Variant0(VersionDigit),
        Variant1(ControllerNetworkMemberVRevVariant1),
    }

    impl From<&ControllerNetworkMemberVRev> for ControllerNetworkMemberVRev {
        fn from(value: &ControllerNetworkMemberVRev) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for ControllerNetworkMemberVRev {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<ControllerNetworkMemberVRevVariant1> for ControllerNetworkMemberVRev {
        fn from(value: ControllerNetworkMemberVRevVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// ControllerNetworkMemberVRevVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkMemberVRevVariant1(f64);
    impl ::std::ops::Deref for ControllerNetworkMemberVRevVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<ControllerNetworkMemberVRevVariant1> for f64 {
        fn from(value: ControllerNetworkMemberVRevVariant1) -> Self {
            value.0
        }
    }

    impl From<&ControllerNetworkMemberVRevVariant1>
        for ControllerNetworkMemberVRevVariant1
    {
        fn from(value: &ControllerNetworkMemberVRevVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for ControllerNetworkMemberVRevVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ControllerNetworkMemberVRevVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// ControllerNetworkObjtype
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "network"
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
    pub enum ControllerNetworkObjtype {
        #[serde(rename = "network")]
        Network,
    }

    impl From<&ControllerNetworkObjtype> for ControllerNetworkObjtype {
        fn from(value: &ControllerNetworkObjtype) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ControllerNetworkObjtype {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Network => write!(f, "network"),
            }
        }
    }

    impl std::str::FromStr for ControllerNetworkObjtype {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "network" => Ok(Self::Network),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ControllerNetworkObjtype {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ControllerNetworkObjtype {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ControllerNetworkObjtype {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// ControllerNetworkRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "dns": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/NetworkDNS"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/EmptyArrayItem"
    ///        }
    ///      ]
    ///    },
    ///    "enableBroadcast": {
    ///      "type": "boolean"
    ///    },
    ///    "ipAssignmentPools": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "ipRangeEnd",
    ///          "ipRangeStart"
    ///        ],
    ///        "properties": {
    ///          "ipRangeEnd": {
    ///            "anyOf": [
    ///              {
    ///                "$ref": "#/components/schemas/IPv4"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/IPv6"
    ///              }
    ///            ]
    ///          },
    ///          "ipRangeStart": {
    ///            "anyOf": [
    ///              {
    ///                "$ref": "#/components/schemas/IPv4"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/IPv6"
    ///              }
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "mtu": {
    ///      "$ref": "#/components/schemas/MTU"
    ///    },
    ///    "multicastLimit": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "private": {
    ///      "type": "boolean"
    ///    },
    ///    "routes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "target"
    ///        ],
    ///        "properties": {
    ///          "target": {
    ///            "anyOf": [
    ///              {
    ///                "$ref": "#/components/schemas/IPv4"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/IPv6"
    ///              }
    ///            ]
    ///          },
    ///          "via": {
    ///            "anyOf": [
    ///              {
    ///                "$ref": "#/components/schemas/IPv4"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/IPv6"
    ///              }
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "v4AssignMode": {
    ///      "type": "object",
    ///      "properties": {
    ///        "zt": {
    ///          "type": "boolean"
    ///        }
    ///      }
    ///    },
    ///    "v6AssignMode": {
    ///      "type": "object",
    ///      "properties": {
    ///        "6plane": {
    ///          "type": "boolean"
    ///        },
    ///        "rfc4193": {
    ///          "type": "boolean"
    ///        },
    ///        "zt": {
    ///          "type": "boolean"
    ///        }
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequest {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dns: Option<ControllerNetworkRequestDns>,
        #[serde(
            rename = "enableBroadcast",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enable_broadcast: Option<bool>,
        #[serde(
            rename = "ipAssignmentPools",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub ip_assignment_pools:
            Vec<ControllerNetworkRequestIpAssignmentPoolsItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mtu: Option<Mtu>,
        #[serde(
            rename = "multicastLimit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub multicast_limit: Option<USafeint>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub routes: Vec<ControllerNetworkRequestRoutesItem>,
        #[serde(
            rename = "v4AssignMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v4_assign_mode: Option<ControllerNetworkRequestV4AssignMode>,
        #[serde(
            rename = "v6AssignMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub v6_assign_mode: Option<ControllerNetworkRequestV6AssignMode>,
    }

    impl From<&ControllerNetworkRequest> for ControllerNetworkRequest {
        fn from(value: &ControllerNetworkRequest) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestDns
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/NetworkDNS"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/EmptyArrayItem"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum ControllerNetworkRequestDns {
        NetworkDns(NetworkDns),
        EmptyArrayItem(EmptyArrayItem),
    }

    impl From<&ControllerNetworkRequestDns> for ControllerNetworkRequestDns {
        fn from(value: &ControllerNetworkRequestDns) -> Self {
            value.clone()
        }
    }

    impl From<NetworkDns> for ControllerNetworkRequestDns {
        fn from(value: NetworkDns) -> Self {
            Self::NetworkDns(value)
        }
    }

    impl From<EmptyArrayItem> for ControllerNetworkRequestDns {
        fn from(value: EmptyArrayItem) -> Self {
            Self::EmptyArrayItem(value)
        }
    }

    /// ControllerNetworkRequestIpAssignmentPoolsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "ipRangeEnd",
    ///    "ipRangeStart"
    ///  ],
    ///  "properties": {
    ///    "ipRangeEnd": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    },
    ///    "ipRangeStart": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestIpAssignmentPoolsItem {
        #[serde(rename = "ipRangeEnd")]
        pub ip_range_end:
            ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd,
        #[serde(rename = "ipRangeStart")]
        pub ip_range_start:
            ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart,
    }

    impl From<&ControllerNetworkRequestIpAssignmentPoolsItem>
        for ControllerNetworkRequestIpAssignmentPoolsItem
    {
        fn from(value: &ControllerNetworkRequestIpAssignmentPoolsItem) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd>
        for ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd
    {
        fn from(
            value: &ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd,
        ) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart>
        for ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart
    {
        fn from(
            value: &ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart,
        ) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestRoutesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "target": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    },
    ///    "via": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestRoutesItem {
        pub target: ControllerNetworkRequestRoutesItemTarget,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub via: Option<ControllerNetworkRequestRoutesItemVia>,
    }

    impl From<&ControllerNetworkRequestRoutesItem>
        for ControllerNetworkRequestRoutesItem
    {
        fn from(value: &ControllerNetworkRequestRoutesItem) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestRoutesItemTarget
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestRoutesItemTarget {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkRequestRoutesItemTarget>
        for ControllerNetworkRequestRoutesItemTarget
    {
        fn from(value: &ControllerNetworkRequestRoutesItemTarget) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestRoutesItemVia
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestRoutesItemVia {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkRequestRoutesItemVia>
        for ControllerNetworkRequestRoutesItemVia
    {
        fn from(value: &ControllerNetworkRequestRoutesItemVia) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestV4AssignMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "zt": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestV4AssignMode {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zt: Option<bool>,
    }

    impl From<&ControllerNetworkRequestV4AssignMode>
        for ControllerNetworkRequestV4AssignMode
    {
        fn from(value: &ControllerNetworkRequestV4AssignMode) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRequestV6AssignMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "6plane": {
    ///      "type": "boolean"
    ///    },
    ///    "rfc4193": {
    ///      "type": "boolean"
    ///    },
    ///    "zt": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRequestV6AssignMode {
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

    impl From<&ControllerNetworkRequestV6AssignMode>
        for ControllerNetworkRequestV6AssignMode
    {
        fn from(value: &ControllerNetworkRequestV6AssignMode) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRoutesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "target": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    },
    ///    "via": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRoutesItem {
        pub target: ControllerNetworkRoutesItemTarget,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub via: Option<ControllerNetworkRoutesItemVia>,
    }

    impl From<&ControllerNetworkRoutesItem> for ControllerNetworkRoutesItem {
        fn from(value: &ControllerNetworkRoutesItem) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRoutesItemTarget
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRoutesItemTarget {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkRoutesItemTarget>
        for ControllerNetworkRoutesItemTarget
    {
        fn from(value: &ControllerNetworkRoutesItemTarget) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkRoutesItemVia
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkRoutesItemVia {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&ControllerNetworkRoutesItemVia> for ControllerNetworkRoutesItemVia {
        fn from(value: &ControllerNetworkRoutesItemVia) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkV4AssignMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "zt": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkV4AssignMode {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zt: Option<bool>,
    }

    impl From<&ControllerNetworkV4AssignMode> for ControllerNetworkV4AssignMode {
        fn from(value: &ControllerNetworkV4AssignMode) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworkV6AssignMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "6plane": {
    ///      "type": "boolean"
    ///    },
    ///    "rfc4193": {
    ///      "type": "boolean"
    ///    },
    ///    "zt": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworkV6AssignMode {
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

    impl From<&ControllerNetworkV6AssignMode> for ControllerNetworkV6AssignMode {
        fn from(value: &ControllerNetworkV6AssignMode) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworks
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "data",
    ///    "meta"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ControllerNetwork"
    ///      }
    ///    },
    ///    "meta": {
    ///      "type": "object",
    ///      "required": [
    ///        "networkCount"
    ///      ],
    ///      "properties": {
    ///        "networkCount": {
    ///          "$ref": "#/components/schemas/uSafeint"
    ///        }
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworks {
        pub data: Vec<ControllerNetwork>,
        pub meta: ControllerNetworksMeta,
    }

    impl From<&ControllerNetworks> for ControllerNetworks {
        fn from(value: &ControllerNetworks) -> Self {
            value.clone()
        }
    }

    /// ControllerNetworksMeta
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "networkCount"
    ///  ],
    ///  "properties": {
    ///    "networkCount": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerNetworksMeta {
        #[serde(rename = "networkCount")]
        pub network_count: USafeint,
    }

    impl From<&ControllerNetworksMeta> for ControllerNetworksMeta {
        fn from(value: &ControllerNetworksMeta) -> Self {
            value.clone()
        }
    }

    /// ControllerStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "apiVersion",
    ///    "clock",
    ///    "controller",
    ///    "databaseReady"
    ///  ],
    ///  "properties": {
    ///    "apiVersion": {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    "clock": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "controller": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    },
    ///    "databaseReady": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ControllerStatus {
        #[serde(rename = "apiVersion")]
        pub api_version: VersionDigit,
        pub clock: USafeint,
        pub controller: bool,
        #[serde(rename = "databaseReady")]
        pub database_ready: bool,
    }

    impl From<&ControllerStatus> for ControllerStatus {
        fn from(value: &ControllerStatus) -> Self {
            value.clone()
        }
    }

    /// Domain
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "hostname"
    /// }
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct Domain(pub String);
    impl ::std::ops::Deref for Domain {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<Domain> for String {
        fn from(value: Domain) -> Self {
            value.0
        }
    }

    impl From<&Domain> for Domain {
        fn from(value: &Domain) -> Self {
            value.clone()
        }
    }

    impl From<String> for Domain {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Domain {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for Domain {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// EmptyArrayItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {},
    ///  "maxItems": 0,
    ///  "minItems": 0
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EmptyArrayItem(pub Vec<::serde_json::Value>);
    impl ::std::ops::Deref for EmptyArrayItem {
        type Target = Vec<::serde_json::Value>;
        fn deref(&self) -> &Vec<::serde_json::Value> {
            &self.0
        }
    }

    impl From<EmptyArrayItem> for Vec<::serde_json::Value> {
        fn from(value: EmptyArrayItem) -> Self {
            value.0
        }
    }

    impl From<&EmptyArrayItem> for EmptyArrayItem {
        fn from(value: &EmptyArrayItem) -> Self {
            value.clone()
        }
    }

    impl From<Vec<::serde_json::Value>> for EmptyArrayItem {
        fn from(value: Vec<::serde_json::Value>) -> Self {
            Self(value)
        }
    }

    /// IPv4
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "ipv4"
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IPv4(pub std::net::Ipv4Addr);
    impl ::std::ops::Deref for IPv4 {
        type Target = std::net::Ipv4Addr;
        fn deref(&self) -> &std::net::Ipv4Addr {
            &self.0
        }
    }

    impl From<IPv4> for std::net::Ipv4Addr {
        fn from(value: IPv4) -> Self {
            value.0
        }
    }

    impl From<&IPv4> for IPv4 {
        fn from(value: &IPv4) -> Self {
            value.clone()
        }
    }

    impl From<std::net::Ipv4Addr> for IPv4 {
        fn from(value: std::net::Ipv4Addr) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for IPv4 {
        type Err = <std::net::Ipv4Addr as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for IPv4 {
        type Error = <std::net::Ipv4Addr as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IPv4 {
        type Error = <std::net::Ipv4Addr as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IPv4 {
        type Error = <std::net::Ipv4Addr as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for IPv4 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// IPv6
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "ipv6"
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IPv6(pub std::net::Ipv6Addr);
    impl ::std::ops::Deref for IPv6 {
        type Target = std::net::Ipv6Addr;
        fn deref(&self) -> &std::net::Ipv6Addr {
            &self.0
        }
    }

    impl From<IPv6> for std::net::Ipv6Addr {
        fn from(value: IPv6) -> Self {
            value.0
        }
    }

    impl From<&IPv6> for IPv6 {
        fn from(value: &IPv6) -> Self {
            value.clone()
        }
    }

    impl From<std::net::Ipv6Addr> for IPv6 {
        fn from(value: std::net::Ipv6Addr) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for IPv6 {
        type Err = <std::net::Ipv6Addr as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for IPv6 {
        type Error = <std::net::Ipv6Addr as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for IPv6 {
        type Error = <std::net::Ipv6Addr as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for IPv6 {
        type Error = <std::net::Ipv6Addr as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for IPv6 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// IpSlashPort
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string"
    /// }
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct IpSlashPort(pub String);
    impl ::std::ops::Deref for IpSlashPort {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<IpSlashPort> for String {
        fn from(value: IpSlashPort) -> Self {
            value.0
        }
    }

    impl From<&IpSlashPort> for IpSlashPort {
        fn from(value: &IpSlashPort) -> Self {
            value.clone()
        }
    }

    impl From<String> for IpSlashPort {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for IpSlashPort {
        type Err = std::convert::Infallible;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for IpSlashPort {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// JoinedNetwork
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ],
    ///  "required": [
    ///    "allowDNS",
    ///    "allowDefault",
    ///    "allowGlobal",
    ///    "allowManaged",
    ///    "assignedAddresses",
    ///    "bridge",
    ///    "broadcastEnabled",
    ///    "dns",
    ///    "id",
    ///    "mac",
    ///    "mtu",
    ///    "multicastSubscriptions",
    ///    "name",
    ///    "netconfRevision",
    ///    "portDeviceName",
    ///    "portError",
    ///    "routes",
    ///    "status",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "allowDNS": {
    ///      "type": "boolean"
    ///    },
    ///    "allowDefault": {
    ///      "type": "boolean"
    ///    },
    ///    "allowGlobal": {
    ///      "type": "boolean"
    ///    },
    ///    "allowManaged": {
    ///      "type": "boolean"
    ///    },
    ///    "assignedAddresses": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authenticationExpiryTime": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "authenticationURL": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "bridge": {
    ///      "type": "boolean"
    ///    },
    ///    "broadcastEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "dns": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/NetworkDNS"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/EmptyArrayItem"
    ///        }
    ///      ]
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/ZTNetworkID"
    ///    },
    ///    "mac": {
    ///      "type": "string"
    ///    },
    ///    "mtu": {
    ///      "$ref": "#/components/schemas/MTU"
    ///    },
    ///    "multicastSubscriptions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "adi",
    ///          "mac"
    ///        ],
    ///        "properties": {
    ///          "adi": {
    ///            "$ref": "#/components/schemas/uSafeint"
    ///          },
    ///          "mac": {
    ///            "type": "string"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "netconfRevision": {
    ///      "type": "integer",
    ///      "format": "uint16"
    ///    },
    ///    "portDeviceName": {
    ///      "type": "string"
    ///    },
    ///    "portError": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "routes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "flags",
    ///          "metric",
    ///          "target"
    ///        ],
    ///        "properties": {
    ///          "flags": {
    ///            "type": "integer",
    ///            "format": "uint16"
    ///          },
    ///          "metric": {
    ///            "type": "integer",
    ///            "format": "uint16"
    ///          },
    ///          "target": {
    ///            "$ref": "#/components/schemas/IPSlashPort"
    ///          },
    ///          "via": {
    ///            "anyOf": [
    ///              {
    ///                "$ref": "#/components/schemas/IPv4"
    ///              },
    ///              {
    ///                "$ref": "#/components/schemas/IPv6"
    ///              }
    ///            ]
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "REQUESTING_CONFIGURATION",
    ///        "OK",
    ///        "ACCESS_DENIED",
    ///        "NOT_FOUND",
    ///        "PORT_ERROR",
    ///        "CLIENT_TOO_OLD",
    ///        "AUTHENTICATION_REQUIRED"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "PUBLIC",
    ///        "PRIVATE"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JoinedNetwork {
        #[serde(rename = "allowDefault")]
        pub allow_default: bool,
        #[serde(rename = "allowDNS")]
        pub allow_dns: bool,
        #[serde(rename = "allowGlobal")]
        pub allow_global: bool,
        #[serde(rename = "allowManaged")]
        pub allow_managed: bool,
        #[serde(rename = "assignedAddresses")]
        pub assigned_addresses: Vec<String>,
        #[serde(
            rename = "authenticationExpiryTime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authentication_expiry_time: Option<USafeint>,
        #[serde(
            rename = "authenticationURL",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authentication_url: Option<String>,
        pub bridge: bool,
        #[serde(rename = "broadcastEnabled")]
        pub broadcast_enabled: bool,
        pub dns: JoinedNetworkDns,
        pub id: ZtNetworkId,
        pub mac: String,
        pub mtu: Mtu,
        #[serde(rename = "multicastSubscriptions")]
        pub multicast_subscriptions:
            Vec<JoinedNetworkMulticastSubscriptionsItem>,
        pub name: String,
        #[serde(rename = "netconfRevision")]
        pub netconf_revision: u16,
        #[serde(rename = "portDeviceName")]
        pub port_device_name: String,
        #[serde(rename = "portError")]
        pub port_error: i32,
        pub routes: Vec<JoinedNetworkRoutesItem>,
        pub status: JoinedNetworkStatus,
        #[serde(rename = "type")]
        pub type_: JoinedNetworkType,
    }

    impl From<&JoinedNetwork> for JoinedNetwork {
        fn from(value: &JoinedNetwork) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for JoinedNetwork {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let name = self.name.clone();
            let id: String = self.id.clone().into();
            let status: String = self.status.to_string();
            write!(f, "Network {name} (ID: {id})\n  * Status: {status}")
        }
    }

    /// JoinedNetworkDns
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "allOf": [
    ///    {},
    ///    {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/NetworkDNS"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/EmptyArrayItem"
    ///        }
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum JoinedNetworkDns {
        NetworkDns(NetworkDns),
        EmptyArrayItem(EmptyArrayItem),
    }

    impl From<&JoinedNetworkDns> for JoinedNetworkDns {
        fn from(value: &JoinedNetworkDns) -> Self {
            value.clone()
        }
    }

    impl From<NetworkDns> for JoinedNetworkDns {
        fn from(value: NetworkDns) -> Self {
            Self::NetworkDns(value)
        }
    }

    impl From<EmptyArrayItem> for JoinedNetworkDns {
        fn from(value: EmptyArrayItem) -> Self {
            Self::EmptyArrayItem(value)
        }
    }

    /// JoinedNetworkMulticastSubscriptionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "adi",
    ///    "mac"
    ///  ],
    ///  "properties": {
    ///    "adi": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "mac": {
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JoinedNetworkMulticastSubscriptionsItem {
        pub adi: USafeint,
        pub mac: String,
    }

    impl From<&JoinedNetworkMulticastSubscriptionsItem>
        for JoinedNetworkMulticastSubscriptionsItem
    {
        fn from(value: &JoinedNetworkMulticastSubscriptionsItem) -> Self {
            value.clone()
        }
    }

    /// JoinedNetworkRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "allowDNS": {
    ///      "type": "boolean"
    ///    },
    ///    "allowDefault": {
    ///      "type": "boolean"
    ///    },
    ///    "allowGlobal": {
    ///      "type": "boolean"
    ///    },
    ///    "allowManaged": {
    ///      "type": "boolean"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JoinedNetworkRequest {
        #[serde(
            rename = "allowDefault",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_default: Option<bool>,
        #[serde(
            rename = "allowDNS",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_dns: Option<bool>,
        #[serde(
            rename = "allowGlobal",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_global: Option<bool>,
        #[serde(
            rename = "allowManaged",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_managed: Option<bool>,
    }

    impl From<&JoinedNetworkRequest> for JoinedNetworkRequest {
        fn from(value: &JoinedNetworkRequest) -> Self {
            value.clone()
        }
    }

    /// JoinedNetworkRoutesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "flags",
    ///    "metric",
    ///    "target"
    ///  ],
    ///  "properties": {
    ///    "flags": {
    ///      "type": "integer",
    ///      "format": "uint16"
    ///    },
    ///    "metric": {
    ///      "type": "integer",
    ///      "format": "uint16"
    ///    },
    ///    "target": {
    ///      "$ref": "#/components/schemas/IPSlashPort"
    ///    },
    ///    "via": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/IPv4"
    ///        },
    ///        {
    ///          "$ref": "#/components/schemas/IPv6"
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JoinedNetworkRoutesItem {
        pub flags: u16,
        pub metric: u16,
        pub target: IpSlashPort,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub via: Option<JoinedNetworkRoutesItemVia>,
    }

    impl From<&JoinedNetworkRoutesItem> for JoinedNetworkRoutesItem {
        fn from(value: &JoinedNetworkRoutesItem) -> Self {
            value.clone()
        }
    }

    /// JoinedNetworkRoutesItemVia
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JoinedNetworkRoutesItemVia {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&JoinedNetworkRoutesItemVia> for JoinedNetworkRoutesItemVia {
        fn from(value: &JoinedNetworkRoutesItemVia) -> Self {
            value.clone()
        }
    }

    /// JoinedNetworkStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "allOf": [
    ///    {},
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "REQUESTING_CONFIGURATION",
    ///        "OK",
    ///        "ACCESS_DENIED",
    ///        "NOT_FOUND",
    ///        "PORT_ERROR",
    ///        "CLIENT_TOO_OLD",
    ///        "AUTHENTICATION_REQUIRED"
    ///      ]
    ///    }
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
    pub enum JoinedNetworkStatus {
        #[serde(rename = "REQUESTING_CONFIGURATION")]
        RequestingConfiguration,
        #[serde(rename = "OK")]
        Ok,
        #[serde(rename = "ACCESS_DENIED")]
        AccessDenied,
        #[serde(rename = "NOT_FOUND")]
        NotFound,
        #[serde(rename = "PORT_ERROR")]
        PortError,
        #[serde(rename = "CLIENT_TOO_OLD")]
        ClientTooOld,
        #[serde(rename = "AUTHENTICATION_REQUIRED")]
        AuthenticationRequired,
    }

    impl From<&JoinedNetworkStatus> for JoinedNetworkStatus {
        fn from(value: &JoinedNetworkStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for JoinedNetworkStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::RequestingConfiguration => {
                    write!(f, "REQUESTING_CONFIGURATION")
                }
                Self::Ok => write!(f, "OK"),
                Self::AccessDenied => write!(f, "ACCESS_DENIED"),
                Self::NotFound => write!(f, "NOT_FOUND"),
                Self::PortError => write!(f, "PORT_ERROR"),
                Self::ClientTooOld => write!(f, "CLIENT_TOO_OLD"),
                Self::AuthenticationRequired => {
                    write!(f, "AUTHENTICATION_REQUIRED")
                }
            }
        }
    }

    impl std::str::FromStr for JoinedNetworkStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "REQUESTING_CONFIGURATION" => Ok(Self::RequestingConfiguration),
                "OK" => Ok(Self::Ok),
                "ACCESS_DENIED" => Ok(Self::AccessDenied),
                "NOT_FOUND" => Ok(Self::NotFound),
                "PORT_ERROR" => Ok(Self::PortError),
                "CLIENT_TOO_OLD" => Ok(Self::ClientTooOld),
                "AUTHENTICATION_REQUIRED" => Ok(Self::AuthenticationRequired),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for JoinedNetworkStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for JoinedNetworkStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for JoinedNetworkStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// JoinedNetworkType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "allOf": [
    ///    {},
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "PUBLIC",
    ///        "PRIVATE"
    ///      ]
    ///    }
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
    pub enum JoinedNetworkType {
        #[serde(rename = "PUBLIC")]
        Public,
        #[serde(rename = "PRIVATE")]
        Private,
    }

    impl From<&JoinedNetworkType> for JoinedNetworkType {
        fn from(value: &JoinedNetworkType) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for JoinedNetworkType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => write!(f, "PUBLIC"),
                Self::Private => write!(f, "PRIVATE"),
            }
        }
    }

    impl std::str::FromStr for JoinedNetworkType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "PUBLIC" => Ok(Self::Public),
                "PRIVATE" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for JoinedNetworkType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for JoinedNetworkType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for JoinedNetworkType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// LeaveResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "boolean",
    ///      "enum": [
    ///        true
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct LeaveResult {
        pub result: bool,
    }

    impl From<&LeaveResult> for LeaveResult {
        fn from(value: &LeaveResult) -> Self {
            value.clone()
        }
    }

    /// Mtu
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "integer",
    ///  "format": "uint32",
    ///  "minimum": 1280.0
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Mtu(pub i64);
    impl ::std::ops::Deref for Mtu {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<Mtu> for i64 {
        fn from(value: Mtu) -> Self {
            value.0
        }
    }

    impl From<&Mtu> for Mtu {
        fn from(value: &Mtu) -> Self {
            value.clone()
        }
    }

    impl From<i64> for Mtu {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Mtu {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Mtu {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Mtu {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Mtu {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for Mtu {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// NetworkCap
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "type": "integer"
    ///  },
    ///  "maxItems": 2,
    ///  "minItems": 2
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkCap(pub [i64; 2usize]);
    impl ::std::ops::Deref for NetworkCap {
        type Target = [i64; 2usize];
        fn deref(&self) -> &[i64; 2usize] {
            &self.0
        }
    }

    impl From<NetworkCap> for [i64; 2usize] {
        fn from(value: NetworkCap) -> Self {
            value.0
        }
    }

    impl From<&NetworkCap> for NetworkCap {
        fn from(value: &NetworkCap) -> Self {
            value.clone()
        }
    }

    impl From<[i64; 2usize]> for NetworkCap {
        fn from(value: [i64; 2usize]) -> Self {
            Self(value)
        }
    }

    /// NetworkCapsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/NetworkCap"
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkCapsItem(pub Vec<NetworkCap>);
    impl ::std::ops::Deref for NetworkCapsItem {
        type Target = Vec<NetworkCap>;
        fn deref(&self) -> &Vec<NetworkCap> {
            &self.0
        }
    }

    impl From<NetworkCapsItem> for Vec<NetworkCap> {
        fn from(value: NetworkCapsItem) -> Self {
            value.0
        }
    }

    impl From<&NetworkCapsItem> for NetworkCapsItem {
        fn from(value: &NetworkCapsItem) -> Self {
            value.clone()
        }
    }

    impl From<Vec<NetworkCap>> for NetworkCapsItem {
        fn from(value: Vec<NetworkCap>) -> Self {
            Self(value)
        }
    }

    /// NetworkDns
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "domain",
    ///    "servers"
    ///  ],
    ///  "properties": {
    ///    "domain": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/domain"
    ///        },
    ///        {
    ///          "type": "string",
    ///          "enum": [
    ///            ""
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "servers": {
    ///      "type": "array",
    ///      "items": {
    ///        "anyOf": [
    ///          {
    ///            "$ref": "#/components/schemas/IPv4"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/IPv6"
    ///          }
    ///        ]
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkDns {
        // Commented out this type because the response being received contains
        // a string meant to be `Domain` instead of an object, causing the error
        // to occur during deserialization.
        // pub domain: NetworkDnsDomain,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<Domain>,
        pub servers: Vec<NetworkDnsServersItem>,
    }

    impl From<&NetworkDns> for NetworkDns {
        fn from(value: &NetworkDns) -> Self {
            value.clone()
        }
    }

    /// NetworkDnsDomain
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/domain"
    ///    },
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        ""
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkDnsDomain {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<Domain>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<NetworkDnsDomainSubtype1>,
    }

    impl From<&NetworkDnsDomain> for NetworkDnsDomain {
        fn from(value: &NetworkDnsDomain) -> Self {
            value.clone()
        }
    }

    /// NetworkDnsDomainSubtype1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    ""
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
    pub enum NetworkDnsDomainSubtype1 {
        #[serde(rename = "")]
        X,
    }

    impl From<&NetworkDnsDomainSubtype1> for NetworkDnsDomainSubtype1 {
        fn from(value: &NetworkDnsDomainSubtype1) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NetworkDnsDomainSubtype1 {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X => write!(f, ""),
            }
        }
    }

    impl std::str::FromStr for NetworkDnsDomainSubtype1 {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "" => Ok(Self::X),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for NetworkDnsDomainSubtype1 {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for NetworkDnsDomainSubtype1 {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for NetworkDnsDomainSubtype1 {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// NetworkDnsServersItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/IPv4"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IPv6"
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkDnsServersItem {
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_0: Option<IPv4>,
        #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
        pub subtype_1: Option<IPv6>,
    }

    impl From<&NetworkDnsServersItem> for NetworkDnsServersItem {
        fn from(value: &NetworkDnsServersItem) -> Self {
            value.clone()
        }
    }

    /// NetworkRule
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "allOf": [
    ///    {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    }
    ///  ],
    ///  "required": [
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "not": {
    ///      "type": "boolean"
    ///    },
    ///    "or": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkRule {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub not: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub or: Option<bool>,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&NetworkRule> for NetworkRule {
        fn from(value: &NetworkRule) -> Self {
            value.clone()
        }
    }

    /// NetworkTag
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "type": "integer"
    ///  },
    ///  "maxItems": 2,
    ///  "minItems": 2
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkTag(pub [i64; 2usize]);
    impl ::std::ops::Deref for NetworkTag {
        type Target = [i64; 2usize];
        fn deref(&self) -> &[i64; 2usize] {
            &self.0
        }
    }

    impl From<NetworkTag> for [i64; 2usize] {
        fn from(value: NetworkTag) -> Self {
            value.0
        }
    }

    impl From<&NetworkTag> for NetworkTag {
        fn from(value: &NetworkTag) -> Self {
            value.clone()
        }
    }

    impl From<[i64; 2usize]> for NetworkTag {
        fn from(value: [i64; 2usize]) -> Self {
            Self(value)
        }
    }

    /// NetworkTagsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/NetworkTag"
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NetworkTagsItem(pub Vec<NetworkTag>);
    impl ::std::ops::Deref for NetworkTagsItem {
        type Target = Vec<NetworkTag>;
        fn deref(&self) -> &Vec<NetworkTag> {
            &self.0
        }
    }

    impl From<NetworkTagsItem> for Vec<NetworkTag> {
        fn from(value: NetworkTagsItem) -> Self {
            value.0
        }
    }

    impl From<&NetworkTagsItem> for NetworkTagsItem {
        fn from(value: &NetworkTagsItem) -> Self {
            value.clone()
        }
    }

    impl From<Vec<NetworkTag>> for NetworkTagsItem {
        fn from(value: Vec<NetworkTag>) -> Self {
            Self(value)
        }
    }

    /// NodeStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "clock",
    ///    "config",
    ///    "online",
    ///    "planetWorldId",
    ///    "planetWorldTimestamp",
    ///    "publicIdentity",
    ///    "tcpFallbackActive",
    ///    "version",
    ///    "versionBuild",
    ///    "versionMajor",
    ///    "versionMinor",
    ///    "versionRev"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "$ref": "#/components/schemas/ZTAddress"
    ///    },
    ///    "clock": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "config": {
    ///      "type": "object",
    ///      "required": [
    ///        "settings"
    ///      ],
    ///      "properties": {
    ///        "settings": {
    ///          "type": "object",
    ///          "required": [
    ///            "allowTcpFallbackRelay",
    ///            "forceTcpRelay",
    ///            "listeningOn",
    ///            "portMappingEnabled",
    ///            "primaryPort",
    ///            "secondaryPort",
    ///            "softwareUpdate",
    ///            "softwareUpdateChannel",
    ///            "surfaceAddresses",
    ///            "tertiaryPort"
    ///          ],
    ///          "properties": {
    ///            "allowManagementFrom": {
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/IPSlashPort"
    ///              }
    ///            },
    ///            "allowTcpFallbackRelay": {
    ///              "type": "boolean"
    ///            },
    ///            "forceTcpRelay": {
    ///              "type": "boolean"
    ///            },
    ///            "listeningOn": {
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/IPSlashPort"
    ///              }
    ///            },
    ///            "portMappingEnabled": {
    ///              "type": "boolean"
    ///            },
    ///            "primaryPort": {
    ///              "$ref": "#/components/schemas/Port"
    ///            },
    ///            "secondaryPort": {
    ///              "$ref": "#/components/schemas/Port"
    ///            },
    ///            "softwareUpdate": {
    ///              "type": "string"
    ///            },
    ///            "softwareUpdateChannel": {
    ///              "type": "string"
    ///            },
    ///            "surfaceAddresses": {
    ///              "type": "array",
    ///              "items": {
    ///                "$ref": "#/components/schemas/IPSlashPort"
    ///              }
    ///            },
    ///            "tertiaryPort": {
    ///              "$ref": "#/components/schemas/Port"
    ///            }
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "online": {
    ///      "type": "boolean"
    ///    },
    ///    "planetWorldId": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "planetWorldTimestamp": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "publicIdentity": {
    ///      "type": "string"
    ///    },
    ///    "tcpFallbackActive": {
    ///      "type": "boolean"
    ///    },
    ///    "version": {
    ///      "type": "string"
    ///    },
    ///    "versionBuild": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "versionMajor": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "versionMinor": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "versionRev": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatus {
        pub address: ZtAddress,
        pub clock: USafeint,
        pub config: NodeStatusConfig,
        pub online: bool,
        #[serde(rename = "planetWorldId")]
        pub planet_world_id: USafeint,
        #[serde(rename = "planetWorldTimestamp")]
        pub planet_world_timestamp: USafeint,
        #[serde(rename = "publicIdentity")]
        pub public_identity: String,
        #[serde(rename = "tcpFallbackActive")]
        pub tcp_fallback_active: bool,
        pub version: String,
        #[serde(rename = "versionBuild")]
        pub version_build: NodeStatusVersionBuild,
        #[serde(rename = "versionMajor")]
        pub version_major: NodeStatusVersionMajor,
        #[serde(rename = "versionMinor")]
        pub version_minor: NodeStatusVersionMinor,
        #[serde(rename = "versionRev")]
        pub version_rev: NodeStatusVersionRev,
    }

    impl From<&NodeStatus> for NodeStatus {
        fn from(value: &NodeStatus) -> Self {
            value.clone()
        }
    }

    /// NodeStatusConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "settings"
    ///  ],
    ///  "properties": {
    ///    "settings": {
    ///      "type": "object",
    ///      "required": [
    ///        "allowTcpFallbackRelay",
    ///        "forceTcpRelay",
    ///        "listeningOn",
    ///        "portMappingEnabled",
    ///        "primaryPort",
    ///        "secondaryPort",
    ///        "softwareUpdate",
    ///        "softwareUpdateChannel",
    ///        "surfaceAddresses",
    ///        "tertiaryPort"
    ///      ],
    ///      "properties": {
    ///        "allowManagementFrom": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/IPSlashPort"
    ///          }
    ///        },
    ///        "allowTcpFallbackRelay": {
    ///          "type": "boolean"
    ///        },
    ///        "forceTcpRelay": {
    ///          "type": "boolean"
    ///        },
    ///        "listeningOn": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/IPSlashPort"
    ///          }
    ///        },
    ///        "portMappingEnabled": {
    ///          "type": "boolean"
    ///        },
    ///        "primaryPort": {
    ///          "$ref": "#/components/schemas/Port"
    ///        },
    ///        "secondaryPort": {
    ///          "$ref": "#/components/schemas/Port"
    ///        },
    ///        "softwareUpdate": {
    ///          "type": "string"
    ///        },
    ///        "softwareUpdateChannel": {
    ///          "type": "string"
    ///        },
    ///        "surfaceAddresses": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/IPSlashPort"
    ///          }
    ///        },
    ///        "tertiaryPort": {
    ///          "$ref": "#/components/schemas/Port"
    ///        }
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatusConfig {
        pub settings: NodeStatusConfigSettings,
    }

    impl From<&NodeStatusConfig> for NodeStatusConfig {
        fn from(value: &NodeStatusConfig) -> Self {
            value.clone()
        }
    }

    /// NodeStatusConfigSettings
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "allowTcpFallbackRelay",
    ///    "forceTcpRelay",
    ///    "listeningOn",
    ///    "portMappingEnabled",
    ///    "primaryPort",
    ///    "secondaryPort",
    ///    "softwareUpdate",
    ///    "softwareUpdateChannel",
    ///    "surfaceAddresses",
    ///    "tertiaryPort"
    ///  ],
    ///  "properties": {
    ///    "allowManagementFrom": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IPSlashPort"
    ///      }
    ///    },
    ///    "allowTcpFallbackRelay": {
    ///      "type": "boolean"
    ///    },
    ///    "forceTcpRelay": {
    ///      "type": "boolean"
    ///    },
    ///    "listeningOn": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IPSlashPort"
    ///      }
    ///    },
    ///    "portMappingEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "primaryPort": {
    ///      "$ref": "#/components/schemas/Port"
    ///    },
    ///    "secondaryPort": {
    ///      "$ref": "#/components/schemas/Port"
    ///    },
    ///    "softwareUpdate": {
    ///      "type": "string"
    ///    },
    ///    "softwareUpdateChannel": {
    ///      "type": "string"
    ///    },
    ///    "surfaceAddresses": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IPSlashPort"
    ///      }
    ///    },
    ///    "tertiaryPort": {
    ///      "$ref": "#/components/schemas/Port"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatusConfigSettings {
        #[serde(
            rename = "allowManagementFrom",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub allow_management_from: Vec<IpSlashPort>,
        #[serde(rename = "allowTcpFallbackRelay")]
        pub allow_tcp_fallback_relay: bool,
        #[serde(rename = "forceTcpRelay")]
        pub force_tcp_relay: bool,
        #[serde(rename = "listeningOn")]
        pub listening_on: Vec<IpSlashPort>,
        #[serde(rename = "portMappingEnabled")]
        pub port_mapping_enabled: bool,
        #[serde(rename = "primaryPort")]
        pub primary_port: Port,
        #[serde(rename = "secondaryPort")]
        pub secondary_port: Port,
        #[serde(rename = "softwareUpdate")]
        pub software_update: String,
        #[serde(rename = "softwareUpdateChannel")]
        pub software_update_channel: String,
        #[serde(rename = "surfaceAddresses")]
        pub surface_addresses: Vec<IpSlashPort>,
        #[serde(rename = "tertiaryPort")]
        pub tertiary_port: Port,
    }

    impl From<&NodeStatusConfigSettings> for NodeStatusConfigSettings {
        fn from(value: &NodeStatusConfigSettings) -> Self {
            value.clone()
        }
    }

    /// NodeStatusVersionBuild
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum NodeStatusVersionBuild {
        Variant0(VersionDigit),
        Variant1(NodeStatusVersionBuildVariant1),
    }

    impl From<&NodeStatusVersionBuild> for NodeStatusVersionBuild {
        fn from(value: &NodeStatusVersionBuild) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for NodeStatusVersionBuild {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<NodeStatusVersionBuildVariant1> for NodeStatusVersionBuild {
        fn from(value: NodeStatusVersionBuildVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    ///  NodeStatusVersionBuildVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatusVersionBuildVariant1(f64);
    impl ::std::ops::Deref for NodeStatusVersionBuildVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<NodeStatusVersionBuildVariant1> for f64 {
        fn from(value: NodeStatusVersionBuildVariant1) -> Self {
            value.0
        }
    }

    impl From<&NodeStatusVersionBuildVariant1> for NodeStatusVersionBuildVariant1 {
        fn from(value: &NodeStatusVersionBuildVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for NodeStatusVersionBuildVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for NodeStatusVersionBuildVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// NodeStatusVersionMajor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum NodeStatusVersionMajor {
        Variant0(VersionDigit),
        Variant1(NodeStatusVersionMajorVariant1),
    }

    impl From<&NodeStatusVersionMajor> for NodeStatusVersionMajor {
        fn from(value: &NodeStatusVersionMajor) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for NodeStatusVersionMajor {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<NodeStatusVersionMajorVariant1> for NodeStatusVersionMajor {
        fn from(value: NodeStatusVersionMajorVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// NodeStatusVersionMajorVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatusVersionMajorVariant1(f64);
    impl ::std::ops::Deref for NodeStatusVersionMajorVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<NodeStatusVersionMajorVariant1> for f64 {
        fn from(value: NodeStatusVersionMajorVariant1) -> Self {
            value.0
        }
    }

    impl From<&NodeStatusVersionMajorVariant1> for NodeStatusVersionMajorVariant1 {
        fn from(value: &NodeStatusVersionMajorVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for NodeStatusVersionMajorVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for NodeStatusVersionMajorVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// NodeStatusVersionMinor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum NodeStatusVersionMinor {
        Variant0(VersionDigit),
        Variant1(NodeStatusVersionMinorVariant1),
    }

    impl From<&NodeStatusVersionMinor> for NodeStatusVersionMinor {
        fn from(value: &NodeStatusVersionMinor) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for NodeStatusVersionMinor {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<NodeStatusVersionMinorVariant1> for NodeStatusVersionMinor {
        fn from(value: NodeStatusVersionMinorVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// NodeStatusVersionMinorVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatusVersionMinorVariant1(f64);
    impl ::std::ops::Deref for NodeStatusVersionMinorVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<NodeStatusVersionMinorVariant1> for f64 {
        fn from(value: NodeStatusVersionMinorVariant1) -> Self {
            value.0
        }
    }

    impl From<&NodeStatusVersionMinorVariant1> for NodeStatusVersionMinorVariant1 {
        fn from(value: &NodeStatusVersionMinorVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for NodeStatusVersionMinorVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for NodeStatusVersionMinorVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// NodeStatusVersionRev
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum NodeStatusVersionRev {
        Variant0(VersionDigit),
        Variant1(NodeStatusVersionRevVariant1),
    }

    impl From<&NodeStatusVersionRev> for NodeStatusVersionRev {
        fn from(value: &NodeStatusVersionRev) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for NodeStatusVersionRev {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<NodeStatusVersionRevVariant1> for NodeStatusVersionRev {
        fn from(value: NodeStatusVersionRevVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// NodeStatusVersionRevVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct NodeStatusVersionRevVariant1(f64);
    impl ::std::ops::Deref for NodeStatusVersionRevVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<NodeStatusVersionRevVariant1> for f64 {
        fn from(value: NodeStatusVersionRevVariant1) -> Self {
            value.0
        }
    }

    impl From<&NodeStatusVersionRevVariant1> for NodeStatusVersionRevVariant1 {
        fn from(value: &NodeStatusVersionRevVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for NodeStatusVersionRevVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for NodeStatusVersionRevVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// Peer
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "address",
    ///    "isBonded",
    ///    "latency",
    ///    "paths",
    ///    "role",
    ///    "tunneled",
    ///    "version",
    ///    "versionMajor",
    ///    "versionMinor",
    ///    "versionRev"
    ///  ],
    ///  "properties": {
    ///    "address": {
    ///      "$ref": "#/components/schemas/ZTAddress"
    ///    },
    ///    "isBonded": {
    ///      "type": "boolean"
    ///    },
    ///    "latency": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/uSafeint"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "paths": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "active",
    ///          "address",
    ///          "expired",
    ///          "lastReceive",
    ///          "lastSend",
    ///          "localSocket",
    ///          "preferred",
    ///          "trustedPathId"
    ///        ],
    ///        "properties": {
    ///          "active": {
    ///            "type": "boolean"
    ///          },
    ///          "address": {
    ///            "$ref": "#/components/schemas/IPSlashPort"
    ///          },
    ///          "expired": {
    ///            "type": "boolean"
    ///          },
    ///          "lastReceive": {
    ///            "$ref": "#/components/schemas/uSafeint"
    ///          },
    ///          "lastSend": {
    ///            "$ref": "#/components/schemas/uSafeint"
    ///          },
    ///          "localSocket": {
    ///            "$ref": "#/components/schemas/uSafeint"
    ///          },
    ///          "preferred": {
    ///            "type": "boolean"
    ///          },
    ///          "trustedPathId": {
    ///            "$ref": "#/components/schemas/uSafeint"
    ///          }
    ///        }
    ///      }
    ///    },
    ///    "role": {
    ///      "type": "string",
    ///      "enum": [
    ///        "LEAF",
    ///        "PLANET",
    ///        "MOON"
    ///      ]
    ///    },
    ///    "tunneled": {
    ///      "type": "boolean"
    ///    },
    ///    "version": {
    ///      "type": "string"
    ///    },
    ///    "versionMajor": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "versionMinor": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "versionRev": {
    ///      "anyOf": [
    ///        {
    ///          "$ref": "#/components/schemas/VersionDigit"
    ///        },
    ///        {
    ///          "type": "number",
    ///          "enum": [
    ///            -1.0
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Peer {
        pub address: ZtAddress,
        #[serde(rename = "isBonded")]
        pub is_bonded: bool,
        pub latency: PeerLatency,
        pub paths: Vec<PeerPathsItem>,
        pub role: PeerRole,
        pub tunneled: bool,
        pub version: String,
        #[serde(rename = "versionMajor")]
        pub version_major: PeerVersionMajor,
        #[serde(rename = "versionMinor")]
        pub version_minor: PeerVersionMinor,
        #[serde(rename = "versionRev")]
        pub version_rev: PeerVersionRev,
    }

    impl From<&Peer> for Peer {
        fn from(value: &Peer) -> Self {
            value.clone()
        }
    }

    /// PeerLatency
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PeerLatency {
        Variant0(USafeint),
        Variant1(PeerLatencyVariant1),
    }

    impl From<&PeerLatency> for PeerLatency {
        fn from(value: &PeerLatency) -> Self {
            value.clone()
        }
    }

    impl From<USafeint> for PeerLatency {
        fn from(value: USafeint) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<PeerLatencyVariant1> for PeerLatency {
        fn from(value: PeerLatencyVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// PeerLatencyVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct PeerLatencyVariant1(f64);
    impl ::std::ops::Deref for PeerLatencyVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<PeerLatencyVariant1> for f64 {
        fn from(value: PeerLatencyVariant1) -> Self {
            value.0
        }
    }

    impl From<&PeerLatencyVariant1> for PeerLatencyVariant1 {
        fn from(value: &PeerLatencyVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for PeerLatencyVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PeerLatencyVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// PeerPathsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "active",
    ///    "address",
    ///    "expired",
    ///    "lastReceive",
    ///    "lastSend",
    ///    "localSocket",
    ///    "preferred",
    ///    "trustedPathId"
    ///  ],
    ///  "properties": {
    ///    "active": {
    ///      "type": "boolean"
    ///    },
    ///    "address": {
    ///      "$ref": "#/components/schemas/IPSlashPort"
    ///    },
    ///    "expired": {
    ///      "type": "boolean"
    ///    },
    ///    "lastReceive": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "lastSend": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "localSocket": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    },
    ///    "preferred": {
    ///      "type": "boolean"
    ///    },
    ///    "trustedPathId": {
    ///      "$ref": "#/components/schemas/uSafeint"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PeerPathsItem {
        pub active: bool,
        pub address: IpSlashPort,
        pub expired: bool,
        #[serde(rename = "lastReceive")]
        pub last_receive: USafeint,
        #[serde(rename = "lastSend")]
        pub last_send: USafeint,
        #[serde(rename = "localSocket")]
        pub local_socket: USafeint,
        pub preferred: bool,
        #[serde(rename = "trustedPathId")]
        pub trusted_path_id: USafeint,
    }

    impl From<&PeerPathsItem> for PeerPathsItem {
        fn from(value: &PeerPathsItem) -> Self {
            value.clone()
        }
    }

    /// PeerRole
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "LEAF",
    ///    "PLANET",
    ///    "MOON"
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
    pub enum PeerRole {
        #[serde(rename = "LEAF")]
        Leaf,
        #[serde(rename = "PLANET")]
        Planet,
        #[serde(rename = "MOON")]
        Moon,
    }

    impl From<&PeerRole> for PeerRole {
        fn from(value: &PeerRole) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for PeerRole {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Leaf => write!(f, "LEAF"),
                Self::Planet => write!(f, "PLANET"),
                Self::Moon => write!(f, "MOON"),
            }
        }
    }

    impl std::str::FromStr for PeerRole {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "LEAF" => Ok(Self::Leaf),
                "PLANET" => Ok(Self::Planet),
                "MOON" => Ok(Self::Moon),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PeerRole {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PeerRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PeerRole {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// PeerVersionMajor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PeerVersionMajor {
        Variant0(VersionDigit),
        Variant1(PeerVersionMajorVariant1),
    }

    impl From<&PeerVersionMajor> for PeerVersionMajor {
        fn from(value: &PeerVersionMajor) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for PeerVersionMajor {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<PeerVersionMajorVariant1> for PeerVersionMajor {
        fn from(value: PeerVersionMajorVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// PeerVersionMajorVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct PeerVersionMajorVariant1(f64);
    impl ::std::ops::Deref for PeerVersionMajorVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<PeerVersionMajorVariant1> for f64 {
        fn from(value: PeerVersionMajorVariant1) -> Self {
            value.0
        }
    }

    impl From<&PeerVersionMajorVariant1> for PeerVersionMajorVariant1 {
        fn from(value: &PeerVersionMajorVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for PeerVersionMajorVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PeerVersionMajorVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// PeerVersionMinor
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PeerVersionMinor {
        Variant0(VersionDigit),
        Variant1(PeerVersionMinorVariant1),
    }

    impl From<&PeerVersionMinor> for PeerVersionMinor {
        fn from(value: &PeerVersionMinor) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for PeerVersionMinor {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<PeerVersionMinorVariant1> for PeerVersionMinor {
        fn from(value: PeerVersionMinorVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// PeerVersionMinorVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct PeerVersionMinorVariant1(f64);
    impl ::std::ops::Deref for PeerVersionMinorVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<PeerVersionMinorVariant1> for f64 {
        fn from(value: PeerVersionMinorVariant1) -> Self {
            value.0
        }
    }

    impl From<&PeerVersionMinorVariant1> for PeerVersionMinorVariant1 {
        fn from(value: &PeerVersionMinorVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for PeerVersionMinorVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PeerVersionMinorVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// PeerVersionRev
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/VersionDigit"
    ///    },
    ///    {
    ///      "type": "number",
    ///      "enum": [
    ///        -1.0
    ///      ]
    ///    }
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum PeerVersionRev {
        Variant0(VersionDigit),
        Variant1(PeerVersionRevVariant1),
    }

    impl From<&PeerVersionRev> for PeerVersionRev {
        fn from(value: &PeerVersionRev) -> Self {
            value.clone()
        }
    }

    impl From<VersionDigit> for PeerVersionRev {
        fn from(value: VersionDigit) -> Self {
            Self::Variant0(value)
        }
    }

    impl From<PeerVersionRevVariant1> for PeerVersionRev {
        fn from(value: PeerVersionRevVariant1) -> Self {
            Self::Variant1(value)
        }
    }

    /// PeerVersionRevVariant1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "number",
    ///  "enum": [
    ///    -1.0
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    pub struct PeerVersionRevVariant1(f64);
    impl ::std::ops::Deref for PeerVersionRevVariant1 {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl From<PeerVersionRevVariant1> for f64 {
        fn from(value: PeerVersionRevVariant1) -> Self {
            value.0
        }
    }

    impl From<&PeerVersionRevVariant1> for PeerVersionRevVariant1 {
        fn from(value: &PeerVersionRevVariant1) -> Self {
            value.clone()
        }
    }

    impl std::convert::TryFrom<f64> for PeerVersionRevVariant1 {
        type Error = self::error::ConversionError;
        fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
            if ![-1.0_f64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for PeerVersionRevVariant1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<f64>::deserialize(deserializer)?).map_err(|e| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
        }
    }

    /// Port
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "integer",
    ///  "format": "uint16",
    ///  "minimum": 0.0
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Port(pub u16);
    impl ::std::ops::Deref for Port {
        type Target = u16;
        fn deref(&self) -> &u16 {
            &self.0
        }
    }

    impl From<Port> for u16 {
        fn from(value: Port) -> Self {
            value.0
        }
    }

    impl From<&Port> for Port {
        fn from(value: &Port) -> Self {
            value.clone()
        }
    }

    impl From<u16> for Port {
        fn from(value: u16) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for Port {
        type Err = <u16 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for Port {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Port {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Port {
        type Error = <u16 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for Port {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// USafeint
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "integer",
    ///  "format": "int64",
    ///  "minimum": 0.0
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct USafeint(pub i64);
    impl ::std::ops::Deref for USafeint {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl From<USafeint> for i64 {
        fn from(value: USafeint) -> Self {
            value.0
        }
    }

    impl From<&USafeint> for USafeint {
        fn from(value: &USafeint) -> Self {
            value.clone()
        }
    }

    impl From<i64> for USafeint {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for USafeint {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for USafeint {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for USafeint {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for USafeint {
        type Error = <i64 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for USafeint {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// VersionDigit
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "integer",
    ///  "format": "uint8",
    ///  "minimum": 0.0
    /// }
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct VersionDigit(pub u8);
    impl ::std::ops::Deref for VersionDigit {
        type Target = u8;
        fn deref(&self) -> &u8 {
            &self.0
        }
    }

    impl From<VersionDigit> for u8 {
        fn from(value: VersionDigit) -> Self {
            value.0
        }
    }

    impl From<&VersionDigit> for VersionDigit {
        fn from(value: &VersionDigit) -> Self {
            value.clone()
        }
    }

    impl From<u8> for VersionDigit {
        fn from(value: u8) -> Self {
            Self(value)
        }
    }

    impl std::str::FromStr for VersionDigit {
        type Err = <u8 as std::str::FromStr>::Err;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl std::convert::TryFrom<&str> for VersionDigit {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for VersionDigit {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for VersionDigit {
        type Error = <u8 as std::str::FromStr>::Err;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for VersionDigit {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// ZtAddress
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "pattern": "[a-f0-9]{10}"
    /// }
    /// ```
    /// </details>
    #[derive(
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct ZtAddress(String);
    impl ::std::ops::Deref for ZtAddress {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<ZtAddress> for String {
        fn from(value: ZtAddress) -> Self {
            value.0
        }
    }

    impl From<&ZtAddress> for ZtAddress {
        fn from(value: &ZtAddress) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for ZtAddress {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("[a-f0-9]{10}")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"[a-f0-9]{10}\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ZtAddress {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for ZtAddress {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for ZtAddress {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ZtAddress {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?.parse().map_err(
                |e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                },
            )
        }
    }

    /// ZtNetworkId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "pattern": "[a-f0-9]{16}"
    /// }
    /// ```
    /// </details>
    #[derive(
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub struct ZtNetworkId(String);
    impl ::std::ops::Deref for ZtNetworkId {
        type Target = String;
        fn deref(&self) -> &String {
            &self.0
        }
    }

    impl From<ZtNetworkId> for String {
        fn from(value: ZtNetworkId) -> Self {
            value.0
        }
    }

    impl From<&ZtNetworkId> for ZtNetworkId {
        fn from(value: &ZtNetworkId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for ZtNetworkId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            if regress::Regex::new("[a-f0-9]{16}")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"[a-f0-9]{16}\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ZtNetworkId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for ZtNetworkId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for ZtNetworkId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: String,
        ) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ZtNetworkId {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            String::deserialize(deserializer)?.parse().map_err(
                |e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                },
            )
        }
    }
}

#[derive(Clone, Debug)]
/// Client for ZeroTierOne Service
///
/// <p>This API controls the ZeroTier service that runs in the background on
/// your computer. This is how zerotier-cli, and the macOS and Windows apps
/// control the service. </p> <p>API requests must be authenticated via an
/// authentication token. ZeroTier One saves this token in the authtoken.secret
/// file in its working directory. This token may be supplied via the X-ZT1-Auth
/// HTTP request header. </p> <p>For example: <code>curl -H "X-ZT1-Auth: $TOKEN" http://localhost:9993/status</code> </p>
/// <p>The token can be found in: <ul> <li>Mac :: ~/Library/Application
/// Support/ZeroTier/authtoken.secret</li> <li>Windows ::
/// \ProgramData\ZeroTier\One</li> <li>Linux :: /var/lib/zerotier-one</li> </ul>
/// </p> <p>Learn more about the spec at <a href="https://github.com/zerotier/zerotier-one-api-spec">github</a></p>
///
/// Version: 1.1.1
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
        "1.1.1"
    }
}

#[allow(clippy::all)]
impl Client {
    /// Controller Status
    /// 
    /// Controller healthcheck
    /// 
    /// Sends a `GET` request to `/controller`
    pub async fn controller_read_controller_status<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::ControllerStatus>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!("{}/controller", self.baseurl,);
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// List Network IDs
    /// 
    /// List IDs of all networks hosted by this controller
    /// 
    /// Sends a `GET` request to `/controller/network`
    pub async fn network_read_networks<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::ControllerNetworkIdList>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!("{}/controller/network", self.baseurl,);
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Generate Random Network ID
    /// 
    /// Create a new network with a random ID.
    /// 
    /// Sends a `POST` request to `/controller/network`
    ///
    /// Arguments:
    /// - `body`: Node ID of the controller
    pub async fn random_network_random_network<'a>(
        &'a self,
        body: &'a types::ControllerNetworkRequest,
    ) -> Result<
        ResponseValue<types::ControllerNetwork>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!("{}/controller/network", self.baseurl,);
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get Network by ID
    /// 
    /// Get details for a network by its ID.
    /// 
    /// Sends a `GET` request to `/controller/network/{network_id}`
    pub async fn network_read_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::ControllerNetwork>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}",
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Create or Update Network
    /// 
    /// Sends a `POST` request to `/controller/network/{network_id}`
    pub async fn network_post_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
        body: &'a types::ControllerNetworkRequest,
    ) -> Result<
        ResponseValue<types::ControllerNetwork>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}",
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Delete Network
    /// 
    /// Sends a `DELETE` request to `/controller/network/{network_id}`
    pub async fn network_delete_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::ControllerNetwork>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// List Network Member IDs
    ///
    /// Object containing all member IDs as keys and their memberRevisionCounter
    /// values as values
    ///
    /// Sends a `GET` request to `/controller/network/{network_id}/member`
    /// 
    /// Arguments:
    /// - `network_id`: Network ID of the Network
    pub async fn network_members_list_network_members<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::ControllerNetworkMemberList>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}/member",
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get Network Member by ID
    ///
    /// Sends a `GET` request to
    /// `/controller/network/{network_id}/member/{node_id}`
    ///
    /// Arguments:
    /// - `network_id`: Network ID of the Network
    /// - `node_id`: Node ID of the Network Member
    pub async fn network_member_get_network_member<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
        node_id: &'a types::ZtAddress,
    ) -> Result<
        ResponseValue<types::ControllerNetworkMember>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}/member/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
            encode_path(&node_id.to_string()),
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Create or Update Network Member
    ///
    /// Sends a `POST` request to
    /// `/controller/network/{network_id}/member/{node_id}`
    ///
    /// Arguments:
    /// - `network_id`: Network ID of the Network
    /// - `node_id`: Node ID of the Network Member
    /// - `body`
    pub async fn network_member_post_network_member<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
        node_id: &'a types::ZtAddress,
        body: &'a types::ControllerNetworkMemberRequest,
    ) -> Result<
        ResponseValue<types::ControllerNetworkMember>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}/member/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
            encode_path(&node_id.to_string()),
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Delete Network Member
    ///
    /// Sends a `DELETE` request to
    /// `/controller/network/{network_id}/member/{node_id}`
    ///
    /// Arguments:
    /// - `network_id`: Network ID of the Network
    /// - `node_id`: Node ID of the Network Member
    pub async fn network_member_del_network_member<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
        node_id: &'a types::ZtAddress,
    ) -> Result<
        ResponseValue<types::ControllerNetworkMember>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/controller/network/{}/member/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
            encode_path(&node_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// List Joined Networks
    /// 
    /// All the networks that this node is joined to
    /// 
    /// Sends a `GET` request to `/network`
    pub async fn network_membership_read_networks<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<Vec<types::JoinedNetwork>>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get Joined Network by ID
    /// 
    /// Joined Network by ID
    /// 
    /// Sends a `GET` request to `/network/{network_id}`
    pub async fn network_membership_get_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::JoinedNetwork>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Join or Update Network Settings
    /// 
    /// Sends a `POST` request to `/network/{network_id}`
    pub async fn network_membership_set_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
        body: &'a types::JoinedNetworkRequest,
    ) -> Result<
        ResponseValue<types::JoinedNetwork>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Leave Network
    /// 
    /// Sends a `DELETE` request to `/network/{network_id}`
    pub async fn network_membership_del_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::LeaveResult>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/network/{}",
            self.baseurl,
            encode_path(&network_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .delete(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// List Peers
    /// 
    /// All the nodes your node knows about
    /// 
    /// Sends a `GET` request to `/peer`
    pub async fn node_peer_read_networks<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<Vec<types::Peer>>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!("{}/peer", self.baseurl,);
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Get Joined Network by ID
    /// 
    /// Get Peer by ID
    /// 
    /// Sends a `GET` request to `/peer/{network_id}`
    pub async fn node_peer_get_network<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::Peer>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/peer/{}",
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// Node Status
    /// 
    /// Sends a `GET` request to `/status`
    pub async fn node_status_read_status<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::NodeStatus>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// List all networks
    /// 
    /// List all networks
    /// 
    /// Sends a `GET` request to `/unstable/controller/network`
    pub async fn network_read_networks2<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::ControllerNetworks>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!("{}/unstable/controller/network", self.baseurl,);
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    /// List all Network Members
    /// 
    /// List all Network Members
    /// 
    /// Sends a `GET` request to
    /// `/unstable/controller/network/{network_id}/member`
    ///
    /// Arguments:
    /// - `network_id`: Network ID of the Network
    pub async fn member_list_network_members2<'a>(
        &'a self,
        network_id: &'a types::ZtNetworkId,
    ) -> Result<
        ResponseValue<types::ControllerNetworkMemberListFull>,
        Error<::serde_json::Map<String, ::serde_json::Value>>,
    > {
        let url = format!(
            "{}/unstable/controller/network/{}/member",
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
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
