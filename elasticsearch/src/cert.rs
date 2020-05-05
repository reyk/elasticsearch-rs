/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//! Certificate components
pub use reqwest::Certificate;

/// Validation applied to a SSL/TLS certificate, to establish a HTTPS connection.
///
/// # Examples
///
/// ## Default
///
/// The client is configured by default to validate that a certificate used to establish a
/// HTTPS connection is one that is signed by a trusted Certificate Authority (CA) and passes
/// hostname verification. [CertificateValidation::Default] is a provided variant only to
/// be able to change from another validation mode back to the default.
///
/// ## Full validation
///
/// This requires the `native-tls`, or `rustls-tls` feature to be enabled.
///
/// With Elasticsearch running at `https://example.com`, configured to use a certificate generated
/// with your own Certificate Authority (CA), and where the certificate contains a CommonName (CN)
/// or Subject Alternative Name (SAN) that matches the hostname of Elasticsearch
#[cfg_attr(
    any(feature = "native-tls", feature = "rustls-tls"),
    doc = r##"
```rust,norun
# use elasticsearch::{
#     auth::Credentials,
#     cert::{Certificate,CertificateValidation},
#     Error, Elasticsearch,
#     http::transport::{TransportBuilder,SingleNodeConnectionPool},
# };
# use std::fs::File;
# use std::io::Read;
# use url::Url;
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
let url = Url::parse("https://example.com")?;
let conn_pool = SingleNodeConnectionPool::new(url);

// load the CA certificate
let mut buf = Vec::new();
File::open("my_ca_cert.pem")?
    .read_to_end(&mut buf)?;
let cert = Certificate::from_pem(&buf)?;

let transport = TransportBuilder::new(conn_pool)
    .cert_validation(CertificateValidation::Full(cert))
    .build()?;
let client = Elasticsearch::new(transport);
let _response = client.ping().send().await?;
# Ok(())
# }
```
"##
)]
/// ## Certificate validation
///
/// This requires the `native-tls` feature to be enabled.
///
/// With Elasticsearch running at `https://example.com`, configured to use a certificate generated
/// with your own Certificate Authority (CA)
#[cfg_attr(
    feature = "native-tls",
    doc = r##"
```rust,norun
# use elasticsearch::{
#     auth::Credentials,
#     cert::{Certificate,CertificateValidation},
#     Error, Elasticsearch,
#     http::transport::{TransportBuilder,SingleNodeConnectionPool},
# };
# use std::fs::File;
# use std::io::Read;
# use url::Url;
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
let url = Url::parse("https://example.com")?;
let conn_pool = SingleNodeConnectionPool::new(url);

// load the CA certificate
let mut buf = Vec::new();
File::open("my_ca_cert.pem")?
    .read_to_end(&mut buf)?;
let cert = Certificate::from_pem(&buf)?;
let transport = TransportBuilder::new(conn_pool)
    .cert_validation(CertificateValidation::Certificate(cert))
    .build()?;
let client = Elasticsearch::new(transport);
let _response = client.ping().send().await?;
# Ok(())
# }
```
"##
)]
/// ## No validation
///
/// No validation is performed on the certificate provided by the server.
/// **Use on production clusters is strongly discouraged**
///
/// ```rust,norun
/// # use elasticsearch::{
/// #     auth::Credentials,
/// #     cert::{Certificate,CertificateValidation},
/// #     Error, Elasticsearch,
/// #     http::transport::{TransportBuilder,SingleNodeConnectionPool},
/// # };
/// # use std::fs::File;
/// # use std::io::Read;
/// # use url::Url;
/// # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
/// let url = Url::parse("https://example.com")?;
/// let conn_pool = SingleNodeConnectionPool::new(url);
/// let transport = TransportBuilder::new(conn_pool)
///     .cert_validation(CertificateValidation::None)
///     .build()?;
/// let client = Elasticsearch::new(transport);
/// let _response = client.ping().send().await?;
/// # Ok(())
/// # }
/// ```
pub enum CertificateValidation {
    /// Default validation of the certificate, which validates that the certificate provided by the
    /// server is signed by a trusted Certificate Authority (CA) and also verifies that the server’s hostname
    /// (or IP address) matches the names identified by the CommonName (CN) or Subject Alternative
    /// Name (SAN) within the certificate.
    ///
    /// A trusted CA is one that is trusted by the operating system on which the client is running,
    /// which typically means that the CA certificate is in the certificate/truststore of the
    /// operating system. This is the default mode of operation.
    Default,
    /// Full validation of the certificate, which validates that the certificate provided by the
    /// server is signed by a trusted Certificate Authority (CA) and also verifies that the server’s hostname
    /// (or IP address) matches the names identified by the CommonName (CN) or Subject Alternative
    /// Name (SAN) within the certificate.
    ///
    /// This is useful for self-signed certificates generated by your own CA,
    /// where the certificate contains the CommonName (CN) or a Subject Alternative Name (SAN)
    /// that matches the server hostname.
    ///
    /// Typically, the certificate provided to the client is the Certificate Authority (CA)
    /// used to sign the certificate used by the server.
    ///
    /// # Optional
    /// This requires the `native-tls`, or `rustls-tls` feature to be enabled.
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    Full(Certificate),
    /// Validates that the certificate provided by the server is signed by a trusted
    /// Certificate Authority (CA), but does not perform hostname verification.
    ///
    /// This is useful for self-signed certificates generated by your own CA
    /// that **do not** contain the CommonName (CN) or a Subject Alternative Name (SAN)
    /// that matches the server hostname.
    ///
    /// Typically, the certificate provided to the client will be the Certificate Authority (CA)
    /// used to sign the certificate used by the server.
    ///
    /// # Optional
    ///
    /// This requires the `native-tls` feature to be enabled.
    #[cfg(feature = "native-tls")]
    Certificate(Certificate),
    /// No validation is performed on the certificate provided by the server.
    ///
    /// This disables many of the security benefits of SSL/TLS and should only be used after very
    /// careful consideration. It is primarily intended as a temporary diagnostic mechanism when
    /// attempting to resolve TLS errors, and **its use on production clusters is strongly discouraged**.
    None,
}
