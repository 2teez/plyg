
use super::*;

#[test]
fn test_get_new_file_name() {
    assert_eq!(
        "hello.go",
        get_new_file_name(/*filename*/ "hello.rs", /*file extension*/ "go")
    );
}

#[test]
fn test_get_new_file_name_with_several_ext() {
    assert_eq!(
        "hello.copy.new.dart",
        get_new_file_name(
            /*filename*/ "hello.copy.new.c",
            /*file extension*/ "dart"
        ),
        "It changes only the last extension for the file"
    );
}
