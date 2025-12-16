pub struct NamedPipe { name: String }
impl NamedPipe { pub fn new(name: &str) -> Self { Self { name: name.to_string() } } }
