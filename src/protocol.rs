// ========== Locator ==========

#[derive(Debug, Clone)]
pub struct Locator {
    pub role: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub states: Option<String>,
    pub strict: bool,
}

impl Locator {
    pub fn from_parts(role: Option<String>, name: Option<String>, description: Option<String>, states: Option<String>, strict: bool) -> Self {
        Locator {
            role,
            name,
            description,
            states,
            strict,
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
                if r != role {
                    return false;
                }
            } else if !role.starts_with(r) {
                return false;
            }
        }

        if let Some(ref n) = self.name {
            if let Some(name_val) = name {
                if self.strict {
                    if n != name_val {
                        return false;
                    }
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
                    if d != desc_val {
                        return false;
                    }
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

// ========== Element Info (internal representation) ==========

#[derive(Debug, Clone)]
pub struct ElementInfo {
    pub context: u64,
    pub name: Option<String>,
    pub role: Option<String>,
    pub description: Option<String>,
    pub states: Option<String>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub index_in_parent: i32,
    pub children_count: i32,
}
