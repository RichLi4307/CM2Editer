use std::collections::HashMap;
use std::sync::OnceLock;

use crate::api::definitions::{NodeDefinition, all_definitions};
use crate::graph::types::NodeType;

/// Returns the cached static slice of all node definitions.
pub fn all_node_definitions() -> &'static Vec<NodeDefinition> {
    static ALL: OnceLock<Vec<NodeDefinition>> = OnceLock::new();
    ALL.get_or_init(all_definitions)
}

/// Returns the static registry mapping [`NodeType`] to its definition.
pub fn registry() -> &'static HashMap<NodeType, NodeDefinition> {
    static REG: OnceLock<HashMap<NodeType, NodeDefinition>> = OnceLock::new();
    REG.get_or_init(|| {
        all_node_definitions()
            .iter()
            .map(|d| (d.node_type, d.clone()))
            .collect()
    })
}

/// Looks up the definition for a given [`NodeType`].
pub fn get_definition(node_type: NodeType) -> Option<&'static NodeDefinition> {
    registry().get(&node_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::definitions::ParamType;
    use crate::graph::types::PortType;

    #[test]
    fn test_registry_contains_all_variants() {
        let all = all_node_definitions();
        let reg = registry();
        assert_eq!(reg.len(), all.len());
        assert_eq!(all.len(), 171);
    }

    #[test]
    fn test_get_definition_goto() {
        let definition = get_definition(NodeType::Goto);
        assert!(definition.is_some());
        let definition = definition.unwrap();
        assert_eq!(definition.node_type, NodeType::Goto);
        assert!(definition.inputs.iter().any(|p| p.port_type == PortType::Flow));
        assert!(definition.outputs.iter().any(|p| p.port_type == PortType::Flow));
    }

    #[test]
    fn test_get_definition_log() {
        let definition = get_definition(NodeType::Log).unwrap();
        assert_eq!(definition.node_type, NodeType::Log);
        assert!(
            definition
                .params
                .iter()
                .any(|p| p.name == "output" && p.param_type == ParamType::String)
        );
    }
}
