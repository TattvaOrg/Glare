use crate::package::{parse_size_to_bytes, InstallReason, Package, PackageSource};
use anyhow::Result;
use std::collections::HashSet;
use std::process::Command;

/// Load all installed packages by running `pacman -Qi`
pub fn load_all_packages() -> Result<Vec<Package>> {
    let output = Command::new("pacman").args(["-Qi"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let packages = parse_qi_output(&stdout);
    Ok(packages)
}

/// Get names of AUR/foreign packages via `pacman -Qm`
pub fn load_aur_names() -> Result<HashSet<String>> {
    let output = Command::new("pacman").args(["-Qm"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let names: HashSet<String> = stdout
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(|s| s.to_string())
        .collect();
    Ok(names)
}

/// Get names of orphan packages via `pacman -Qdt`
pub fn load_orphan_names() -> Result<HashSet<String>> {
    let output = Command::new("pacman").args(["-Qdtq"]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let names: HashSet<String> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.trim().to_string())
        .collect();
    Ok(names)
}

/// Load files for a specific package via `pacman -Ql <name>`
pub fn load_package_files(name: &str) -> Result<Vec<String>> {
    let output = Command::new("pacman").args(["-Ql", name]).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            // Format: "pkgname /path/to/file"
            line.split_once(' ').map(|(_, path)| path.to_string())
        })
        .collect();
    Ok(files)
}

fn parse_qi_output(output: &str) -> Vec<Package> {
    let mut packages = Vec::new();
    let mut current_fields: Vec<(String, String)> = Vec::new();
    let mut current_key = String::new();
    let mut current_value = String::new();

    for line in output.lines() {
        if line.is_empty() {
            // End of package block
            if !current_key.is_empty() {
                current_fields.push((current_key.clone(), current_value.clone()));
            }
            if !current_fields.is_empty() {
                if let Some(pkg) = build_package(&current_fields) {
                    packages.push(pkg);
                }
            }
            current_fields.clear();
            current_key.clear();
            current_value.clear();
            continue;
        }

        // Try to find " : " separator - pacman uses format "Key<spaces>: Value"
        // The key is right-padded with spaces to align the colons
        if let Some(sep_pos) = find_field_separator(line) {
            // Save previous field
            if !current_key.is_empty() {
                current_fields.push((current_key.clone(), current_value.clone()));
            }
            current_key = line[..sep_pos].trim().to_string();
            current_value = line[sep_pos + 3..].trim().to_string();
        } else {
            // Continuation line (starts with spaces)
            if !current_key.is_empty() {
                current_value.push(' ');
                current_value.push_str(line.trim());
            }
        }
    }

    // Handle last package if file doesn't end with empty line
    if !current_key.is_empty() {
        current_fields.push((current_key, current_value));
    }
    if !current_fields.is_empty() {
        if let Some(pkg) = build_package(&current_fields) {
            packages.push(pkg);
        }
    }

    packages
}

fn find_field_separator(line: &str) -> Option<usize> {
    // Find " : " where the colon is preceded by at least one space
    // pacman format: "Name            : value"
    // We look for " : " pattern
    line.find(" : ")
}

fn build_package(fields: &[(String, String)]) -> Option<Package> {
    let get = |key: &str| -> String {
        fields
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .unwrap_or_default()
    };

    let get_list = |key: &str| -> Vec<String> {
        let val = get(key);
        if val.is_empty() || val == "None" {
            Vec::new()
        } else {
            val.split_whitespace().map(|s| s.to_string()).collect()
        }
    };

    // For optional deps, the format is different: "pkg: description  pkg2: description2"
    // They may span multiple continuation lines. The value we have is all joined.
    // We'll split on double-space or parse more carefully.
    let get_opt_deps = |key: &str| -> Vec<String> {
        let val = get(key);
        if val.is_empty() || val == "None" {
            return Vec::new();
        }
        // Optional deps can be like: "pkg1: desc1 pkg2: desc2" but this is tricky
        // In pacman output, each opt dep is on its own continuation line.
        // Since we join with ' ', let's just return the whole thing split reasonably
        // Actually, pacman -Qi shows optional deps one per line (continuation lines).
        // After our parsing, they're joined with spaces. Let's split on known patterns.
        // Safest: just return the whole string as one entry for display
        // Better: split on " [installed]" boundaries or treat each "word: description" as one entry
        val.split("  ") // pacman puts double-space between opt deps when on same line
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let name = get("Name");
    if name.is_empty() {
        return None;
    }

    let installed_size = get("Installed Size");
    let installed_size_bytes = parse_size_to_bytes(&installed_size);

    let install_reason_str = get("Install Reason");
    let install_reason = if install_reason_str.contains("Explicitly") {
        InstallReason::Explicit
    } else if install_reason_str.contains("dependency") || install_reason_str.contains("Dependency")
    {
        InstallReason::Dependency
    } else {
        InstallReason::Unknown
    };

    Some(Package {
        name,
        version: get("Version"),
        description: get("Description"),
        architecture: get("Architecture"),
        url: get("URL"),
        licenses: get_list("Licenses"),
        groups: get_list("Groups"),
        provides: get_list("Provides"),
        depends: get_list("Depends On"),
        optional_deps: get_opt_deps("Optional Deps"),
        required_by: get_list("Required By"),
        optional_for: get_list("Optional For"),
        conflicts: get_list("Conflicts With"),
        replaces: get_list("Replaces"),
        installed_size,
        installed_size_bytes,
        packager: get("Packager"),
        build_date: get("Build Date"),
        install_date: get("Install Date"),
        install_reason,
        validated_by: get("Validated By"),
        source: PackageSource::Official, // will be overridden
        is_orphan: false,                // will be overridden
    })
}
