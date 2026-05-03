#!/usr/bin/env python3
"""
Simple Python client for the JAB TCP Server.

Usage:
    python client.py <command> [args]

Commands:
    select_window <hwnd>  - Select a Java window by HWND
    find_elements <locator> - Find elements matching locator
    click_element <context> - Click an element by context ID
    type_text <context> <text> - Type text into element
    get_text <context> - Get text from element
    version - Get server version
"""

import socket
import json
import sys


def call_server(method, params, host='127.0.0.1', port=9250):
    """Send a request to the JAB server and return the response."""
    req = {"id": 1, "method": method, "params": params}
    
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall((json.dumps(req) + '\n').encode())
        
        response = b""
        while True:
            chunk = s.recv(4096)
            if not chunk:
                break
            response += chunk
            if b'\n' in chunk:
                break
    
    return json.loads(response.decode().strip())


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    
    command = sys.argv[1]
    
    if command == "select_window":
        if len(sys.argv) < 3:
            print("Usage: select_window <hwnd>")
            sys.exit(1)
        hwnd = int(sys.argv[2])
        result = call_server("select_window", {"hwnd": hwnd})
        print(json.dumps(result, indent=2))
    
    elif command == "find_elements":
        if len(sys.argv) < 3:
            print("Usage: find_elements <locator>")
            sys.exit(1)
        locator = sys.argv[2]
        result = call_server("find_elements", {"locator": locator})
        print(json.dumps(result, indent=2))
    
    elif command == "click_element":
        if len(sys.argv) < 3:
            print("Usage: click_element <context>")
            sys.exit(1)
        context = int(sys.argv[2])
        result = call_server("click_element", {"context": context})
        print(json.dumps(result, indent=2))
    
    elif command == "type_text":
        if len(sys.argv) < 4:
            print("Usage: type_text <context> <text>")
            sys.exit(1)
        context = int(sys.argv[2])
        text = sys.argv[3]
        result = call_server("type_text", {"context": context, "text": text})
        print(json.dumps(result, indent=2))
    
    elif command == "get_text":
        if len(sys.argv) < 3:
            print("Usage: get_text <context>")
            sys.exit(1)
        context = int(sys.argv[2])
        result = call_server("get_element_text", {"context": context})
        print(json.dumps(result, indent=2))
    
    elif command == "version":
        result = call_server("get_version_info", {})
        print(json.dumps(result, indent=2))
    
    else:
        print(f"Unknown command: {command}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
