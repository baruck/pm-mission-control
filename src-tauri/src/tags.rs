use regex::Regex;

pub fn sanitize_label(raw: &str) -> String {
    raw.trim()
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '_' || *c == '-')
        .collect()
}

pub fn extract_tag_pairs(text: &str) -> Vec<String> {
    let re = Regex::new(r"<tag>(.*?)</tag>").expect("valid tag regex");
    let mut tags = Vec::new();
    for cap in re.captures_iter(text) {
        let tag = sanitize_label(&cap[1]);
        if !tag.is_empty() && !tags.contains(&tag) {
            tags.push(tag);
        }
    }
    tags
}

pub fn topic_prefix_tag(line: &str) -> Option<String> {
    let re = Regex::new(r"^\s*-?\s*\[([^\]]+)\]").expect("valid topic regex");
    re.captures(line)
        .map(|cap| sanitize_label(cap.get(1).map(|m| m.as_str()).unwrap_or_default()))
        .filter(|tag| !tag.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_sanitizes_tag_labels() {
        let tags = extract_tag_pairs("hello <tag>Need Refactor!</tag> then <tag>qa-review</tag>");
        assert_eq!(tags, vec!["need_refactor", "qa-review"]);
    }

    #[test]
    fn extracts_topic_prefix() {
        assert_eq!(
            topic_prefix_tag("- [Marketing Manager] something"),
            Some("marketing_manager".to_string())
        );
    }
}
