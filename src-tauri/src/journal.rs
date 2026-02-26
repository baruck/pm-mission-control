use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarkerBlock {
    pub id: Uuid,
    pub bucket: String,
    pub kind: String,
    pub status: String,
    pub remote: Option<String>,
    pub body: String,
}

pub fn parse_marker_blocks(markdown: &str) -> Vec<MarkerBlock> {
    let re = Regex::new(
        r"(?s)<!-- MC_BLOCK id=([0-9a-fA-F-]+) bucket=([^\s]+) kind=([^\s]+) status=([^\s]+) remote=([^\s>]*) -->\n(.*?)\n<!-- /MC_BLOCK -->",
    )
    .expect("valid marker regex");

    re.captures_iter(markdown)
        .filter_map(|cap| {
            Some(MarkerBlock {
                id: Uuid::parse_str(cap.get(1)?.as_str()).ok()?,
                bucket: cap.get(2)?.as_str().replace('_', " "),
                kind: cap.get(3)?.as_str().to_string(),
                status: cap.get(4)?.as_str().to_string(),
                remote: match cap.get(5)?.as_str() {
                    "" | "none" => None,
                    value => Some(value.to_string()),
                },
                body: cap.get(6)?.as_str().to_string(),
            })
        })
        .collect()
}

pub fn render_marker_block(block: &MarkerBlock) -> String {
    format!(
        "<!-- MC_BLOCK id={} bucket={} kind={} status={} remote={} -->\n{}\n<!-- /MC_BLOCK -->",
        block.id,
        block.bucket.replace(' ', "_"),
        block.kind,
        block.status,
        block.remote.clone().unwrap_or_else(|| "none".into()),
        block.body
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_roundtrip_works() {
        let block = MarkerBlock {
            id: Uuid::new_v4(),
            bucket: "Tasks for me".to_string(),
            kind: "jira".to_string(),
            status: "created".to_string(),
            remote: Some("PECR-123".to_string()),
            body: "- a card".to_string(),
        };

        let md = render_marker_block(&block);
        let parsed = parse_marker_blocks(&md);
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].bucket, "Tasks for me");
        assert_eq!(parsed[0].remote.as_deref(), Some("PECR-123"));
    }
}
