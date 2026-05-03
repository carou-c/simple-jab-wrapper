#!/bin/bash
# Generate JSON Schema for the typed API
# This script creates a temporary build that outputs the schema

# We need to build for the host (Linux) to run the schema generation
# Temporarily modify build to output schema

cat > src/main_temp.rs << 'MAINEOF'
#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod protocol;
mod jab_api;
mod server;

use protocol::RpcMethod;

fn main() {
    let schema = schemars::schema_for!(RpcMethod);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
MAINEOF

# This won't work easily because of the Windows target in .cargo/config.toml
echo "Schema generation needs to be done on Windows or with a different target"
echo "For now, here's a manual schema based on the Rust types:"

python3 << 'PYEOF'
import json

schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "RpcMethod",
    "oneOf": [
        {
            "type": "object",
            "required": ["method", "params"],
            "properties": {
                "method": {"type": "string", "enum": ["SelectWindow"]},
                "params": {
                    "type": "object",
                    "properties": {
                        "hwnd": {"type": "integer", "description": "Window handle"}
                    },
                    "required": ["hwnd"]
                }
            }
        },
        {
            "type": "object",
            "required": ["method", "params"],
            "properties": {
                "method": {"type": "string", "enum": ["FindElements"]},
                "params": {
                    "type": "object",
                    "properties": {
                        "locator": {"type": "string", "description": "Locator string (e.g., 'role:button and name:OK')"}
                    },
                    "required": ["locator"]
                }
            }
        },
        {
            "type": "object",
            "required": ["method", "params"],
            "properties": {
                "method": {"type": "string", "enum": ["ClickElement"]},
                "params": {
                    "type": "object",
                    "properties": {
                        "context": {"type": "integer", "description": "Element context ID"}
                    },
                    "required": ["context"]
                }
            }
        },
        {
            "type": "object",
            "required": ["method", "params"],
            "properties": {
                "method": {"type": "string", "enum": ["TypeText"]},
                "params": {
                    "type": "object",
                    "properties": {
                        "context": {"type": "integer", "description": "Element context ID"},
                        "text": {"type": "string", "description": "Text to type"}
                    },
                    "required": ["context", "text"]
                }
            }
        },
        {
            "type": "object",
            "required": ["method", "params"],
            "properties": {
                "method": {"type": "string", "enum": ["GetElementText"]},
                "params": {
                    "type": "object",
                    "properties": {
                        "context": {"type": "integer", "description": "Element context ID"}
                    },
                    "required": ["context"]
                }
            }
        },
        {
            "type": "object",
            "required": ["method"],
            "properties": {
                "method": {"type": "string", "enum": ["GetVersionInfo"]}
            }
        },
        {
            "type": "object",
            "required": ["method"],
            "properties": {
                "method": {"type": "string", "enum": ["ListWindows"]}
            }
        }
    ]
}

print(json.dumps(schema, indent=2))
PYEOF
