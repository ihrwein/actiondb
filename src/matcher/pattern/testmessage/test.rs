use super::TestMessage;
use serde_json;
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
},
"tags": ["tag1", "tag2"]
}
"#;
    let result = serde_json::from_str::<TestMessage>(buffer);
    let expexted_tags = &["tag1".to_owned(), "tag2".to_owned()];
    println!("{:?}", result);
    let msg = result.expect("Failed to deserialize a valid TestMessage from JSON");
    assert_eq!("lame-servers: info: unexpected RCODE (REFUSED) resolving \
                'ns1.example.org/AAAA/IN': 192.0.2.1#53",
               msg.message());
    assert_eq!(6, msg.values().len());
    assert_eq!(msg.tags().expect("Deserialized TestMessage doesn't have the expected tags"),
               expexted_tags);
    assert_eq!(Some("AAAA"),
               msg.values().get("dnsqry.type").map(|x| x.borrow()));
}

#[test]
fn test_given_json_test_message_when_it_does_not_have_a_message_field_then_error_is_returned() {
    let buffer = r#"
{
"values": {
"dnsqry.query": "ns1.example.org",
}
}
"#;
    let result = serde_json::from_str::<TestMessage>(buffer);
    println!("{:?}", result);
    let _ = result.err()
                  .expect("Failed to return error when a serialized TestMessage doesn't have a \
                           message field");
}

#[test]
fn test_given_json_test_message_when_it_does_not_have_the_optional_fields_then_it_can_be_loaded_successfully
    () {
    let buffer = r#"
{
"message": "lame-servers: info: unexpected RCODE (REFUSED) resolving 'ns1.example.org/AAAA/IN': 192.0.2.1#53"
}
"#;
    let result = serde_json::from_str::<TestMessage>(buffer);
    println!("{:?}", result);
    let msg = result.expect("Failed to deserialize a valid TestMessage from JSON when it doesn't \
                             contain values");
    assert_eq!("lame-servers: info: unexpected RCODE (REFUSED) resolving \
                'ns1.example.org/AAAA/IN': 192.0.2.1#53",
               msg.message());
    assert_eq!(0, msg.values().len());
}

#[test]
fn test_given_json_test_message_when_it_contains_not_just_the_valid_fields_then_we_return_an_error
    () {
    let buffer = r#"
{
"message": "lame-servers: info: unexpected RCODE (REFUSED) resolving 'ns1.example.org/AAAA/IN': 192.0.2.1#53",
"field": "this field is not in TestMessage"
}
"#;
    let result = serde_json::from_str::<TestMessage>(buffer);
    println!("{:?}", result);
    let _ = result.err().expect("Failed to return an error when a serialized TestMessage \
                                 contains non-valid fields");
}
