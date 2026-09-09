use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallReason {
    Explicit,
    Dependency,
    Unknown,
}

impl fmt::Display for InstallReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallReason::Explicit => write!(f, "Explicitly installed"),
            InstallReason::Dependency => write!(f, "Installed as dependency"),
            InstallReason::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSource {
    Official,
    AUR,
}

impl fmt::Display for PackageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PackageSource::Official => write!(f, "Official"),
            PackageSource::AUR => write!(f, "AUR"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub architecture: String,
    pub url: String,
    pub licenses: Vec<String>,
    pub groups: Vec<String>,
    pub provides: Vec<String>,
    pub depends: Vec<String>,
    pub optional_deps: Vec<String>,
    pub required_by: Vec<String>,
    pub optional_for: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub installed_size: String,       // raw string like "5.86 MiB"
    pub installed_size_bytes: u64,    // parsed to bytes for sorting
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
    pub install_reason: InstallReason,
    pub validated_by: String,
    pub source: PackageSource,
    pub is_orphan: bool,
}

#[derive(Debug, Clone)]
pub struct FileGroup {
    pub category: String,
    pub base_path: String,
    pub files: Vec<String>,
}

/// Parse a size string like "5.86 MiB" or "123.45 KiB" to bytes
pub fn parse_size_to_bytes(size_str: &str) -> u64 {
    let parts: Vec<&str> = size_str.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return 0;
    }
    let value: f64 = parts[0].parse().unwrap_or(0.0);
    let multiplier: f64 = match parts[1] {
        "B" => 1.0,
        "KiB" => 1024.0,
        "MiB" => 1024.0 * 1024.0,
        "GiB" => 1024.0 * 1024.0 * 1024.0,
        "TiB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => 0.0,
    };
    (value * multiplier) as u64
}

/// Format bytes to a human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * 1024;
    const GIB: u64 = 1024 * 1024 * 1024;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Group a list of file paths by their install location category
pub fn group_files(files: Vec<String>) -> Vec<FileGroup> {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<(u8, String, String), Vec<String>> = BTreeMap::new();

    for file in files {
        let trimmed = file.trim().to_string();
        if trimmed.is_empty() || trimmed.ends_with('/') {
            continue; // skip directories
        }
        let (order, category, base_path) = categorize_path(&trimmed);
        groups
            .entry((order, category, base_path))
            .or_default()
            .push(trimmed);
    }

    groups
        .into_iter()
        .map(|((_, category, base_path), mut files)| {
            files.sort();
            FileGroup {
                category,
                base_path,
                files,
            }
        })
        .collect()
}

fn categorize_path(path: &str) -> (u8, String, String) {
    if path.starts_with("/usr/bin/") || path.starts_with("/usr/sbin/") || path.starts_with("/bin/") || path.starts_with("/sbin/") {
        (0, "Binaries".to_string(), "/usr/bin/".to_string())
    } else if path.starts_with("/usr/lib/") || path.starts_with("/lib/") {
        (1, "Libraries".to_string(), "/usr/lib/".to_string())
    } else if path.starts_with("/usr/include/") {
        (2, "Headers".to_string(), "/usr/include/".to_string())
    } else if path.starts_with("/usr/share/man/") {
        (3, "Man Pages".to_string(), "/usr/share/man/".to_string())
    } else if path.starts_with("/usr/share/doc/") {
        (4, "Documentation".to_string(), "/usr/share/doc/".to_string())
    } else if path.starts_with("/usr/share/licenses/") {
        (5, "Licenses".to_string(), "/usr/share/licenses/".to_string())
    } else if path.starts_with("/usr/share/") {
        (6, "Data & Resources".to_string(), "/usr/share/".to_string())
    } else if path.starts_with("/etc/") {
        (7, "Configuration".to_string(), "/etc/".to_string())
    } else if path.starts_with("/var/") {
        (8, "Variable Data".to_string(), "/var/".to_string())
    } else if path.starts_with("/opt/") {
        (9, "Optional".to_string(), "/opt/".to_string())
    } else {
        (10, "Other".to_string(), "/".to_string())
    }
}
