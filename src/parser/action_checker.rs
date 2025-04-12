//! Contains logic for checking action conflicts.

use crate::core_types::{Action, ActionType};
use std::collections::HashMap;

/// Checks the final sorted list of actions for potential conflicts on the same path.
pub(crate) fn check_action_conflicts(final_actions: &[Action]) {
    let mut paths_seen: HashMap<String, (ActionType, usize)> = HashMap::new();
    println!("Checking action sequence...");
    for (i, action) in final_actions.iter().enumerate() {
        let path = &action.path;
        let current_act_type = action.action_type.clone();
        if let Some((prev_act_type, prev_idx)) = paths_seen.get(path) {
            println!(
                "  Info: Action '{:?}' for path '{}' (item {}) follows action '{:?}' (item {}). Ensure sequence is intended.",
                current_act_type, path, i + 1, prev_act_type, prev_idx + 1
            );
        }
        paths_seen.insert(path.clone(), (current_act_type, i));
    }
}
