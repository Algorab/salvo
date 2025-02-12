use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use super::{separated_by_commas, Any, WILDCARD};
use salvo_core::http::header::{self, HeaderName, HeaderValue};
use salvo_core::{Depot, Request};

/// Holds configuration for how to set the [`Access-Control-Allow-Headers`][mdn] header.
///
/// See [`Cors::allow_headers`] for more details.
///
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Headers
/// [`Cors::allow_headers`]: super::Cors::allow_headers
#[derive(Clone, Default)]
#[must_use]
pub struct AllowHeaders(AllowHeadersInner);

type JudgeFn = Arc<dyn for<'a> Fn(&'a HeaderValue, &'a Request, &'a Depot) -> HeaderValue + Send + Sync + 'static>;
impl AllowHeaders {
    /// Allow any headers by sending a wildcard (`*`)
    ///
    /// See [`Cors::allow_headers`] for more details.
    ///
    /// [`Cors::allow_headers`]: super::Cors::allow_headers
    pub fn any() -> Self {
        Self(AllowHeadersInner::Exact(WILDCARD.clone()))
    }

    /// Set multiple allowed headers
    ///
    /// See [`Cors::allow_headers`] for more details.
    ///
    /// [`Cors::allow_headers`]: super::Cors::allow_headers
    pub fn list<I>(headers: I) -> Self
    where
        I: IntoIterator<Item = HeaderName>,
    {
        let headers = headers.into_iter().map(Into::into);
        match separated_by_commas(headers) {
            None => Self(AllowHeadersInner::None),
            Some(v) => Self(AllowHeadersInner::Exact(v)),
        }
    }

    /// Allow custom allow headers based on a given predicate
    ///
    /// See [`Cors::allow_headers`] for more details.
    ///
    /// [`Cors::allow_headers`]: super::Cors::allow_headers
    pub fn judge<F>(f: F) -> Self
    where
        F: Fn(&HeaderValue, &Request, &Depot) -> HeaderValue + Send + Sync + 'static,
    {
        Self(AllowHeadersInner::Judge(Arc::new(f)))
    }

    /// Allow any headers, by mirroring the preflight [`Access-Control-Request-Headers`][mdn]
    /// header.
    ///
    /// See [`Cors::allow_headers`] for more details.
    ///
    /// [`Cors::allow_headers`]: super::Cors::allow_headers
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Request-Headers
    pub fn mirror_request() -> Self {
        Self(AllowHeadersInner::MirrorRequest)
    }

    pub(super) fn is_wildcard(&self) -> bool {
        matches!(&self.0, AllowHeadersInner::Exact(v) if v == WILDCARD)
    }

    pub(super) fn to_header(
        &self,
        origin: Option<&HeaderValue>,
        req: &Request,
        depot: &Depot,
    ) -> Option<(HeaderName, HeaderValue)> {
        let allow_headers = match &self.0 {
            AllowHeadersInner::None => return None,
            AllowHeadersInner::Exact(v) => v.clone(),
            AllowHeadersInner::Judge(f) => f(origin?, req, depot),
            AllowHeadersInner::MirrorRequest => req.headers().get(header::ACCESS_CONTROL_REQUEST_HEADERS)?.clone(),
        };

        Some((header::ACCESS_CONTROL_ALLOW_HEADERS, allow_headers))
    }
}

impl fmt::Debug for AllowHeaders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            AllowHeadersInner::None => f.debug_tuple("None").finish(),
            AllowHeadersInner::Exact(inner) => f.debug_tuple("Exact").field(inner).finish(),
            AllowHeadersInner::Judge(_) => f.debug_tuple("Judge").finish(),
            AllowHeadersInner::MirrorRequest => f.debug_tuple("MirrorRequest").finish(),
        }
    }
}

impl From<Any> for AllowHeaders {
    fn from(_: Any) -> Self {
        Self::any()
    }
}

impl<const N: usize> From<[HeaderName; N]> for AllowHeaders {
    fn from(arr: [HeaderName; N]) -> Self {
        Self::list(arr)
    }
}

impl From<Vec<HeaderName>> for AllowHeaders {
    fn from(vec: Vec<HeaderName>) -> Self {
        Self::list(vec)
    }
}

impl<'a> From<&'a str> for AllowHeaders {
    fn from(val: &'a str) -> Self {
        Self::list([HeaderName::from_str(val).unwrap()])
    }
}

impl<'a> From<&'a String> for AllowHeaders {
    fn from(val: &'a String) -> Self {
        Self::list([HeaderName::from_str(val).unwrap()])
    }
}

impl<'a> From<Vec<&'a str>> for AllowHeaders {
    fn from(vals: Vec<&'a str>) -> Self {
        Self::list(
            vals.into_iter()
                .map(|v| HeaderName::from_str(v).unwrap())
                .collect::<Vec<_>>(),
        )
    }
}
impl<'a> From<&'a Vec<String>> for AllowHeaders {
    fn from(vals: &'a Vec<String>) -> Self {
        Self::list(
            vals.iter()
                .map(|v| HeaderName::from_str(v).unwrap())
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Clone)]
enum AllowHeadersInner {
    None,
    Exact(HeaderValue),
    Judge(JudgeFn),
    MirrorRequest,
}

impl Default for AllowHeadersInner {
    fn default() -> Self {
        Self::None
    }
}
