use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ========== Request Parameter Types ==========

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SelectWindowParams {
    pub hwnd: u64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindElementsParams {
    pub locator: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ClickElementParams {
    pub context: u64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypeTextParams {
    pub context: u64,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetElementTextParams {
    pub context: u64,
}

// ========== Response Value Types ==========

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SelectWindowValue {
    pub status: String,
    pub vm_id: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FindElementsValue {
    pub elements: Vec<ElementInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ClickElementValue {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypeTextValue {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetElementTextValue {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VersionInfoValue {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListWindowsValue {
    pub windows: Vec<WindowInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WindowInfo {
    pub hwnd: u64,
    pub title: Option<String>,
    pub pid: Option<u32>,
}

// ========== Response Value Enum ==========

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "data")]
pub enum ResponseValue {
    SelectWindow(SelectWindowValue),
    FindElements(FindElementsValue),
    ClickElement(ClickElementValue),
    TypeText(TypeTextValue),
    GetElementText(GetElementTextValue),
    VersionInfo(VersionInfoValue),
    ListWindows(ListWindowsValue),
}

// ========== Response Wrapper ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: u32,
    pub value: Option<ResponseValue>,
    pub error: Option<String>,
}

// ========== RPC Method Enum (for requests) ==========

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "method", content = "params")]
pub enum RpcMethod {
    SelectWindow(SelectWindowParams),
    FindElements(FindElementsParams),
    ClickElement(ClickElementParams),
    TypeText(TypeTextParams),
    GetElementText(GetElementTextParams),
    GetVersionInfo,
    ListWindows,
}

// ========== Request Wrapper ==========

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: u32,
    #[serde(flatten)]
    pub method: RpcMethod,
}

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
