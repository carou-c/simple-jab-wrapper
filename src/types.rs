pub fn element_info_to_proto(info: &crate::protocol::ElementInfo) -> crate::jab_wrapper::ElementInfo {
    crate::jab_wrapper::ElementInfo {
        context: info.context,
        name: info.name.clone().unwrap_or_default(),
        role: info.role.clone().unwrap_or_default(),
        description: info.description.clone().unwrap_or_default(),
        states: info.states.clone().unwrap_or_default(),
        x: info.x,
        y: info.y,
        width: info.width,
        height: info.height,
        index_in_parent: info.index_in_parent,
        children_count: info.children_count,
    }
}
