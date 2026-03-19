use base64::{Engine, engine::general_purpose::STANDARD};

fn main() {
    simple_soap();
    soap_with_attachment();
}

/// Simple SOAP 1.1 request (no attachments).
fn simple_soap() {
    println!("=== Simple SOAP Request ===\n");

    let url = "http://www.dneonline.com/calculator.asmx";

    let body = r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope
    xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Body>
    <Add xmlns="http://tempuri.org/">
      <intA>5</intA>
      <intB>4</intB>
    </Add>
  </soap:Body>
</soap:Envelope>"#;

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "\"http://tempuri.org/Add\"")
        .body(body)
        .send()
        .expect("request failed");

    let status = response.status();
    let text = response.text().expect("failed to read response body");

    println!("Status: {status}");
    println!("Response:\n{text}\n");
}

/// SOAP with Attachments (SwA) using MIME multipart/related.
///
/// Builds a multipart/related message per the SwA specification (W3C Note):
///   - The root part contains the SOAP envelope (Content-Type: text/xml).
///   - Subsequent parts carry binary attachments referenced by Content-ID.
///
/// The SOAP envelope references the attachment via `cid:` URI.
///
/// NOTE: This targets a mock endpoint that will likely return an error, since
/// there is no public SwA service available for testing. The example
/// demonstrates how to correctly construct the MIME message.
fn soap_with_attachment() {
    println!("=== SOAP with Attachments (SwA) ===\n");

    let url = "http://httpbin.org/post"; // echo service for demonstration

    let boundary = "MIME_boundary_soap_attachment";

    // A small PNG (1x1 transparent pixel) as a sample binary attachment.
    let png_bytes: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x62,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xE2, 0x21, 0xBC, 0x33, 0x00, 0x00, 0x00, 0x00,
        0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    let attachment_b64 = STANDARD.encode(png_bytes);

    // SOAP envelope referencing the attachment via cid: URI
    let soap_envelope = r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope
    xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Body>
    <UploadImage xmlns="http://example.org/imageservice">
      <fileName>pixel.png</fileName>
      <imageData href="cid:pixel.png@example.org"/>
    </UploadImage>
  </soap:Body>
</soap:Envelope>"#;

    // Build the MIME multipart/related body by hand.
    let mut body = String::new();

    // Root part: the SOAP envelope
    body.push_str(&format!("--{boundary}\r\n"));
    body.push_str("Content-Type: text/xml; charset=utf-8\r\n");
    body.push_str("Content-Transfer-Encoding: 8bit\r\n");
    body.push_str("Content-ID: <soap-envelope@example.org>\r\n");
    body.push_str("\r\n");
    body.push_str(soap_envelope);
    body.push_str("\r\n");

    // Attachment part: the PNG image (base64-encoded)
    body.push_str(&format!("--{boundary}\r\n"));
    body.push_str("Content-Type: image/png\r\n");
    body.push_str("Content-Transfer-Encoding: base64\r\n");
    body.push_str("Content-ID: <pixel.png@example.org>\r\n");
    body.push_str("\r\n");
    body.push_str(&attachment_b64);
    body.push_str("\r\n");

    // Closing boundary
    body.push_str(&format!("--{boundary}--\r\n"));

    let content_type = format!(
        "multipart/related; boundary=\"{boundary}\"; type=\"text/xml\"; start=\"<soap-envelope@example.org>\""
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", &content_type)
        .header("SOAPAction", "\"http://example.org/imageservice/UploadImage\"")
        .body(body)
        .send()
        .expect("request failed");

    let status = response.status();
    let text = response.text().expect("failed to read response body");

    println!("Status: {status}");
    println!("Response:\n{text}\n");
}

