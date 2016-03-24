use actiondb::matcher::PatternLoader;
use actiondb::matcher::BuildError;

use std::env;

const DIR_PREFIX: &'static str = "tests/file";

#[test]
fn test_given_a_valid_json_pattern_file_when_it_is_deserialized_then_we_can_extract_the_patterns_from_it
    () {
    println!("dir: {:?}", env::current_dir());
    let file_name = format!("{}/ssh_ok.json", DIR_PREFIX);
    let file = PatternLoader::load_file(&file_name)
                   .ok()
                   .expect("Failed to load JSON serialized Pattern");
    assert_eq!(file.patterns().len(), 3);
}

#[test]
fn test_given_an_invalid_json_pattern_file_when_it_is_deserialized_then_we_get_deserialization_error
    () {
    println!("dir: {:?}", env::current_dir());
    let file_name = format!("{}/ssh_wrong.json", DIR_PREFIX);
    match PatternLoader::load_file(&file_name) {
        Err(BuildError::DeserJson(err)) => {
            println!("{:?}", err);
        }
        Ok(_) | Err(_) => unreachable!(),
    }
}

#[test]
fn test_given_a_non_existing_pattern_file_when_it_is_deserialized_then_we_get_io_error() {
    println!("dir: {:?}", env::current_dir());
    let file_name = format!("{}/ssh_non_existing.json", DIR_PREFIX);
    match PatternLoader::load_file(&file_name) {
        Err(BuildError::Io(err)) => {
            println!("{:?}", err);
        }
        Ok(_) | Err(_) => unreachable!(),
    }
}

#[test]
fn test_given_valid_yaml_pattern_file_when_it_is_deserialized_then_we_get_the_deserialized_contents() {
    let file = PatternLoader::load_file("tests/file/ssh_ok.yaml")
                   .ok()
                   .expect("Failed to load YAML serialized pattern file");
    assert_eq!(file.patterns().len(), 3);
}

#[test]
fn test_given_not_existing_yaml_pattern_file_when_it_is_opened_we_get_io_error() {
    let _ = PatternLoader::load_file("tests/file/ssh_non_existing.yaml")
                   .err()
                   .expect("Successfully opened a non-existing YAML file");
}
