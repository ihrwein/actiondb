use super::Pattern;
use uuid::Uuid;
/*
#[test]
fn test_given_yaml_document_when_it_does_not_contain_errors_then_pattern_can_be_created_from_it() {
    let buffer = r#"
name: SSH_DISCONNECT
uuid: 9a49c47d-29e9-4072-be84-3b76c6814743
pattern: "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user"
    "#;

    let docs = YamlLoader::load_from_str(buffer).unwrap();
    assert_eq!(docs.len(), 1);
    let doc = &docs[0];
    let expected_uuid = Uuid::parse_str("9a49c47d-29e9-4072-be84-3b76c6814743").ok().unwrap();
    let pattern = Pattern::from_yaml(doc).ok().unwrap();
    assert_eq!(pattern.name(), Some("SSH_DISCONNECT"));
    assert_eq!(pattern.uuid().as_bytes(), expected_uuid.as_bytes());
    assert_eq!(pattern.pattern().len(), 15);
}

#[test]
fn test_given_yaml_pattern_when_it_does_not_have_name_then_pattern_can_be_built_from_it() {
    let buffer = r#"
uuid: 9a49c47d-29e9-4072-be84-3b76c6814743
pattern: "Jun %{INT:day} %{INT:hour}:%{INT:min}:%{INT:sec} lobotomy sshd[%{INT:pid}]: Received disconnect from %{GREEDY:ipaddr}: %{INT:dunno}: disconnected by user"
    "#;

    let docs = YamlLoader::load_from_str(buffer).unwrap();
    assert_eq!(docs.len(), 1);
    let doc = &docs[0];
    let pattern = Pattern::from_yaml(doc).ok().unwrap();
    assert_eq!(pattern.name(), None);
}
*/
