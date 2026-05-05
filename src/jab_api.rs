use crate::bindings::*;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct JabApi {
    initialized: bool,
}

impl JabApi {
    pub fn new() -> Self {
        unsafe {
            let ok = initializeAccessBridge();
            if ok == 0 {
                eprintln!("Warning: initializeAccessBridge returned {}", ok);
            }
            JabApi { initialized: ok != 0 }
        }
    }

    pub fn shutdown(&self) {
        unsafe {
            shutdownAccessBridge();
        }
    }

    pub fn get_context_from_hwnd(&self, hwnd: u64) -> Option<(i32, u64)> {
        unsafe {
            let mut vm_id: i32 = 0;
            let mut ac: JOBJECT64 = 0;
            let ok = GetAccessibleContextFromHWND(hwnd as HWND, &mut vm_id, &mut ac);
            if ok != 0 {
                Some((vm_id, ac as u64))
            } else {
                None
            }
        }
    }

    pub fn get_context_info(&self, vm_id: i32, ac: u64) -> Option<AccessibleContextInfo> {
        unsafe {
            let mut info: AccessibleContextInfo = std::mem::zeroed();
            let ok = GetAccessibleContextInfo(vm_id, ac as JOBJECT64, &mut info);
            if ok != 0 {
                Some(info)
            } else {
                None
            }
        }
    }

    pub fn get_child(&self, vm_id: i32, ac: u64, index: i32) -> Option<u64> {
        unsafe {
            let child = GetAccessibleChildFromContext(vm_id, ac as JOBJECT64, index);
            if child != 0 {
                Some(child as u64)
            } else {
                None
            }
        }
    }

    pub fn request_focus(&self, vm_id: i32, ac: u64) -> bool {
        unsafe {
            requestFocus(vm_id, ac as JOBJECT64) != 0
        }
    }

    pub fn set_text(&self, vm_id: i32, ac: u64, text: &str) -> bool {
        unsafe {
            let wide: Vec<u16> = OsStr::new(text).encode_wide().chain(std::iter::once(0)).collect();
            setTextContents(vm_id, ac as JOBJECT64, wide.as_ptr()) != 0
        }
    }

    pub fn get_text_range(&self, vm_id: i32, ac: u64, start: i32, end: i32) -> Option<String> {
        unsafe {
            let mut buf: [u16; 1024] = std::mem::zeroed();
            let ok = GetAccessibleTextRange(vm_id, ac as JOBJECT64, start, end, buf.as_mut_ptr(), 1024);
            if ok != 0 {
                let len = buf.iter().position(|&c| c == 0).unwrap_or(1024);
                String::from_utf16(&buf[..len]).ok()
            } else {
                None
            }
        }
    }

    pub fn do_action(&self, vm_id: i32, ac: u64, action_name: &str) -> bool {
        unsafe {
            let mut actions: AccessibleActionsToDo = std::mem::zeroed();
            let name_wide: Vec<u16> = OsStr::new(action_name).encode_wide().chain(std::iter::once(0)).collect();
            actions.actions[0].name[..name_wide.len()].copy_from_slice(&name_wide);

            let mut failure: i32 = 0;
            doAccessibleActions(vm_id, ac as JOBJECT64, &mut actions, &mut failure) != 0
        }
    }

    pub fn traverse_tree<F>(&self, vm_id: i32, ac: u64, depth: i32, max_depth: i32, callback: &mut F)
    where
        F: FnMut(i32, u64, &AccessibleContextInfo) -> bool,
    {
        if depth > max_depth {
            return;
        }

        if let Some(info) = self.get_context_info(vm_id, ac) {
            if !callback(depth, ac, &info) {
                return;
            }

            let children_count = info.childrenCount;
            for i in 0..children_count {
                if let Some(child) = self.get_child(vm_id, ac, i) {
                    self.traverse_tree(vm_id, child, depth + 1, max_depth, callback);
                }
            }
        }
    }
}

impl Drop for JabApi {
    fn drop(&mut self) {
        if self.initialized {
            self.shutdown();
        }
    }
}
