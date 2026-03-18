fn main() {
    let url = "http://www.dneonline.com/calculator.asmx";

    let body = r#"<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope
    xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <soap:Body>
    <Add xmlns="http://tempuri.org/">
      <intA>5</intA>
      <intB>3</intB>
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
    println!("Response:\n{text}");
}
