use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};

use crate::config::yaml_parser;

#[derive(Debug, Default)]
pub struct Snippet {
    pub title: String,
    pub description: String,
    pub command: String,
    pub path: PathBuf,
    pub used: u16,
    pub last_used: Option<DateTime<Local>>,
}

pub fn load_snippets(data_dir: &str) -> io::Result<Vec<Snippet>> {
    let md_extension = OsStr::new("md");
    let mut result: Vec<Snippet> = vec![];

    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension() == Some(md_extension) {
            let content = fs::read_to_string(&path)?;
            if let Some(snippet) = extract_snippet(&path, &content) {
                result.push(snippet);
            }
        }
    }

    Ok(result)
}

fn extract_snippet(path: &Path, content: &str) -> Option<Snippet> {
    let mut start_offset = 0;
    let mut lines = content.split_inclusive('\n').map(|line| {
        let start = start_offset;
        start_offset += line.len();
        (start, line.trim_end_matches(['\r', '\n']))
    });

    let mut frontmatter_raw = None;
    let mut code_raw = None;
    'outer: while let Some((idx, val)) = lines.next() {
        if val == "---" {
            let start_pos = idx + val.len() + 1;

            for (fm_idx, fm_val) in lines.by_ref() {
                if fm_val == "---" {
                    frontmatter_raw = Some(content[start_pos..fm_idx].trim());
                    break;
                }
            }
        } else if val.starts_with("```") {
            let start_pos = idx + val.len() + 1;

            for (c_idx, c_val) in lines.by_ref() {
                if c_val.starts_with("```") {
                    code_raw = Some(content[start_pos..c_idx].trim());
                    break 'outer;
                }
            }

            code_raw = Some(content[start_pos..].trim());
        }
    }

    let code = code_raw?;
    let mut snippet = Snippet {
        title: path.file_stem()?.to_string_lossy().into_owned(),
        path: path.to_path_buf(),
        command: code.to_string(),
        ..Default::default()
    };

    if let Some(fm_text) = frontmatter_raw
        && let Ok(yaml) = yaml_parser::parse_yaml_from_string(fm_text)
    {
        let get_str = |k| yaml.get(k).and_then(|v| v.as_str());

        snippet.description = get_str("description").unwrap_or("").to_string();
        snippet.used = get_str("used").and_then(|s| s.parse().ok()).unwrap_or(0);
        snippet.last_used = get_str("last_used")
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|d| d.with_timezone(&Local));
    }

    Some(snippet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_parses_valid_md() {
        let content = "---
description: Test snippet
used: 5
---
```
echo 'hello world'
```
";

        let path = Path::new("test-file.sh");
        let result = extract_snippet(path, content).unwrap();

        assert_eq!(result.title, "test-file");
        assert_eq!(result.command, "echo 'hello world'");
        assert_eq!(result.description, "Test snippet");
        assert_eq!(result.used, 5);
    }

    #[test]
    fn it_parses_valid_md_no_frontmatter() {
        let content = "```bash
echo 'hello world'
```
";

        let path = Path::new("test-file.sh");
        let result = extract_snippet(path, content).unwrap();

        assert_eq!(result.title, "test-file");
        assert_eq!(result.command, "echo 'hello world'");
        assert_eq!(result.description, "");
        assert_eq!(result.used, 0);
    }

    #[test]
    fn it_parses_valid_md_no_frontmatter_with_no_trailing_line() {
        let content = "```bash
echo 'hello world'
```";

        let path = Path::new("test-file.sh");
        let result = extract_snippet(path, content).unwrap();

        assert_eq!(result.title, "test-file");
        assert_eq!(result.command, "echo 'hello world'");
        assert_eq!(result.description, "");
        assert_eq!(result.used, 0);
        assert!(!result.command.contains("```"));
    }

    #[test]
    fn it_parses_first_code_block() {
        let content = "```bash
echo 'hello world'
```

```bash
echo 'hello world 2'
```

";

        let path = Path::new("test-file.sh");
        let result = extract_snippet(path, content).unwrap();

        assert_eq!(result.title, "test-file");
        assert_eq!(result.command, "echo 'hello world'");
        assert_eq!(result.description, "");
        assert_eq!(result.used, 0);
    }

    #[test]
    fn it_ignores_md_without_code_block() {
        let content = "---
description: Test
---
this file doesn't have code blocks
";

        let path = Path::new("test-file.sh");
        assert!(extract_snippet(path, content).is_none());
    }
}
