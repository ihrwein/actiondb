use super::TestMessage;
use serde::json;
use std::borrow::Borrow;

#[test]
fn test_given_json_test_message_when_it_is_deserialized_then_we_get_the_right_instance() {
    let buffer = r#"
{
"message": "lame-servers: info: unexpected RCODE (REFUSED) resolving 'ns1.example.org/AAAA/IN': 192.0.2.1#53",
"values": {
"dnsqry.query": "ns1.example.org",
"dnsqry.type": "AAAA",
"dnsqry.class": "IN",
"dnsqry.client_ip": "192.0.2.1",
"dnsqry.client_port": "53",
"dnslame.reason": "unexpected RCODE (REFUSED)"
}
}
"#;
    let result = json::from_str::<TestMessage>(buffer);
    println!("{:?}", result);
    let msg = result.ok().expect("Failed to deserialize a valid TestMessage from JSON");
    assert_eq!("lame-servers: info: unexpected RCODE (REFUSED) resolving 'ns1.example.org/AAAA/IN': 192.0.2.1#53", msg.message());
    assert_eq!(6, msg.values().len());
    assert_eq!(Some("AAAA"), msg.values().get("dnsqry.type").map(|x| x.borrow()));
}
