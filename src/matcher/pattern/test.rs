use super::Pattern;
use uuid::Uuid;

#[test]
fn test_given_json_document_when_it_does_not_contain_errors_then_pattern_can_be_created_from_it() {
    let buffer = r#"
{
  "name": "SSH_DISCONNECT",
  "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user",
  "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743",
  "values": {
      "key1": "value1",
      "key2": "value2"
  },
  "tags": ["tag1", "tag2"]
}
"#;

    let expected_uuid = Uuid::parse_str("9a49c47d-29e9-4072-be84-3b76c6814743").ok().unwrap();
    let expected_tags = &["tag1".to_string(), "tag2".to_string()];
    let result = Pattern::from_json(buffer);
    println!("{:?}", result);
    let pattern = result.ok().expect("Failed to deserialize a JSON Pattern");
    assert_eq!(pattern.name(), Some("SSH_DISCONNECT"));
    assert_eq!(pattern.uuid().as_bytes(), expected_uuid.as_bytes());
    assert_eq!(pattern.values().expect("Pattern created from JSON but it doesn't contain the expected additional values")
                                .get("key1")
                                .expect("Pattern created from JSON but it doesn't contain the expected additional value"), "value1");
    assert_eq!(pattern.values().expect("Pattern created from JSON but it doesn't contain the expected additional values")
                                .get("key2")
                                .expect("Pattern created from JSON but it doesn't contain the expected additional value"), "value2");
    assert_eq!(pattern.values().expect("").get("key3"), None);
    assert_eq!(pattern.tags().expect("Pattern created from JSON but it doesn't contain the expected tags"), expected_tags);
    assert_eq!(pattern.pattern().len(), 15);
}

#[test]
fn test_given_json_pattern_when_it_does_not_have_the_optional_paramaters_then_pattern_can_be_built_from_it() {
    let buffer = r#"
{
  "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743",
  "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user"
}
"#;

    let pattern = Pattern::from_json(buffer).ok().expect("Failed to deserialize a JSON Pattern");
    assert_eq!(pattern.name(), None);
}

#[test]
fn test_given_json_pattern_when_its_uuid_is_invalid_then_pattern_cannot_be_built_from_it() {
    let buffer = r#"
{
  "uuid": "sdfsdf-12f-sdfd--23",
  "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user"
}
"#;

    let _ = Pattern::from_json(buffer).err().expect("We created a Pattern with an invalid Uuid");
}

#[test]
fn test_given_json_pattern_when_its_pattern_is_invalid_then_pattern_cannot_be_built_from_it() {
    let buffer = r#"
{
  "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743",
  "pattern": "Jun %{INT:da"
}
"#;

    let _ = Pattern::from_json(buffer).err().expect("We created a Pattern with an invalid pattern field");
}

#[test]
fn test_given_json_pattern_when_test_messages_are_specified_then_they_are_parsed() {
    let buffer = r#"
{
  "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743",
  "pattern": "Jun %{INT:day}",
  "test_messages": [
      {
          "message": "Jun 1",
          "values": {
              "day": "1"
          }
      }
  ]
}
"#;

    let _ = Pattern::from_json(buffer).ok().expect("Failed to create a Pattern when test_messages are specified");
}

#[test]
fn test_given_json_pattern_with_invalid_uuid_when_we_try_to_create_pattern_then_it_fails() {
    let buffer = r#"
{
  "uuid": "not valid uuid",
  "pattern": "Jun %{INT:day}"
}
"#;

    let _ = Pattern::from_json(buffer).err().expect("We created a Pattern with an invalid uuid field");
}

#[test]
fn test_given_json_pattern_when_it_does_not_have_the_pattern_field_then_it_cannot_be_created() {
    let buffer = r#"
{
  "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743",
}
"#;

    let _ = Pattern::from_json(buffer).err().expect("We created a Pattern without the pattern field");
}
