use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};

use crate::config::yaml_parser;

#[derive(Debug, Default)]
pub struct Snippet {
    title: String,
    description: String,
    command: String,
    path: PathBuf,
    used: u16,
    last_used: Option<DateTime<Local>>,
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
    let mut in_frontmatter = false;
    let mut in_code_block = false;
    let mut frontmatter_range: (Option<usize>, Option<usize>) = (None, None);
    let mut code_block_range: (Option<usize>, Option<usize>) = (None, None);
    let mut frontmatter: Vec<&str> = vec![];
    let mut code_block: Vec<&str> = vec![];

    fn set_block(index: usize, in_block: &mut bool, range: &mut (Option<usize>, Option<usize>)) {
        if *in_block {
            range.1 = Some(index);
        } else {
            range.0 = Some(index);
        }
        *in_block = !*in_block;
    }

    // find boundaries
    let all_lines = content.lines();
    for (i, line) in all_lines.enumerate() {
        match line {
            "---" => {
                set_block(i, &mut in_frontmatter, &mut frontmatter_range);
            }
            "```" => {
                set_block(i, &mut in_code_block, &mut code_block_range);
            }
            _ if in_frontmatter => frontmatter.push(line),
            _ if in_code_block => code_block.push(line),
            _ => {}
        }
    }

    if code_block.is_empty() {
        return None;
    }

    let mut snippet = Snippet {
        title: path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".to_string()),
        path: PathBuf::from(path),
        command: code_block.join("\n"),
        ..Default::default()
    };

    if let Ok(frontmatter_yaml) = yaml_parser::parse_yaml_from_string(&frontmatter.join("\n")) {
        snippet.description = match frontmatter_yaml.get("description") {
            Some(Some(v)) => v.get_string(),
            _ => None,
        }
        .unwrap_or("".to_string());

        snippet.used = match frontmatter_yaml.get("used") {
            Some(Some(v)) => v.get_string(),
            _ => None,
        }
        .unwrap_or("0".to_string())
        .parse::<u16>()
        .unwrap();

        let last_used = match frontmatter_yaml.get("last_used") {
            Some(Some(v)) => v.get_string(),
            _ => None,
        };

        if let Some(date_str) = last_used {
            snippet.last_used = DateTime::parse_from_rfc3339(&date_str)
                .ok()
                .map(|d| d.with_timezone(&Local));
        }
    }

    Some(snippet)
}
