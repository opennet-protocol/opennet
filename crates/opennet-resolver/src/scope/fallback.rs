use opennet_core::Scope;
pub fn get_fallback_scope(scope: &Scope) -> Scope {
    match scope {
        Scope::Region(_) => Scope::Global,
        _ => Scope::Global,
    }
}
