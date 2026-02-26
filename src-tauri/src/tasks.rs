use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub state: String,
    pub created_at: String,
    pub due: Option<String>,
    pub remind_at: Option<String>,
    pub tags: Vec<String>,
    pub source_journal: Option<String>,
    pub linked_to: Option<Uuid>,
    pub title: String,
    pub notes: String,
}

impl Task {
    pub fn new(title: &str, state: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: state.to_string(),
            created_at: Utc::now().to_rfc3339(),
            due: None,
            remind_at: None,
            tags: vec![],
            source_journal: None,
            linked_to: None,
            title: title.to_string(),
            notes: String::new(),
        }
    }

    pub fn as_markdown(&self) -> String {
        format!(
            "---\nid: {}\nstate: {}\ncreatedAt: {}\ndue: {}\nremindAt: {}\ntags: [{}]\nsourceJournal: {}\nlinkedTo: {}\n---\n# {}\n{}\n",
            self.id,
            self.state,
            self.created_at,
            self.due.clone().unwrap_or_default(),
            self.remind_at.clone().unwrap_or_default(),
            self.tags.join(", "),
            self.source_journal.clone().unwrap_or_default(),
            self.linked_to.map(|v| v.to_string()).unwrap_or_default(),
            self.title,
            self.notes
        )
    }
}

pub fn save_task(tasks_root: &Path, task: &Task) -> Result<()> {
    fs::create_dir_all(tasks_root)?;
    let path = tasks_root.join(format!("{}.md", task.id));
    fs::write(path, task.as_markdown())?;
    Ok(())
}
