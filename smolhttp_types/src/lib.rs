use std::fmt;
use std::ops::Deref;

impl From<String> for Method {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

/// An HTTP request. This does not include a body, which is expected to be handled separately from
/// the request itself.
///
/// Maps to `HTTP-message` in the HTTP grammar, restricted to HTTP/1.1 requests.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Request {
    /// The request method. Maps to `method` in the HTTP grammar.
    pub method: Method,

    /// The request target. This is usually, but not always, a path. Maps to `request-target` in
    /// the HTTP grammar.
    pub request_target: String,

    /// The request headers. Maps to `*( header-field CRLF )` in the HTTP grammar.
    pub headers: Vec<Header>,
}

/// An HTTP response. This does not include a body, which is expected to be handled separately from
/// the response itself.
///
/// Maps to `HTTP-message` in the HTTP grammar, restricted to HTTP/1.1 responses.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Response {
    /// The status code. Maps to `status-code` in the HTTP grammar.
    pub status_code: StatusCode,

    /// A textual description of the status code. Maps to `reason-phrase` in the HTTP grammar.
    pub reason_phrase: String,

    /// The response headers. Maps to `*( header-field CRLF )` in the HTTP grammar.
    pub headers: Vec<Header>,
}

/// The status code of an HTTP response. The complete list is maintained by
/// [IANA](http://www.iana.org/assignments/http-status-codes). Maps to `status-code` in the HTTP
/// grammar.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StatusCode(u16);

impl StatusCode {
    /// Construct a `StatusCode`. Returns `None` if the provided code is outside the valid range.
    pub fn new(code: u16) -> Option<Self> {
        if (100..600).contains(&code) {
            Some(Self(code))
        } else {
            None
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(code: StatusCode) -> Self {
        code.0
    }
}

// TODO: more trait implementations

/// The method of an HTTP request. The complete list is maintained by
/// [IANA](http://www.iana.org/assignments/http-methods).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Method(pub String);

impl Deref for Method {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0[..]
    }
}

/// An HTTP header. Maps to `header-field` in the HTTP grammar.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Header {
    /// The name of the header. A registry is maintained by
    /// [IANA](http://www.iana.org/assignments/message-headers), but registration is not required
    /// by the specification. Maps to `field-name`.
    pub name: String,

    /// The value of the header. Maps to `field-value`.
    pub value: String,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
