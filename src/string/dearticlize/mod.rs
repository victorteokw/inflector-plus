/// Converts a `&str` to dearticlized `String`
///
/// ```
///     use inflector::string::dearticlize::dearticlize;
///     let mock_string: &str = "an egg";
///     let expected_string: String = "egg".to_owned();
///     let asserted_string: String = dearticlize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::dearticlize::dearticlize;
///     let mock_string: &str = "a user";
///     let expected_string: String = "user".to_owned();
///     let asserted_string: String = dearticlize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::dearticlize::dearticlize;
///     let mock_string: &str = "an hour";
///     let expected_string: String = "hour".to_owned();
///     let asserted_string: String = dearticlize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn dearticlize(non_dearticlized_string: &str) -> String {
    if non_dearticlized_string.starts_with("an ") {
        non_dearticlized_string[3..non_dearticlized_string.len()].to_owned()
    } else if non_dearticlized_string.starts_with("a ") {
        non_dearticlized_string[2..non_dearticlized_string.len()].to_owned()
    } else {
        non_dearticlized_string.to_owned()
    }
}

#[test]
fn dearticlize_removes_an() {
    assert_eq!("apple", dearticlize("an apple"));
    assert_eq!("egg", dearticlize("an egg"));
    assert_eq!("oddball", dearticlize("an oddball"));
}

#[test]
fn dearticlize_removes_a() {
    assert_eq!("banana", dearticlize("a banana"));
    assert_eq!("holy", dearticlize("a holy"));
    assert_eq!("yellow", dearticlize("a yellow"));
}
