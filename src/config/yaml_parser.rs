use std::{collections::HashMap, fs, path::Path};

/// Represents yaml value
#[derive(Debug)]
pub enum YamlValue {
    String(String),
    Array(Vec<String>),
    Null,
}

impl YamlValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            YamlValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_vec(&self) -> Option<&[String]> {
        match self {
            YamlValue::Array(a) => Some(a.as_slice()),
            _ => None,
        }
    }
}

/// Represent errors for parsing yaml
#[derive(Debug, PartialEq, Eq)]
pub enum YamlParserError {
    FileNotFound,
    FileParseError,
}

/// Parse yaml from file
pub fn parse_yaml_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, YamlValue>, YamlParserError> {
    let content = fs::read_to_string(path).map_err(|_| YamlParserError::FileNotFound)?;
    parse_yaml_from_string(&content)
}

/// Parse yaml from string
pub fn parse_yaml_from_string(
    yaml_content: &str,
) -> Result<HashMap<String, YamlValue>, YamlParserError> {
    let mut result: HashMap<String, YamlValue> = HashMap::new();
    let mut multiline_key: Option<String> = None;
    for line in yaml_content.lines() {
        let line = remove_yaml_comments(line);
        let (key, value) = get_key_value(line);

        match (key, value) {
            // key: value
            (Some(k), Some(v)) => {
                let s_val = YamlValue::String(v.to_string());
                result.entry(k.to_string()).or_insert(s_val);
            }

            // key:
            (Some(k), None) => {
                let a_val = YamlValue::Array(vec![]);
                result.entry(k.to_string()).or_insert(a_val);
                multiline_key = Some(k.to_string());
            }

            // - value
            (None, Some(v)) => {
                if let Some(ref ml_key) = multiline_key
                    && let Some(YamlValue::Array(arr)) = result.get_mut(ml_key)
                {
                    arr.push(v.to_string());
                    continue; // Move to next line
                }
            }

            // empty line or not match
            (None, None) => {}
        }
    }

    if result.is_empty() {
        Err(YamlParserError::FileParseError)
    } else {
        Ok(result)
    }
}

/// Clear value from yaml, trims qoutes, dashes, and spaces
fn clear_yaml_value(value: &str) -> &str {
    value
        .trim()
        .trim_matches('"')
        .trim_start_matches('-')
        .trim()
}

/// Get key and value pair from yaml line
fn get_key_value(value: &str) -> (Option<&str>, Option<&str>) {
    let mut in_quotes = false;
    let bytes = value.as_bytes();

    fn get_value(v: &str) -> Option<&str> {
        let clean = clear_yaml_value(v);
        if clean.is_empty() { None } else { Some(clean) }
    }

    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b'"' => in_quotes = !in_quotes,
            b':' if !in_quotes => {
                return (Some(value[..i].trim()), get_value(&value[i + 1..]));
            }
            _ => {}
        }
    }

    (None, get_value(value))
}

/// Remove comments
fn remove_yaml_comments(value: &str) -> &str {
    let mut in_quotes = false;
    let bytes = value.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        match byte {
            b'"' => in_quotes = !in_quotes,
            b'#' if !in_quotes => return value[..i].trim(),
            _ => {}
        }
    }

    value.trim()
}

#[cfg(test)]
mod yaml_parser_tests {
    use super::*;

    #[test]
    fn it_cant_parse_invalid_yaml() {
        let yaml = "I'm not a valid yaml.";
        let result = parse_yaml_from_string(yaml);
        assert!(result.is_err_and(|x| x == YamlParserError::FileParseError));
    }

    #[test]
    fn it_parses_valid_yaml() {
        let yaml = "
value: testing
foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";

        let result = parse_yaml_from_string(yaml);
        assert!(result.is_ok());

        let map = result.unwrap();

        let key_value = &map["value"];
        if let Some(val) = key_value.as_str() {
            assert_eq!(val, "testing");
        } else {
            panic!("Could not get foo.");
        }

        let foo = &map["foo"];
        if let Some(foo_val) = foo.as_vec() {
            assert_eq!(foo_val.len(), 2);
            assert_eq!(foo_val[0], "list1");
            assert_eq!(foo_val[1], "list2");
        } else {
            panic!("Could not get foo.");
        }

        let foo = &map["bar"];
        if let Some(foo_val) = foo.as_vec() {
            assert_eq!(foo_val.len(), 2);
            assert_eq!(foo_val[0], "1");
            assert_eq!(foo_val[1], "2.0");
        } else {
            panic!("Could not get bar.");
        }
    }

    #[test]
    fn it_clears_yaml_value() {
        let test_cases = vec![
            ("", ""),
            ("  ", ""),
            (" \"\" ", ""),
            (" - test ", "test"),
            ("\"value\"", "value"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(&clear_yaml_value(input), &expected);
        }
    }

    #[test]
    fn it_gets_key_value() {
        let test_cases = vec![
            ("", (None, None)),
            ("test:3", (Some("test"), Some("3"))),
            ("test:", (Some("test"), None)),
            ("  - array value", (None, Some("array value"))),
            ("test:\"test value\"", (Some("test"), Some("test value"))),
        ];

        for (input, expected) in test_cases {
            let (key, value) = get_key_value(input);

            assert_eq!(key, expected.0);
            assert_eq!(value, expected.1);
        }
    }

    #[test]
    fn it_removes_yaml_comments() {
        let test_cases = vec![
            ("", ""),
            ("# this is comment", ""),
            ("#comment", ""),
            ("32 # comment", "32"),
            (
                "\"in quotes # is not a comment\"",
                "\"in quotes # is not a comment\"",
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(&remove_yaml_comments(input), &expected);
        }
    }
}
