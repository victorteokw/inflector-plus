static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

macro_rules! special_a_cases{
    ($s:ident, $($word: expr), *) => {
        $(
            if $s[..].starts_with($word) {
                let mut result = "a ".to_owned();
                result.push_str($s);
                return result;
            }
        )*
    }
}

macro_rules! special_an_cases{
    ($s:ident, $($word: expr), *) => {
        $(
            if $s[..].starts_with($word) {
                let mut result = "an ".to_owned();
                result.push_str($s);
                return result;
            }
        )*
    }
}

/// Converts a `&str` to articlized `String`
///
/// ```
///     use inflector::string::articlize::articlize;
///     let mock_string: &str = "egg";
///     let expected_string: String = "an egg".to_owned();
///     let asserted_string: String = articlize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::articlize::articlize;
///     let mock_string: &str = "user";
///     let expected_string: String = "a user".to_owned();
///     let asserted_string: String = articlize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
/// ```
///     use inflector::string::articlize::articlize;
///     let mock_string: &str = "hour";
///     let expected_string: String = "an hour".to_owned();
///     let asserted_string: String = articlize(mock_string);
///     assert!(asserted_string == expected_string);
///
/// ```
pub fn articlize(non_articlized_string: &str) -> String {
    special_a_cases![non_articlized_string,
        "euro",
        "user",
        "uni",
        "one",
        "use",
        "ut",
        "uranium",
        "urine",
        "unanimity",
        "euthanasia",
        "ewe",
        "ufo",
        "usu",
        "ub"
    ];
    special_an_cases![non_articlized_string,
        "hour",
        "hourly",
        "honor",
        "honest",
        "heiress",
        "xmas"
    ];
    if let Some(char) = non_articlized_string.chars().nth(0) {
        if VOWELS.contains(&char) {
            format!("an {non_articlized_string}")
        } else {
            format!("a {non_articlized_string}")
        }
    } else {
        non_articlized_string.to_owned()
    }
}

#[test]
fn articlize_strings_with_initial_lowercase_vowel() {
    assert_eq!("an apple", articlize("apple"));
    assert_eq!("an egg", articlize("egg"));
    assert_eq!("an oddball", articlize("oddball"));
}

#[test]
fn articlize_strings_with_initial_lowercase_consonant() {
    assert_eq!("a banana", articlize("banana"));
    assert_eq!("a holy", articlize("holy"));
    assert_eq!("a yellow", articlize("yellow"));
}

#[test]
fn articlize_strings_with_word_starts_with_word() {
    assert_eq!("a bananalike", articlize("bananalike"));
    assert_eq!("an hourly rate", articlize("hourly rate"));
}

#[test]
fn articlize_strings_special_case_user() {
    assert_eq!("a user", articlize("user"));
}

#[test]
fn articlize_strings_special_case_hour() {
    assert_eq!("an hour", articlize("hour"));
}
