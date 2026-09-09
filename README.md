# Glare

**A lightweight system state manager TUI for Arch Linux.**

Glare gives you complete visibility into everything installed on your system -- packages, files, dependencies, disk usage -- all in a fast, keyboard-driven terminal interface.

Think of it as a **file manager, but for your system state**.

## Features

- **Full Package Inventory** -- Browse every installed package with name, version, description, size, install date, and install reason
- **File Location Mapping** -- See exactly where each package installs its files, grouped by category (Binaries, Libraries, Config, Data, etc.)
- **Dependency Graph** -- View what each package depends on, what depends on it, optional deps, provides, conflicts, and replaces
- **Smart Filtering** -- Filter by: All, Explicit, Dependencies, AUR, Orphans, Recent, Largest
- **Fuzzy Search** -- Instantly search packages by name or description
- **Dashboard** -- Overview with package stats, disk usage breakdown, and top 10 largest packages with bar chart
- **Blazing Fast** -- Pure Rust, single binary, queries pacman live with zero persistent storage
- **Adaptive Theme** -- Respects your terminal colors with a clean, minimal aesthetic

```

## Installation

### One-Liner

```bash
curl -sL https://raw.githubusercontent.com/AbsolOrg/Glare/main/install.sh | bash
```

### From Source

```bash
git clone https://github.com/AbsolOrg/Glare.git
cd Glare
cargo build --release
sudo install -Dm755 target/release/glare /usr/local/bin/glare
```

### Requirements

- Arch Linux (or Arch-based: CachyOS, EndeavourOS, Manjaro, etc.)
- `pacman` (comes with Arch)
- Rust toolchain (for building from source)

## Usage

```bash
glare
```

That's it. One command, full system visibility.

## Keybindings

### Navigation
| Key | Action |
|-----|--------|
| `Up` / `k` | Move up (list or scroll detail) |
| `Down` / `j` | Move down (list or scroll detail) |
| `g` / `Home` | Jump to top |
| `G` / `End` | Jump to bottom |
| `PgUp` / `PgDn` | Page up / down |
| `Tab` | Switch between list and detail pane |
| `Enter` | Load file list for selected package |

### Detail Panel
| Key | Action |
|-----|--------|
| `Left` / `h` | Previous tab (Info > Files > Deps) |
| `Right` / `l` | Next tab |

### Filtering
| Key | Filter |
|-----|--------|
| `1` | All packages |
| `2` | Explicitly installed |
| `3` | Dependencies |
| `4` | AUR packages |
| `5` | Orphans (cleanup candidates) |
| `6` | Recently installed |
| `7` | Largest by size |
| `/` | Search by name/description |

### General
| Key | Action |
|-----|--------|
| `d` | Toggle dashboard view |
| `q` | Quit |
| `Ctrl+C` | Force quit |

## Configuration

Glare uses a TOML config file at `~/.config/glare/config.toml`. It works out of the box with zero configuration.

```toml
# AUR helper to use (default: "paru")
aur_helper = "paru"

# Default filter view (default: "all")
# Options: all, explicit, dependencies, aur, orphans, recent, largest
default_filter = "all"

# Default sort order (default: "name")
default_sort = "name"
```

## Architecture

```
src/
|-- main.rs            Entry point, terminal setup, event loop
|-- app.rs             Application state, key handling, filtering
|-- package.rs         Package data structures, file grouping
|-- pacman.rs          Pacman CLI queries and output parsing
|-- config.rs          TOML configuration loading
|-- event.rs           Crossterm event handling
+-- ui/
    |-- mod.rs         Main render orchestration
    |-- theme.rs       Color and style definitions
    |-- header.rs      Top bar with filters and search
    |-- footer.rs      Bottom bar with keybindings
    |-- package_list.rs    Left panel - package list
    |-- detail.rs      Right panel - package details
    +-- dashboard.rs   Dashboard overview with stats
```

## What Glare Shows You

### Per Package (Detail Panel)
- **Info tab**: Name, version, description, install date, install reason (explicit/dependency), source (official/AUR), size, architecture, URL, licenses, groups, packager, build date, orphan status
- **Files tab**: Every file the package owns, grouped by location:
  - Binaries (`/usr/bin/`)
  - Libraries (`/usr/lib/`)
  - Data and Resources (`/usr/share/`)
  - Documentation (`/usr/share/doc/`, `/usr/share/man/`)
  - Configuration (`/etc/`)
  - Headers (`/usr/include/`)
- **Deps tab**: Depends On, Optional Deps, Required By, Optional For, Provides, Conflicts, Replaces

### Dashboard
- Total package count (explicit vs dependencies vs AUR)
- Orphan count (cleanup candidates)
- Total disk usage by all packages
- Top 10 largest packages with visual bar chart

## License

GPL-3.0 -- See [LICENSE](LICENSE) for details.
