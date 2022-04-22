//! # Very Good Security
//! `vgs` is a Rust library for the [Very Good Security](https://verygoodsecurity.com) API.
//! THIS PROJECT IS NOT COMPLETE, PLEASE DON'T USE IT.

/// Redacts sensitive data via the reverse proxy and stores it in a vault
///
/// # Examples
///
/// ```
/// use vgs::redact_via_reverse_proxy;
///
/// let original_data = "Mxolisi Makeba";
/// let reverse_http_proxy_host = "company.environment.verygoodproxy.com";
/// let result = redact_via_reverse_proxy(
///     original_data,
///     reverse_http_proxy_host
/// );
///
/// assert_eq!("Mxolisi Makeba", original_data);
/// ```
pub fn redact_via_reverse_proxy(original_data: &str, _reverse_http_proxy_host: &str) -> String {
    // TODO: Implement redacting sensitive information via the reverse proxy
    original_data.to_string()
}
