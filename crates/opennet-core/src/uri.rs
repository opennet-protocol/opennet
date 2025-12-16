//! URI parsing for open:// and service:// schemes.
//!
//! RFC: OpenNet Core Protocol §8

use serde::{Deserialize, Serialize};
use crate::error::{CoreError, Result};
use crate::scope::Scope;
use crate::service_id::ServiceId;
use crate::constants::{URI_SCHEME_OPEN, URI_SCHEME_SERVICE};

/// Parsed open:// URI.
///
/// Format: `open://[scope.]domain/path`
///
/// Examples:
/// - `open://chat.opennet/room/123`
/// - `open://eu.storage.opennet/files`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenNetUri {
    /// Optional scope prefix.
    pub scope: Scope,
    /// Domain name.
    pub domain: String,
    /// Path component.
    pub path: String,
    /// Query parameters.
    pub query: Option<String>,
}

impl OpenNetUri {
    /// Parse an open:// URI.
    pub fn parse(uri: &str) -> Result<Self> {
        let uri = uri.trim();
        
        // Check scheme
        let rest = uri.strip_prefix("open://")
            .ok_or_else(|| CoreError::InvalidUri("missing open:// scheme".into()))?;
        
        // Split path
        let (authority, path_query) = match rest.find('/') {
            Some(idx) => (&rest[..idx], &rest[idx..]),
            None => (rest, "/"),
        };
        
        // Split query
        let (path, query) = match path_query.find('?') {
            Some(idx) => (&path_query[..idx], Some(path_query[idx+1..].to_string())),
            None => (path_query, None),
        };
        
        // Parse scope and domain from authority
        let (scope, domain) = Self::parse_authority(authority)?;
        
        Ok(Self {
            scope,
            domain,
            path: path.to_string(),
            query,
        })
    }

    /// Parse authority into scope and domain.
    fn parse_authority(authority: &str) -> Result<(Scope, String)> {
        let parts: Vec<&str> = authority.split('.').collect();
        
        if parts.len() < 2 {
            return Err(CoreError::InvalidUri("invalid domain".into()));
        }
        
        // Check for scope prefix (e.g., "eu.chat.opennet")
        if parts.len() >= 3 {
            let potential_scope = parts[0];
            if Self::is_scope_prefix(potential_scope) {
                let scope = Scope::parse(&format!("region.{}", potential_scope))?;
                let domain = parts[1..].join(".");
                return Ok((scope, domain));
            }
        }
        
        Ok((Scope::Global, authority.to_string()))
    }

    /// Check if string is a known scope prefix.
    fn is_scope_prefix(s: &str) -> bool {
        matches!(s, "eu" | "us" | "asia" | "anon" | "test" | "dev")
    }

    /// Get the ServiceId for this URI.
    pub fn service_id(&self) -> ServiceId {
        ServiceId::from_domain(&self.domain)
    }

    /// Convert back to URI string.
    pub fn to_string(&self) -> String {
        let scope_prefix = match &self.scope {
            Scope::Global => String::new(),
            Scope::Region(r) => format!("{}.", r),
            _ => String::new(),
        };
        
        let query_suffix = match &self.query {
            Some(q) => format!("?{}", q),
            None => String::new(),
        };
        
        format!("open://{}{}{}{}", scope_prefix, self.domain, self.path, query_suffix)
    }
}

/// Parsed service:// URI (internal addressing).
///
/// Format: `service://service_id_hex/path`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceUri {
    /// Service identifier.
    pub service_id: ServiceId,
    /// Path component.
    pub path: String,
}

impl ServiceUri {
    /// Parse a service:// URI.
    pub fn parse(uri: &str) -> Result<Self> {
        let rest = uri.strip_prefix("service://")
            .ok_or_else(|| CoreError::InvalidUri("missing service:// scheme".into()))?;
        
        let (id_hex, path) = match rest.find('/') {
            Some(idx) => (&rest[..idx], &rest[idx..]),
            None => (rest, "/"),
        };
        
        let service_id = ServiceId::from_hex(id_hex)?;
        
        Ok(Self {
            service_id,
            path: path.to_string(),
        })
    }
}

impl std::fmt::Display for OpenNetUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
