# Content-MD5 header support for hyperium/headers

This adds the [RFC1864](https://datatracker.ietf.org/doc/html/rfc1864) `Content-MD5` header as a typed header:

```rust
use headers::Header;
use http::HeaderValue;
use headers_content_md5::ContentMd5;

fn it_works() {
    let value = HeaderValue::from_static("Q2hlY2sgSW50ZWdyaXR5IQ==");
    let md5 = ContentMd5::decode(&mut [&value].into_iter()).unwrap();
    
    let expected = "Check Integrity!".as_bytes().try_into().unwrap();
    assert_eq!(md5, ContentMd5(expected))
}
```
