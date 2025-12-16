pub struct UnixSocket { path: String }
impl UnixSocket { pub fn new(path: &str) -> Self { Self { path: path.to_string() } } }
