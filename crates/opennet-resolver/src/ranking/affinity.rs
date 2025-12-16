use opennet_core::Scope;
pub fn calculate_affinity(node_scope: &Scope, request_scope: &Scope) -> f64 {
    if node_scope.matches(request_scope) { 1.0 } else { 0.5 }
}
