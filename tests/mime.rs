use async_std::io;
use http_types::{mime, Body, Response};

#[async_std::test]
async fn guess_plain_text_mime() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/index.html").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(mime::HTML));
    Ok(())
}

#[async_std::test]
async fn guess_binary_mime() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/nori.png").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(mime::PNG));
    Ok(())
}

#[async_std::test]
async fn guess_mime_fallback() -> io::Result<()> {
    let body = Body::from_file("tests/fixtures/unknown.custom_ext").await?;
    let mut res = Response::new(200);
    res.set_body(body);
    assert_eq!(res.content_type(), Some(mime::BYTE_STREAM));
    Ok(())
}
