use std::collections::HashMap;

pub struct RuleManager;

impl RuleManager {
    pub fn get_default_rules() -> HashMap<&'static str, &'static str> {
        let mut rules = HashMap::new();
        rules.insert("jpg", "Images");
        rules.insert("png", "Images");
        rules.insert("mp4", "Videos");
        rules.insert("mov", "Videos");
        rules.insert("txt", "Documents");
        rules.insert("pdf", "Documents");
        rules.insert("docx", "Documents");
        rules.insert("mp3", "Audios");
        rules.insert("wav", "Audios");
        rules
    }
}
