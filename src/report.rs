use crate::resolvers::{MatchKind, ResolvedMatch};
use owo_colors::{OwoColorize, Stream::Stdout};

pub fn explain(m: &ResolvedMatch) -> String {
    let status = if m.is_active {
        "✅ active".if_supports_color(Stdout, |text| text.green()).to_string()
    } else {
        "shadowed".if_supports_color(Stdout, |text| text.bright_black()).to_string()
    };
    let tag = match &m.kind {
        MatchKind::RealBinary => "[real binary]".to_string(),
        MatchKind::Symlink { target } => format!("[symlink -> {}]", target.display()),
        MatchKind::Shim {manager} => format!("[{:?} shim]", manager),
        MatchKind::NotIdentified(reason) => {
            return format!(
                "{}   {}",
                format!("[Not Identified: {}]", reason)
                    .if_supports_color(Stdout, |text| text.red()),
                status
            );
        },
    };

    let tag = tag.if_supports_color(Stdout, |text| text.dimmed());

    format!("{}    {}", tag, status)
}
