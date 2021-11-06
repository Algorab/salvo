//! The extra lib of Savlo web server framework.
#![doc(html_favicon_url = "https://salvo.rs/images/favicon-32x32.png")]
#![doc(html_logo_url = "https://salvo.rs/images/logo.svg")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(private_in_public, unreachable_pub)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[cfg(feature = "basic_auth")]
pub mod basic_auth;
#[cfg(feature = "jwt_auth")]
pub mod jwt_auth;

#[cfg(feature = "compression")]
pub mod compression;
#[cfg(feature = "cors")]
pub mod cors;
#[cfg(feature = "csrf")]
pub mod csrf;
#[cfg(feature = "proxy")]
pub mod proxy;
#[cfg(feature = "serve")]
pub mod serve;
#[cfg(feature = "sse")]
pub mod sse;
#[cfg(feature = "ws")]
pub mod ws;

#[cfg(feature = "size_limiter")]
pub mod size_limiter;
