use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bucket {
    pub heading: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub title: String,
    pub body_lines: Vec<String>,
    pub raw: String,
    pub jira_keys: Vec<String>,
    pub questions: Vec<String>,
}

fn is_bucket_heading(line: &str, next_lines: &[&str]) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with("```") || trimmed.is_empty() {
        return false;
    }

    if trimmed.ends_with(':') {
        return true;
    }

    if !trimmed.contains(':') {
        for probe in next_lines.iter().take(2) {
            if probe.trim_start().starts_with("- ") {
                return true;
            }
            if !probe.trim().is_empty() {
                break;
            }
        }
    }

    false
}

pub fn detect_jira_keys(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b([A-Z][A-Z0-9]+-\d+)\b").expect("valid jira regex");
    let mut found = Vec::new();
    for cap in re.captures_iter(text) {
        let key = cap[1].to_string();
        if !found.contains(&key) {
            found.push(key);
        }
    }
    found
}

pub fn parse_buckets(input: &str, auto_extract_questions: bool) -> Vec<Bucket> {
    let lines: Vec<&str> = input.lines().collect();
    let mut buckets: Vec<(String, Vec<&str>)> = Vec::new();
    let mut current_heading = "Inbox".to_string();
    let mut current_lines = Vec::new();

    for i in 0..lines.len() {
        let line = lines[i];
        let next = lines.get(i + 1).copied().unwrap_or("");
        let next2 = lines.get(i + 2).copied().unwrap_or("");
        if is_bucket_heading(line, &[next, next2]) {
            if !current_lines.is_empty() {
                buckets.push((current_heading.clone(), current_lines));
                current_lines = Vec::new();
            }
            current_heading = line.trim().trim_end_matches(':').to_string();
        } else {
            current_lines.push(line);
        }
    }

    if !current_lines.is_empty() {
        buckets.push((current_heading, current_lines));
    }

    buckets
        .into_iter()
        .map(|(heading, bucket_lines)| Bucket {
            heading,
            blocks: parse_blocks(&bucket_lines.join("\n"), auto_extract_questions),
        })
        .collect()
}

pub fn parse_blocks(input: &str, auto_extract_questions: bool) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.starts_with("- ") {
            if !current.is_empty() {
                blocks.push(materialize_block(&current, auto_extract_questions));
                current.clear();
            }
            current.push(line.to_string());
        } else if !current.is_empty() {
            current.push(line.to_string());
        }
    }

    if !current.is_empty() {
        blocks.push(materialize_block(&current, auto_extract_questions));
    }

    blocks
}

fn materialize_block(lines: &[String], auto_extract_questions: bool) -> Block {
    let title = lines
        .first()
        .map(|line| line.trim_start_matches("- ").trim().to_string())
        .unwrap_or_default();

    let body_lines: Vec<String> = lines.iter().skip(1).map(|s| s.to_string()).collect();
    let raw = lines.join("\n");
    let jira_keys = detect_jira_keys(&raw);
    let questions = if auto_extract_questions {
        body_lines
            .iter()
            .filter_map(|line| {
                let trimmed = line.trim();
                trimmed
                    .strip_prefix("(?)")
                    .map(|value| value.trim().to_string())
            })
            .collect()
    } else {
        Vec::new()
    };

    Block {
        title,
        body_lines,
        raw,
        jira_keys,
        questions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bucket_headings_with_colon_and_without_colon() {
        let input = "Tasks for me:\n- One\n\nImprovements\n- Two\n";
        let buckets = parse_buckets(input, true);
        assert_eq!(buckets.len(), 2);
        assert_eq!(buckets[0].heading, "Tasks for me");
        assert_eq!(buckets[1].heading, "Improvements");
    }

    #[test]
    fn parses_top_level_bullets_as_atomic_blocks() {
        let input = "- M2M refactor\n    - Add flag\n    (?) Is downpayment locked?\n- Another item";
        let blocks = parse_blocks(input, true);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].title, "M2M refactor");
        assert_eq!(blocks[0].questions, vec!["Is downpayment locked?"]);
    }

    #[test]
    fn detects_jira_keys_from_title_and_body() {
        let keys = detect_jira_keys("Do this (PECR-623) and maybe PECR-537");
        assert_eq!(keys, vec!["PECR-623", "PECR-537"]);
    }
}
