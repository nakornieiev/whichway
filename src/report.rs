use crate::doctor::DoctorReport;
use crate::resolvers::{ManagerInfo, MatchKind, ResolvedMatch};
use owo_colors::{OwoColorize, Stream::Stdout};

pub fn explain(m: &ResolvedMatch) -> String {
    let status = if m.is_active {
        "✅ active"
            .if_supports_color(Stdout, |text| text.green())
            .to_string()
    } else {
        "shadowed"
            .if_supports_color(Stdout, |text| text.bright_black())
            .to_string()
    };

    let tag = match &m.kind {
        MatchKind::RealBinary => "real binary".to_string(),
        MatchKind::Symlink { target } => format!("symlink -> {}", target.display()),
        MatchKind::Shim => "shim".to_string(),
        MatchKind::NotIdentified(reason) => {
            return format!(
                "{}   {}",
                format!("[Not Identified: {}]", reason)
                    .if_supports_color(Stdout, |text| text.red()),
                status
            );
        }
    };

    let manager = match &m.manager {
        Some(info) => match info {
            ManagerInfo::Nvm => "nvm ",
            ManagerInfo::Pyenv => "pyenv ",
            ManagerInfo::Asdf => "asdf ",
        },
        None => "",
    };

    let tag = format!("[{}{}]", manager, tag);

    let tag = tag.if_supports_color(Stdout, |text| text.dimmed());

    format!("{}   {}", tag, status)
}

pub fn doctor_explain(report: &DoctorReport) -> String {
    let mut lines: Vec<String> = Vec::new();

    if !report.duplicates.is_empty() {
        for (title, el) in report.duplicates.iter() {
            lines.push(
                format!("⚠️ Found duplicates for: {}", title)
                    .if_supports_color(Stdout, |text| text.yellow())
                    .to_string(),
            );
            el.iter()
                .for_each(|el| lines.push(format!("\t{}", el.display())))
        }
    } else {
        lines.push(
            "✅ Found no duplicates"
                .if_supports_color(Stdout, |text| text.green())
                .to_string(),
        );
    }

    if !report.broken_symlinks.is_empty() {
        lines.push(
            "⚠️ Found broken symlinks:"
                .if_supports_color(Stdout, |text| text.yellow())
                .to_string(),
        );
        report
            .broken_symlinks
            .iter()
            .for_each(|el| lines.push(format!("\t{}", el.display())))
    } else {
        lines.push(
            "✅ Found no broken symlinks"
                .if_supports_color(Stdout, |text| text.green())
                .to_string(),
        );
    }

    if !report.orphan_shims.is_empty() {
        lines.push(
            "⚠️ Found orphan shims:"
                .if_supports_color(Stdout, |text| text.yellow())
                .to_string(),
        );
        report
            .orphan_shims
            .iter()
            .for_each(|el| lines.push(format!("\t{}", el.display())))
    } else {
        lines.push(
            "✅ Found no orphan shims"
                .if_supports_color(Stdout, |text| text.green())
                .to_string(),
        );
    }

    lines.join("\n")
}
