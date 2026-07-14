use crate::resolvers::{MatchKind, ResolvedMatch};

pub fn explain(m: &ResolvedMatch) -> String {
    let status = if m.is_active {
        "✅ active"
    } else {
        "shadowed"
    };
    match &m.kind {
        MatchKind::RealBinary => format!("[real binary]   {}", status),
        MatchKind::Symlink { target } => format!("[symlink -> {}]   {}", target.display(), status),
        MatchKind::Shim {manager} => format!("[shim script    {:?}]   {}", manager, status),
        MatchKind::NotIdentified(reason) => format!("[Not Identified: {}]    {}", reason, status),
    }
}
