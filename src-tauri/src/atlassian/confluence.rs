use serde_json::{json, Value};

pub fn build_create_page_payload(space_id: &str, title: &str, body_storage: &str) -> Value {
    json!({
      "spaceId": space_id,
      "status": "current",
      "title": title,
      "body": {
        "representation": "storage",
        "value": body_storage
      }
    })
}

pub fn append_rough_notes_section(existing_storage: &str, addition: &str) -> String {
    let heading = "<h2>Rough notes (needs refactor)</h2>";
    if existing_storage.contains(heading) {
        format!("{}<p>{}</p>", existing_storage, html_escape(addition))
    } else {
        format!("{}{}<p>{}</p>", existing_storage, heading, html_escape(addition))
    }
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_page_payload_has_storage_body() {
        let payload = build_create_page_payload("123", "Title", "<p>Body</p>");
        assert_eq!(payload["spaceId"], "123");
        assert_eq!(payload["body"]["representation"], "storage");
    }

    #[test]
    fn append_adds_heading_if_missing() {
        let out = append_rough_notes_section("<p>x</p>", "new");
        assert!(out.contains("Rough notes (needs refactor)"));
        assert!(out.ends_with("<p>new</p>"));
    }
}
