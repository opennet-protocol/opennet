#[derive(Debug)] pub enum IpcRequest { Resolve(String), Status }
#[derive(Debug)] pub enum IpcResponse { Resolved(Vec<String>), Error(String), Ok }
