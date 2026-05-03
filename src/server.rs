use crate::bindings::AccessibleContextInfo;
use crate::jab_api::JabApi;
use crate::protocol::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct JabServer {
    api: Arc<Mutex<JabApi>>,
    current_vm_id: Arc<Mutex<Option<i32>>>,
    current_root: Arc<Mutex<Option<u64>>>,
}

impl JabServer {
    pub fn new() -> Self {
        JabServer {
            api: Arc::new(Mutex::new(JabApi::new())),
            current_vm_id: Arc::new(Mutex::new(None)),
            current_root: Arc::new(Mutex::new(None)),
        }
    }

    pub fn run(&self, port: u16) {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).expect("Failed to bind TCP socket");
        println!("JAB Server listening on {}", addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Client connected");
                    self.handle_client(stream);
                    println!("Client disconnected");
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }
    }

    fn handle_client(&self, stream: TcpStream) {
        let mut reader = BufReader::new(stream);

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let response = self.process_request(line.trim());
                    let resp_str = serde_json::to_string(&response).unwrap_or_else(|_| {
                        serde_json::to_string(&Response {
                            id: 0,
                            value: None,
                            error: Some("Serialization error".to_string()),
                        })
                        .unwrap()
                    });
                    if let Err(e) = writeln!(reader.get_mut(), "{}", resp_str) {
                        eprintln!("Write error: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
    }

    fn process_request(&self, line: &str) -> Response {
        let request: Request = match serde_json::from_str(line) {
            Ok(req) => req,
            Err(e) => {
                return Response {
                    id: 0,
                    value: None,
                    error: Some(format!("Invalid JSON: {}", e)),
                };
            }
        };

        let id = request.id;
        let result = match request.method {
            RpcMethod::SelectWindow(params) => self.select_window(params),
            RpcMethod::FindElements(params) => self.find_elements(params),
            RpcMethod::ClickElement(params) => self.click_element(params),
            RpcMethod::TypeText(params) => self.type_text(params),
            RpcMethod::GetElementText(params) => self.get_element_text(params),
            RpcMethod::GetVersionInfo => self.get_version_info(),
            RpcMethod::ListWindows => self.list_windows(),
        };

        match result {
            Ok(value) => Response {
                id,
                value: Some(value),
                error: None,
            },
            Err(err) => Response {
                id,
                value: None,
                error: Some(err),
            },
        }
    }

    fn list_windows(&self) -> Result<ResponseValue, String> {
        Ok(ResponseValue::ListWindows(ListWindowsValue {
            windows: Vec::new(),
        }))
    }

    fn select_window(&self, params: SelectWindowParams) -> Result<ResponseValue, String> {
        let api = self.api.lock().unwrap();
        match api.get_context_from_hwnd(params.hwnd) {
            Some((vm_id, ac)) => {
                let mut vm = self.current_vm_id.lock().unwrap();
                let mut root = self.current_root.lock().unwrap();
                *vm = Some(vm_id);
                *root = Some(ac);
                Ok(ResponseValue::SelectWindow(SelectWindowValue {
                    status: "ok".to_string(),
                    vm_id,
                }))
            }
            None => Err("Failed to get context from HWND".to_string()),
        }
    }

    fn find_elements(&self, params: FindElementsParams) -> Result<ResponseValue, String> {
        let locator = Locator::parse(&params.locator).ok_or("Invalid locator format")?;

        let (vm_id, root) = {
            let vm = self.current_vm_id.lock().unwrap();
            let root = self.current_root.lock().unwrap();
            match (*vm, *root) {
                (Some(vm), Some(r)) => (vm, r),
                _ => return Err("No window selected".to_string()),
            }
        };

        let api = self.api.lock().unwrap();
        let mut results = Vec::new();

        let mut callback = |_depth: i32, ac: u64, info: &AccessibleContextInfo| {
            let name = wide_to_string(&info.name);
            let role = wide_to_string(&info.role);
            let description = wide_to_string(&info.description);
            let states = wide_to_string(&info.states);

            if let Some(ref role_str) = role
                && locator.matches(
                    role_str,
                    name.as_deref(),
                    description.as_deref(),
                    states.as_deref(),
                )
            {
                results.push(ElementInfo {
                    context: ac,
                    name,
                    role,
                    description,
                    states,
                    x: info.x,
                    y: info.y,
                    width: info.width,
                    height: info.height,
                    index_in_parent: info.indexInParent,
                    children_count: info.childrenCount,
                });
            }

            true
        };

        api.traverse_tree(vm_id, root, 0, 50, &mut callback);

        Ok(ResponseValue::FindElements(FindElementsValue {
            elements: results,
        }))
    }

    fn click_element(&self, params: ClickElementParams) -> Result<ResponseValue, String> {
        let api = self.api.lock().unwrap();
        let vm = self.current_vm_id.lock().unwrap();
        if let Some(vm_id) = *vm {
            api.request_focus(vm_id, params.context);
            api.do_action(vm_id, params.context, "click");
            Ok(ResponseValue::ClickElement(ClickElementValue {
                status: "clicked".to_string(),
            }))
        } else {
            Err("No window selected".to_string())
        }
    }

    fn type_text(&self, params: TypeTextParams) -> Result<ResponseValue, String> {
        let api = self.api.lock().unwrap();
        let vm = self.current_vm_id.lock().unwrap();
        if let Some(vm_id) = *vm {
            api.set_text(vm_id, params.context, &params.text);
            Ok(ResponseValue::TypeText(TypeTextValue {
                status: "text set".to_string(),
            }))
        } else {
            Err("No window selected".to_string())
        }
    }

    fn get_element_text(&self, params: GetElementTextParams) -> Result<ResponseValue, String> {
        let api = self.api.lock().unwrap();
        let vm = self.current_vm_id.lock().unwrap();
        if let Some(vm_id) = *vm {
            match api.get_text_range(vm_id, params.context, 0, 1024) {
                Some(text) => Ok(ResponseValue::GetElementText(GetElementTextValue { text })),
                None => Err("Failed to get text".to_string()),
            }
        } else {
            Err("No window selected".to_string())
        }
    }

    fn get_version_info(&self) -> Result<ResponseValue, String> {
        Ok(ResponseValue::VersionInfo(VersionInfoValue {
            version: "1.0".to_string(),
        }))
    }
}

fn wide_to_string(wide: &[u16]) -> Option<String> {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    if len == 0 {
        None
    } else {
        String::from_utf16(&wide[..len]).ok()
    }
}
