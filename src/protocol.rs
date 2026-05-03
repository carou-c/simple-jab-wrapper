use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: u32,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: u32,
    pub result: serde_json::Value,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Locator {
    pub role: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub states: Option<String>,
    pub strict: bool,
}

impl Locator {
    pub fn parse(locator_str: &str) -> Option<Self> {
        let mut role = None;
        let mut name = None;
        let mut description = None;
        let mut states = None;
        let mut strict = false;

        for part in locator_str.split(" and ") {
            let part = part.trim();
            if let Some(stripped) = part.strip_prefix("strict:") {
                strict = stripped.trim() == "True";
            } else if let Some(stripped) = part.strip_prefix("role:") {
                role = Some(stripped.trim().to_string());
            } else if let Some(stripped) = part.strip_prefix("name:") {
                name = Some(stripped.trim().to_string());
            } else if let Some(stripped) = part.strip_prefix("description:") {
                description = Some(stripped.trim().to_string());
            } else if let Some(stripped) = part.strip_prefix("states:") {
                states = Some(stripped.trim().to_string());
            }
        }

        if role.is_none() && name.is_none() && description.is_none() && states.is_none() {
            None
        } else {
            Some(Locator {
                role,
                name,
                description,
                states,
                strict,
            })
        }
    }

    pub fn matches(
        &self,
        role: &str,
        name: Option<&str>,
        description: Option<&str>,
        states_str: Option<&str>,
    ) -> bool {
        if let Some(ref r) = self.role {
            if self.strict {
                if r != role { return false; }
            } else if !role.starts_with(r) {
                return false;
            }
        }

        if let Some(ref n) = self.name {
            if let Some(name_val) = name {
                if self.strict {
                    if n != name_val { return false; }
                } else if !name_val.starts_with(n) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(ref d) = self.description {
            if let Some(desc_val) = description {
                if self.strict {
                    if d != desc_val { return false; }
                } else if !desc_val.starts_with(d) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(ref s) = self.states {
            if let Some(states_val) = states_str {
                if self.strict {
                    if s != states_val {
                        return false;
                    }
                } else if !states_val.contains(s) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

// ElementInfo and WindowInfo are now inline in server.rs
