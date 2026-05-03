# Simple JAB Wrapper

A simple TCP server that wraps a few Java Access Bridge (JAB) functions.

## Why tho?

This project exists to solve a very specific (and distasteful) problem I've ran into:

- Needed to automate a 32-bit legacy Java application
- Necessarely from a 64-bit Python runtime
- JAB DLLs are architecture-specific (32-bit vs 64-bit)

Solution: A very simple 32-bit Rust server wraps JAB calls and exposes them via TCP socket to any 64-bit client.

## Architecture

```text
[32-bit Java App] <--JAB--> [32-bit Rust Server] <--TCP--> [64-bit Python Client]
```

## Running the Server

```cmd
simple-jab-wrapper.exe
# Server listens on 127.0.0.1:9250 by default
# Override port: set JAB_SERVER_PORT=9300
```

## TCP Protocol

JSON-RPC style protocol, line-delimited (each message ends with `\n`).

### Request Format

```json
{
  "id": 1,
  "method": "method_name",
  "params": { ... }
}
```

### Response Format

```json
{
  "id": 1,
  "result": { ... },
  "error": null
}
```

## Available Methods

### select_window

Select a Java window by HWND.

```json
{ "id": 1, "method": "select_window", "params": { "hwnd": 123456 } }
```

### find_elements

Find elements matching a locator string.

```json
{
  "id": 2,
  "method": "find_elements",
  "params": { "locator": "role:button and name:OK" }
}
```

Locator format: `role:<value> and name:<value> and description:<value> and states:<value>`
Use `strict:True` for exact matching.

### click_element

Click an element by its context ID.

```json
{ "id": 3, "method": "click_element", "params": { "context": 12345 } }
```

### type_text

Type text into an element.

```json
{
  "id": 4,
  "method": "type_text",
  "params": { "context": 12345, "text": "hello" }
}
```

### get_element_text

Get text from an element.

```json
{ "id": 5, "method": "get_element_text", "params": { "context": 12345 } }
```

### get_version_info

Get server version.

```json
{ "id": 6, "method": "get_version_info", "params": {} }
```

## Python Client Example

```python
import socket
import json

def call_server(method, params, host='127.0.0.1', port=9250):
    req = {"id": 1, "method": method, "params": params}
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall((json.dumps(req) + '\n').encode())
        response = s.recv(4096)
    return json.loads(response.decode().strip())

# Select window
result = call_server("select_window", {"hwnd": 123456})

# Find buttons
elements = call_server("find_elements", {"locator": "role:button"})

# Click first button
if elements["result"]:
    ctx = elements["result"][0]["context"]
    call_server("click_element", {"context": ctx})
```

Or use the provided `client.py`:

```bash
python client.py select_window 123456
python client.py find_elements "role:button and name:OK"
python client.py click_element 12345
```

