#[derive(Debug, Clone, Copy)]
pub enum RevocationTrigger { KeyCompromise, ForcedDisclosure, UnauthorizedUsage, Voluntary }
