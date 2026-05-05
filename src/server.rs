use crate::jab_api::JabApi;
use crate::jab_wrapper::jab_service_server::{JabService, JabServiceServer};
use crate::jab_wrapper::*;
use crate::protocol::Locator;
use crate::types::element_info_to_proto;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

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

    pub fn into_service(self) -> JabServiceServer<Self> {
        JabServiceServer::new(self)
    }
}

#[tonic::async_trait]
impl JabService for JabServer {
    async fn select_window(
        &self,
        request: Request<SelectWindowRequest>,
    ) -> Result<Response<SelectWindowResponse>, Status> {
        let req = request.into_inner();
        let api = self.api.lock().unwrap();

        match api.get_context_from_hwnd(req.hwnd) {
            Some((vm_id, ac)) => {
                let mut vm = self.current_vm_id.lock().unwrap();
                let mut root = self.current_root.lock().unwrap();
                *vm = Some(vm_id);
                *root = Some(ac);
                Ok(Response::new(SelectWindowResponse {
                    success: true,
                    error: String::new(),
                }))
            }
            None => Ok(Response::new(SelectWindowResponse {
                success: false,
                error: "Failed to get context from HWND".to_string(),
            })),
        }
    }

    async fn find_elements(
        &self,
        request: Request<FindElementsRequest>,
    ) -> Result<Response<FindElementsResponse>, Status> {
        let req = request.into_inner();
        let locator_proto = req.locator.unwrap_or_default();
        let locator = Locator::from_parts(
            if locator_proto.role.is_empty() { None } else { Some(locator_proto.role) },
            if locator_proto.name.is_empty() { None } else { Some(locator_proto.name) },
            if locator_proto.description.is_empty() { None } else { Some(locator_proto.description) },
            if locator_proto.states.is_empty() { None } else { Some(locator_proto.states.join(",")) },
            locator_proto.strict,
        );

        let (vm_id, root) = {
            let vm = self.current_vm_id.lock().unwrap();
            let root = self.current_root.lock().unwrap();
            match (*vm, *root) {
                (Some(vm), Some(r)) => (vm, r),
                _ => {
                    return Ok(Response::new(FindElementsResponse {
                        elements: Vec::new(),
                        error: "No window selected".to_string(),
                    }))
                }
            }
        };

        let api = self.api.lock().unwrap();
        let mut results: Vec<crate::protocol::ElementInfo> = Vec::new();
        let max_depth = req.max_depth;

        let mut callback = |_depth: i32, ac: u64, info: &crate::bindings::AccessibleContextInfo| {
            let name = wide_to_string(&info.name);
            let role = wide_to_string(&info.role);
            let description = wide_to_string(&info.description);
            let states = wide_to_string(&info.states);

            let role_str = if role.is_empty() { return true; } else { &role };

            if locator.matches(
                role_str,
                if name.is_empty() { None } else { Some(name.as_str()) },
                if description.is_empty() { None } else { Some(description.as_str()) },
                if states.is_empty() { None } else { Some(states.as_str()) },
            ) {
                results.push(crate::protocol::ElementInfo {
                    context: ac,
                    name: if name.is_empty() { None } else { Some(name) },
                    role: if role.is_empty() { None } else { Some(role) },
                    description: if description.is_empty() { None } else { Some(description) },
                    states: if states.is_empty() { None } else { Some(states) },
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

        api.traverse_tree(vm_id, root, 0, max_depth, &mut callback);

        Ok(Response::new(FindElementsResponse {
            elements: results.iter().map(element_info_to_proto).collect(),
            error: String::new(),
        }))
    }

    async fn click_element(
        &self,
        request: Request<ClickElementRequest>,
    ) -> Result<Response<ClickElementResponse>, Status> {
        let req = request.into_inner();
        let api = self.api.lock().unwrap();
        let vm = self.current_vm_id.lock().unwrap();

        if let Some(vm_id) = *vm {
            api.request_focus(vm_id, req.context);
            api.do_action(vm_id, req.context, "click");
            Ok(Response::new(ClickElementResponse {
                success: true,
                error: String::new(),
            }))
        } else {
            Ok(Response::new(ClickElementResponse {
                success: false,
                error: "No window selected".to_string(),
            }))
        }
    }

    async fn type_text(
        &self,
        request: Request<TypeTextRequest>,
    ) -> Result<Response<TypeTextResponse>, Status> {
        let req = request.into_inner();
        let api = self.api.lock().unwrap();
        let vm = self.current_vm_id.lock().unwrap();

        if let Some(vm_id) = *vm {
            api.set_text(vm_id, req.context, &req.text);
            Ok(Response::new(TypeTextResponse {
                success: true,
                error: String::new(),
            }))
        } else {
            Ok(Response::new(TypeTextResponse {
                success: false,
                error: "No window selected".to_string(),
            }))
        }
    }

    async fn get_element_text(
        &self,
        request: Request<GetElementTextRequest>,
    ) -> Result<Response<GetElementTextResponse>, Status> {
        let req = request.into_inner();
        let api = self.api.lock().unwrap();
        let vm = self.current_vm_id.lock().unwrap();

        if let Some(vm_id) = *vm {
            match api.get_text_range(vm_id, req.context, 0, 1024) {
                Some(text) => Ok(Response::new(GetElementTextResponse {
                    text,
                    error: String::new(),
                })),
                None => Ok(Response::new(GetElementTextResponse {
                    text: String::new(),
                    error: "Failed to get text".to_string(),
                })),
            }
        } else {
            Ok(Response::new(GetElementTextResponse {
                text: String::new(),
                error: "No window selected".to_string(),
            }))
        }
    }

    async fn get_version_info(
        &self,
        _request: Request<GetVersionInfoRequest>,
    ) -> Result<Response<GetVersionInfoResponse>, Status> {
        Ok(Response::new(GetVersionInfoResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }))
    }

    async fn list_windows(
        &self,
        _request: Request<ListWindowsRequest>,
    ) -> Result<Response<ListWindowsResponse>, Status> {
        Ok(Response::new(ListWindowsResponse { windows: Vec::new() }))
    }
}

fn wide_to_string(wide: &[u16]) -> String {
    let len = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
    if len == 0 {
        String::new()
    } else {
        String::from_utf16(&wide[..len]).unwrap_or_default()
    }
}
