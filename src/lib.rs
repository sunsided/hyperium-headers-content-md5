//! Provides the [`ContentMd5`] typed header.
//!
//! # Example
//!
//! ```
//! use headers::Header;
//! use http::HeaderValue;
//! use headers_content_md5::ContentMd5;
//!
//! let value = HeaderValue::from_static("Q2hlY2sgSW50ZWdyaXR5IQ==");
//! let md5 = ContentMd5::decode(&mut [&value].into_iter()).unwrap();
//! assert_eq!(md5.0, "Check Integrity!".as_bytes())
//! ```

#![deny(unsafe_code)]
#![deny(unused_must_use)]

use base64::{engine::general_purpose::STANDARD as base64, Engine};
use headers::{Header, HeaderValue};

/// `Content-MD5` header, defined in
/// [RFC1864](https://datatracker.ietf.org/doc/html/rfc1864)
///
/// ## Example values
///
/// * `Q2hlY2sgSW50ZWdyaXR5IQ==`
///
/// # Example
///
/// Decoding:
///
/// ```
/// use headers::Header;
/// use http::HeaderValue;
/// use headers_content_md5::ContentMd5;
///
/// let value = HeaderValue::from_static("Q2hlY2sgSW50ZWdyaXR5IQ==");
/// let mut values = [&value].into_iter();
///
/// let md5 = ContentMd5::decode(&mut values).unwrap();
/// assert_eq!(md5.0, "Check Integrity!".as_bytes())
/// ```
///
/// Encoding:
///
/// ```
/// use headers::Header;
/// use http::HeaderValue;
/// use headers_content_md5::ContentMd5;
///
/// let value = HeaderValue::from_static("Q2hlY2sgSW50ZWdyaXR5IQ==");
/// let md5 = ContentMd5("Check Integrity!".as_bytes().try_into().unwrap());
///
/// let mut header = Vec::default();
/// md5.encode(&mut header);
/// assert_eq!(header[0], "Q2hlY2sgSW50ZWdyaXR5IQ==");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContentMd5(pub [u8; 16]);

static CONTENT_MD5: http::header::HeaderName = http::header::HeaderName::from_static("content-md5");

impl Header for ContentMd5 {
    fn name() -> &'static http::header::HeaderName {
        &CONTENT_MD5
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(
        values: &mut I,
    ) -> Result<Self, headers::Error> {
        let value = values.next().ok_or_else(headers::Error::invalid)?;

        // Ensure base64 encoded length fits the expected MD5 digest length.
        if value.len() < 22 || value.len() > 24 {
            return Err(headers::Error::invalid());
        }

        let value = value.to_str().map_err(|_| headers::Error::invalid())?;
        let mut buffer = [0; 18];
        base64
            .decode_slice(value, &mut buffer)
            .map_err(|_| headers::Error::invalid())?;
        let mut slice = [0; 16];
        slice.copy_from_slice(&buffer[..16]);
        Ok(Self(slice))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let encoded = base64.encode(self.0);
        if let Ok(value) = HeaderValue::from_str(&encoded) {
            values.extend(std::iter::once(value));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ContentMd5;
    use headers::Header;
    use http::HeaderValue;

    #[test]
    fn decode_works() {
        let value = HeaderValue::from_static("Q2hlY2sgSW50ZWdyaXR5IQ==");
        let md5 = ContentMd5::decode(&mut [&value].into_iter()).unwrap();
        assert_eq!(md5.0, "Check Integrity!".as_bytes())
    }

    #[test]
    fn encode_works() {
        let md5 = ContentMd5("Check Integrity!".as_bytes().try_into().unwrap());
        let mut header = Vec::default();
        md5.encode(&mut header);
        assert_eq!(header[0], "Q2hlY2sgSW50ZWdyaXR5IQ==");
    }
}
