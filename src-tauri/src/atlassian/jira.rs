use chrono::{DateTime, Utc};
use serde_json::{json, Value};

pub fn build_create_issue_payload(
    project_key: &str,
    issue_type: &str,
    summary: &str,
    body: &str,
    labels: &[String],
) -> Value {
    json!({
      "fields": {
        "project": { "key": project_key },
        "issuetype": { "name": issue_type },
        "summary": summary,
        "description": adf_paragraphs(body),
        "labels": labels
      }
    })
}

pub fn build_append_description_adf(existing: &Value, append_text: &str, now: DateTime<Utc>) -> Value {
    let mut doc = existing.clone();
    let content = doc
        .get_mut("content")
        .and_then(|v| v.as_array_mut())
        .expect("existing adf must have content array");
    content.push(json!({"type":"rule"}));
    content.push(json!({
      "type":"paragraph",
      "content":[{"type":"text","text": format!("Update ({})", now.format("%Y-%m-%d"))}]
    }));
    for line in append_text.lines() {
        content.push(json!({"type":"paragraph","content":[{"type":"text","text":line}]}));
    }
    doc
}

fn adf_paragraphs(body: &str) -> Value {
    let content: Vec<Value> = body
        .lines()
        .map(|line| json!({"type":"paragraph","content":[{"type":"text","text":line}]}))
        .collect();
    json!({"type":"doc","version":1,"content":content})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_payload_contains_labels_and_adf() {
        let payload = build_create_issue_payload(
            "PECR",
            "Story",
            "My title",
            "line1\nline2",
            &["need_refactor".into(), "foo".into()],
        );
        assert_eq!(payload["fields"]["project"]["key"], "PECR");
        assert_eq!(payload["fields"]["labels"][0], "need_refactor");
        assert_eq!(payload["fields"]["description"]["type"], "doc");
    }

    #[test]
    fn append_builder_adds_update_section() {
        let existing = json!({"type":"doc","version":1,"content":[]});
        let updated = build_append_description_adf(&existing, "hello", Utc::now());
        let serialized = serde_json::to_string(&updated).unwrap();
        assert!(serialized.contains("Update ("));
        assert!(serialized.contains("hello"));
    }
}
