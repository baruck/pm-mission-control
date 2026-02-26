use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSettings {
    pub treat_topic_prefix_as_tags: bool,
    pub auto_extract_questions: bool,
}

impl Default for BehaviorSettings {
    fn default() -> Self {
        Self {
            treat_topic_prefix_as_tags: true,
            auto_extract_questions: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlassianSettings {
    pub site_url: String,
    pub email: String,
}
