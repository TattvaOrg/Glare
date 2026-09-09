use crate::config::Config;
use crate::package::{FileGroup, Package, PackageSource, InstallReason, group_files};
use crate::pacman;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterCategory {
    All,
    Explicit,
    Dependencies,
    AUR,
    Orphans,
    Recent,
    Largest,
}

impl FilterCategory {
    pub fn label(&self) -> &str {
        match self {
            Self::All => "All",
            Self::Explicit => "Explicit",
            Self::Dependencies => "Dependencies",
            Self::AUR => "AUR",
            Self::Orphans => "Orphans",
            Self::Recent => "Recent",
            Self::Largest => "Largest",
        }
    }

    pub fn all_categories() -> &'static [FilterCategory] {
        &[
            Self::All,
            Self::Explicit,
            Self::Dependencies,
            Self::AUR,
            Self::Orphans,
            Self::Recent,
            Self::Largest,
        ]
    }

    pub fn shortcut(&self) -> char {
        match self {
            Self::All => '1',
            Self::Explicit => '2',
            Self::Dependencies => '3',
            Self::AUR => '4',
            Self::Orphans => '5',
            Self::Recent => '6',
            Self::Largest => '7',
        }
    }
}

impl fmt::Display for FilterCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailTab {
    Info,
    Files,
    Dependencies,
}

impl DetailTab {
    pub fn label(&self) -> &str {
        match self {
            Self::Info => "Info",
            Self::Files => "Files",
            Self::Dependencies => "Deps",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Info => Self::Files,
            Self::Files => Self::Dependencies,
            Self::Dependencies => Self::Info,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Info => Self::Dependencies,
            Self::Files => Self::Info,
            Self::Dependencies => Self::Files,
        }
    }
}

impl fmt::Display for DetailTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePane {
    List,
    Detail,
}

pub struct App {
    pub packages: Vec<Package>,
    pub filtered_indices: Vec<usize>,
    pub list_state: ListState,
    pub filter: FilterCategory,
    pub search_query: String,
    pub is_searching: bool,
    pub detail_tab: DetailTab,
    pub detail_scroll: u16,
    pub active_pane: ActivePane,
    pub show_dashboard: bool,
    pub should_quit: bool,
    pub orphan_names: HashSet<String>,
    pub aur_names: HashSet<String>,
    pub cached_files: HashMap<String, Vec<FileGroup>>,
    pub total_installed_size: u64,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        // Load all data from pacman
        let mut packages = pacman::load_all_packages()?;
        let aur_names = pacman::load_aur_names().unwrap_or_default();
        let orphan_names = pacman::load_orphan_names().unwrap_or_default();

        // Mark AUR and orphan packages
        for pkg in &mut packages {
            if aur_names.contains(&pkg.name) {
                pkg.source = PackageSource::AUR;
            }
            if orphan_names.contains(&pkg.name) {
                pkg.is_orphan = true;
            }
        }

        // Sort by name initially
        packages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        let total_installed_size: u64 = packages.iter().map(|p| p.installed_size_bytes).sum();
        let filtered_indices: Vec<usize> = (0..packages.len()).collect();

        let mut list_state = ListState::default();
        if !packages.is_empty() {
            list_state.select(Some(0));
        }

        Ok(Self {
            packages,
            filtered_indices,
            list_state,
            filter: FilterCategory::All,
            search_query: String::new(),
            is_searching: false,
            detail_tab: DetailTab::Info,
            detail_scroll: 0,
            active_pane: ActivePane::List,
            show_dashboard: false,
            should_quit: false,
            orphan_names,
            aur_names,
            cached_files: HashMap::new(),
            total_installed_size,
            config,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Ctrl+C or 'q' always quits (unless searching)
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        if self.is_searching {
            self.handle_search_input(key);
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,

            // Pane switching
            KeyCode::Tab => self.toggle_pane(),

            // Search
            KeyCode::Char('/') => self.toggle_search(),

            // Filter categories (1-7)
            KeyCode::Char('1') => self.set_filter(FilterCategory::All),
            KeyCode::Char('2') => self.set_filter(FilterCategory::Explicit),
            KeyCode::Char('3') => self.set_filter(FilterCategory::Dependencies),
            KeyCode::Char('4') => self.set_filter(FilterCategory::AUR),
            KeyCode::Char('5') => self.set_filter(FilterCategory::Orphans),
            KeyCode::Char('6') => self.set_filter(FilterCategory::Recent),
            KeyCode::Char('7') => self.set_filter(FilterCategory::Largest),

            // Dashboard toggle
            KeyCode::Char('d') => self.show_dashboard = !self.show_dashboard,

            // Enter on a package loads its files
            KeyCode::Enter => self.load_files_for_selected(),

            // Navigation - depends on active pane
            KeyCode::Up | KeyCode::Char('k') => {
                if self.active_pane == ActivePane::Detail {
                    self.detail_scroll = self.detail_scroll.saturating_sub(1);
                } else {
                    self.move_selection(-1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.active_pane == ActivePane::Detail {
                    self.detail_scroll = self.detail_scroll.saturating_add(1);
                } else {
                    self.move_selection(1);
                }
            }
            KeyCode::Left | KeyCode::Char('h') if self.active_pane == ActivePane::Detail => {
                self.detail_tab = self.detail_tab.prev();
                self.detail_scroll = 0;
            }
            KeyCode::Right | KeyCode::Char('l') if self.active_pane == ActivePane::Detail => {
                self.detail_tab = self.detail_tab.next();
                self.detail_scroll = 0;
            }

            KeyCode::Home | KeyCode::Char('g') => self.move_to_start(),
            KeyCode::End | KeyCode::Char('G') => self.move_to_end(),
            KeyCode::PageUp => self.move_selection(-20),
            KeyCode::PageDown => self.move_selection(20),

            _ => {}
        }
    }

    fn handle_search_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.is_searching = false;
                self.search_query.clear();
                self.apply_filter();
            }
            KeyCode::Enter => {
                self.is_searching = false;
                // Keep the filter applied
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.apply_filter();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.apply_filter();
            }
            _ => {}
        }
    }

    fn toggle_search(&mut self) {
        self.is_searching = true;
        self.search_query.clear();
    }

    fn toggle_pane(&mut self) {
        self.active_pane = match self.active_pane {
            ActivePane::List => ActivePane::Detail,
            ActivePane::Detail => ActivePane::List,
        };
    }

    fn set_filter(&mut self, filter: FilterCategory) {
        self.filter = filter;
        self.apply_filter();
    }

    pub fn apply_filter(&mut self) {
        let query = self.search_query.to_lowercase();

        self.filtered_indices = self
            .packages
            .iter()
            .enumerate()
            .filter(|(_, pkg)| {
                // Apply category filter
                let category_match = match self.filter {
                    FilterCategory::All => true,
                    FilterCategory::Explicit => pkg.install_reason == InstallReason::Explicit,
                    FilterCategory::Dependencies => pkg.install_reason == InstallReason::Dependency,
                    FilterCategory::AUR => pkg.source == PackageSource::AUR,
                    FilterCategory::Orphans => pkg.is_orphan,
                    FilterCategory::Recent => true, // will sort later
                    FilterCategory::Largest => true, // will sort later
                };

                // Apply search filter
                let search_match = if query.is_empty() {
                    true
                } else {
                    pkg.name.to_lowercase().contains(&query)
                        || pkg.description.to_lowercase().contains(&query)
                };

                category_match && search_match
            })
            .map(|(i, _)| i)
            .collect();

        // Apply special sorting for Recent and Largest
        match self.filter {
            FilterCategory::Largest => {
                self.filtered_indices.sort_by(|&a, &b| {
                    self.packages[b]
                        .installed_size_bytes
                        .cmp(&self.packages[a].installed_size_bytes)
                });
            }
            FilterCategory::Recent => {
                // Sort by install date (reverse chronological)
                // pacman's date format is consistent enough for string comparison
                self.filtered_indices.sort_by(|&a, &b| {
                    self.packages[b]
                        .install_date
                        .cmp(&self.packages[a].install_date)
                });
            }
            _ => {}
        }

        // Reset selection
        if self.filtered_indices.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(0));
        }
        self.detail_scroll = 0;
    }

    pub fn selected_package(&self) -> Option<&Package> {
        let selected = self.list_state.selected()?;
        let pkg_index = self.filtered_indices.get(selected)?;
        self.packages.get(*pkg_index)
    }

    pub fn load_files_for_selected(&mut self) {
        if let Some(pkg) = self.selected_package() {
            let name = pkg.name.clone();
            if !self.cached_files.contains_key(&name) {
                if let Ok(files) = pacman::load_package_files(&name) {
                    let grouped = group_files(files);
                    self.cached_files.insert(name, grouped);
                }
            }
        }
        // Switch to Files tab
        self.detail_tab = DetailTab::Files;
    }

    pub fn selected_files(&self) -> Option<&Vec<FileGroup>> {
        let pkg = self.selected_package()?;
        self.cached_files.get(&pkg.name)
    }

    fn move_selection(&mut self, delta: i32) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0) as i32;
        let max = self.filtered_indices.len() as i32 - 1;
        let new = (current + delta).clamp(0, max) as usize;
        self.list_state.select(Some(new));
        self.detail_scroll = 0;

        // Pre-load files for the newly selected package (cache only)
        // Don't auto-switch tab
    }

    fn move_to_start(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.list_state.select(Some(0));
            self.detail_scroll = 0;
        }
    }

    fn move_to_end(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.list_state.select(Some(self.filtered_indices.len() - 1));
            self.detail_scroll = 0;
        }
    }

    // Stats helpers for dashboard
    pub fn total_packages(&self) -> usize {
        self.packages.len()
    }

    pub fn explicit_count(&self) -> usize {
        self.packages.iter().filter(|p| p.install_reason == InstallReason::Explicit).count()
    }

    pub fn dependency_count(&self) -> usize {
        self.packages.iter().filter(|p| p.install_reason == InstallReason::Dependency).count()
    }

    pub fn aur_count(&self) -> usize {
        self.aur_names.len()
    }

    pub fn orphan_count(&self) -> usize {
        self.orphan_names.len()
    }

    pub fn top_largest(&self, n: usize) -> Vec<&Package> {
        let mut indices: Vec<usize> = (0..self.packages.len()).collect();
        indices.sort_by(|&a, &b| self.packages[b].installed_size_bytes.cmp(&self.packages[a].installed_size_bytes));
        indices.into_iter().take(n).map(|i| &self.packages[i]).collect()
    }
}
