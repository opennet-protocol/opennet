use opennet_core::{OpenNetUri, ServiceId};
use crate::error::{ResolverError, Result};

pub fn parse_uri(uri: &str) -> Result<(ServiceId, String)> {
    let parsed = OpenNetUri::parse(uri).map_err(|e| ResolverError::InvalidUri(e.to_string()))?;
    Ok((parsed.service_id(), parsed.path))
}
