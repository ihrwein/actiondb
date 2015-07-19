use super::Pattern;
use uuid::Uuid;

#[test]
fn test_given_json_document_when_it_does_not_contain_errors_then_pattern_can_be_created_from_it() {
    let buffer = r#"
{
  "name": "SSH_DISCONNECT",
  "pattern": "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user",
  "uuid": "9a49c47d-29e9-4072-be84-3b76c6814743"
}
"#;

    let expected_uuid = Uuid::parse_str("9a49c47d-29e9-4072-be84-3b76c6814743").ok().unwrap();
    let result = Pattern::from_json(buffer);
    println!("{:?}", result);
    let pattern = result.ok().expect("Failed to deserialize a JSON Pattern");
    assert_eq!(pattern.name(), Some("SSH_DISCONNECT"));
    assert_eq!(pattern.uuid().as_bytes(), expected_uuid.as_bytes());
    assert_eq!(pattern.pattern().len(), 15);
}

#[test]
fn test_given_json_pattern_when_it_does_not_have_name_then_pattern_can_be_built_from_it() {
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

