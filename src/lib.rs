//! blnvim-ng — next-generation Rust-native Neovim distribution by pleme-io.
//!
//! This crate is the distribution coordinator for all blnvim-ng plugins.
//! Each plugin is a separate cdylib loaded by Neovim independently via
//! `require()`. This crate provides:
//!
//! - A manifest of all 28 plugins with descriptions and load order
//! - `:BlnvimNgStatus` — shows which plugins are enabled/disabled
//! - `:BlnvimNgVersion` — shows distribution version info
//! - Disable list via `vim.g.blnvim_ng_disable = {"niji", "suji"}`
//!
//! The actual plugin loading is handled by shikake (the plugin loader).
//! blnvim-ng coordinates configuration and reports status.

/// Distribution version (from Cargo.toml).
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A plugin entry in the distribution manifest.
struct PluginEntry {
    /// Crate name (used for `require()` in Neovim).
    name: &'static str,
    /// Japanese meaning.
    meaning: &'static str,
    /// What this plugin replaces or provides.
    description: &'static str,
    /// Whether this is a shared library (not a loadable plugin).
    is_shared_lib: bool,
}

/// The ordered manifest of all blnvim-ng components.
///
/// Load order matters: shared libraries first, then foundation plugins,
/// then UI/editor plugins. Shikake handles the actual loading; this
/// manifest defines the canonical order and metadata.
const MANIFEST: &[PluginEntry] = &[
    // ── Shared libraries (not loaded as plugins) ──
    PluginEntry {
        name: "tane",
        meaning: "seed",
        description: "Plugin SDK — config, autocommands, keymaps, highlights",
        is_shared_lib: true,
    },
    PluginEntry {
        name: "waku",
        meaning: "frame",
        description: "UI primitives — floating windows, borders, inputs",
        is_shared_lib: true,
    },
    PluginEntry {
        name: "kigi",
        meaning: "trees",
        description: "Treesitter integration — tree walking, node queries",
        is_shared_lib: true,
    },
    PluginEntry {
        name: "kakitori",
        meaning: "transcription",
        description: "Text/buffer ops — line manipulation, range ops, marks",
        is_shared_lib: true,
    },
    PluginEntry {
        name: "furui",
        meaning: "sieve",
        description: "Fuzzy matching, scoring, result ranking engine",
        is_shared_lib: true,
    },
    // ── Foundation plugins ──
    PluginEntry {
        name: "ishizue",
        meaning: "foundation",
        description: "Core utilities (replaces plenary.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "shikake",
        meaning: "mechanism",
        description: "Plugin loader and lifecycle manager (replaces lazy.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "kamon",
        meaning: "crest",
        description: "File type icons with Nerd Font support (replaces nvim-web-devicons)",
        is_shared_lib: false,
    },
    // ── UI and editor plugins ──
    PluginEntry {
        name: "koori",
        meaning: "ice",
        description: "Arctic colorscheme (replaces nord.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "tejun",
        meaning: "procedure",
        description: "Key binding hints (replaces which-key.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "tasuki",
        meaning: "sash",
        description: "Statusline (replaces lualine.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "narabi",
        meaning: "lineup",
        description: "Buffer/tab line (replaces bufferline.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "katachi",
        meaning: "form",
        description: "UI components and notifications (replaces nui + noice + nvim-notify)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "niji",
        meaning: "rainbow",
        description: "Color highlighter (replaces nvim-colorizer)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "suji",
        meaning: "line",
        description: "Indent guides (replaces indent-blankline)",
        is_shared_lib: false,
    },
    // ── Editor feature plugins ──
    PluginEntry {
        name: "shirube",
        meaning: "sign",
        description: "TODO comment highlights (replaces todo-comments.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "fuda",
        meaning: "tag",
        description: "Comment toggling (replaces Comment.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "hayashi",
        meaning: "grove",
        description: "File explorer (replaces oil.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "shiboru",
        meaning: "squeeze",
        description: "Fuzzy finder (replaces telescope.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "dougu",
        meaning: "tools",
        description: "Utility toolkit (replaces snacks.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "emaki",
        meaning: "scroll",
        description: "Markdown renderer (replaces render-markdown.nvim)",
        is_shared_lib: false,
    },
    // ── Language and LSP plugins ──
    PluginEntry {
        name: "hokan",
        meaning: "completion",
        description: "Completion engine (replaces nvim-cmp + sources + LuaSnip + autopairs)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "migaku",
        meaning: "polish",
        description: "Code formatter (replaces conform.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "kotowari",
        meaning: "reason",
        description: "LSP client and diagnostics (replaces lspconfig + lspsaga + trouble)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "shiren",
        meaning: "trial",
        description: "Test runner (replaces neotest + adapters)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "kozue",
        meaning: "treetop",
        description: "Treesitter textobjects and autotag (replaces treesitter-textobjects + autotag)",
        is_shared_lib: false,
    },
    // ── Integration plugins ──
    PluginEntry {
        name: "ayumi",
        meaning: "progress",
        description: "Git signs (replaces gitsigns.nvim)",
        is_shared_lib: false,
    },
    PluginEntry {
        name: "watari",
        meaning: "crossing",
        description: "Tmux navigator (replaces vim-tmux-navigator)",
        is_shared_lib: false,
    },
];

/// Check whether a plugin name is in the disable list.
fn is_disabled(name: &str, disabled: &[String]) -> bool {
    disabled.iter().any(|d| d == name)
}

/// Format the status table for `:BlnvimNgStatus`.
fn format_status(disabled: &[String]) -> String {
    let mut lines = Vec::with_capacity(MANIFEST.len() + 5);
    lines.push(format!(
        "blnvim-ng v{VERSION} \u{2014} Rust-native Neovim distribution"
    ));
    lines.push(String::new());
    lines.push(format!(
        "{:<12} {:<6} {:<16} {}",
        "Plugin", "State", "Meaning", "Description"
    ));
    lines.push("\u{2500}".repeat(76));

    for entry in MANIFEST {
        let state = if entry.is_shared_lib {
            "lib"
        } else if is_disabled(entry.name, disabled) {
            "off"
        } else {
            "on"
        };

        let icon = match state {
            "on" => "+",
            "off" => "-",
            _ => " ",
        };

        lines.push(format!(
            " {icon} {:<10} {:<6} {:<16} {}",
            entry.name, state, entry.meaning, entry.description,
        ));
    }

    lines.push(String::new());

    let plugin_count = MANIFEST.iter().filter(|e| !e.is_shared_lib).count();
    let disabled_count = MANIFEST
        .iter()
        .filter(|e| !e.is_shared_lib && is_disabled(e.name, disabled))
        .count();
    let enabled_count = plugin_count - disabled_count;

    lines.push(format!(
        "{enabled_count}/{plugin_count} plugins enabled, 5 shared libraries"
    ));

    lines.join("\n")
}

// ── Neovim plugin entry point ──
//
// Everything below requires nvim-oxi and only compiles when building the
// cdylib for Neovim. Tests exercise the pure-Rust manifest and formatting
// functions above without needing LuaJIT symbols.

#[cfg(not(test))]
mod plugin {
    use nvim_oxi as oxi;
    use nvim_oxi::api;
    use nvim_oxi::api::opts::EchoOpts;
    use tane::prelude::*;

    use crate::{format_status, MANIFEST, VERSION};

    /// Read the disable list from `vim.g.blnvim_ng_disable`.
    ///
    /// Expected format in Lua: `vim.g.blnvim_ng_disable = {"niji", "suji"}`
    /// Returns an empty vec if the variable is not set.
    fn read_disable_list() -> Vec<String> {
        match api::get_var::<Vec<String>>("blnvim_ng_disable") {
            Ok(list) => list,
            Err(_) => Vec::new(),
        }
    }

    /// Convert a `tane::Error` into an `oxi::Error`.
    fn tane_err(e: tane::Error) -> oxi::Error {
        oxi::Error::from(oxi::api::Error::Other(e.to_string()))
    }

    #[oxi::plugin]
    fn blnvim_ng() -> oxi::Result<()> {
        // Read the disable list once at plugin load time.
        let disabled = read_disable_list();

        // Store the normalised disable list for other plugins to query.
        let _ = api::set_var("blnvim_ng_disabled_plugins", disabled.clone());

        // Register :BlnvimNgStatus command.
        UserCommand::new("BlnvimNgStatus")
            .desc("Show blnvim-ng plugin status")
            .bar()
            .register(move |_args| {
                let current_disabled = read_disable_list();
                let output = format_status(&current_disabled);
                let opts = EchoOpts::builder().build();
                let _ = api::echo([(output.as_str(), None::<&str>)], true, &opts);
                Ok(())
            })
            .map_err(tane_err)?;

        // Register :BlnvimNgVersion command.
        UserCommand::new("BlnvimNgVersion")
            .desc("Show blnvim-ng version info")
            .bar()
            .register(|_args| {
                let plugin_count =
                    MANIFEST.iter().filter(|e| !e.is_shared_lib).count();
                let lib_count =
                    MANIFEST.iter().filter(|e| e.is_shared_lib).count();
                let output = format!(
                    "blnvim-ng v{VERSION} ({plugin_count} plugins, {lib_count} shared libraries)",
                );
                let opts = EchoOpts::builder().build();
                let _ = api::echo(
                    [(output.as_str(), None::<&str>)],
                    true,
                    &opts,
                );
                Ok(())
            })
            .map_err(tane_err)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_has_28_entries() {
        assert_eq!(MANIFEST.len(), 28);
    }

    #[test]
    fn manifest_has_5_shared_libs() {
        let count = MANIFEST.iter().filter(|e| e.is_shared_lib).count();
        assert_eq!(count, 5);
    }

    #[test]
    fn manifest_has_23_plugins() {
        let count = MANIFEST.iter().filter(|e| !e.is_shared_lib).count();
        assert_eq!(count, 23);
    }

    #[test]
    fn shared_libs_come_first() {
        let first_plugin_idx = MANIFEST
            .iter()
            .position(|e| !e.is_shared_lib)
            .expect("should have at least one plugin");
        let last_lib_idx = MANIFEST
            .iter()
            .rposition(|e| e.is_shared_lib)
            .expect("should have at least one shared lib");
        assert!(
            last_lib_idx < first_plugin_idx,
            "all shared libs should come before plugins in the manifest"
        );
    }

    #[test]
    fn all_names_unique() {
        let mut names: Vec<&str> = MANIFEST.iter().map(|e| e.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), MANIFEST.len(), "duplicate names in manifest");
    }

    #[test]
    fn all_descriptions_nonempty() {
        for entry in MANIFEST {
            assert!(
                !entry.description.is_empty(),
                "empty description for {}",
                entry.name
            );
        }
    }

    #[test]
    fn all_meanings_nonempty() {
        for entry in MANIFEST {
            assert!(
                !entry.meaning.is_empty(),
                "empty meaning for {}",
                entry.name
            );
        }
    }

    #[test]
    fn is_disabled_checks_correctly() {
        let disabled = vec!["niji".to_string(), "suji".to_string()];
        assert!(is_disabled("niji", &disabled));
        assert!(is_disabled("suji", &disabled));
        assert!(!is_disabled("koori", &disabled));
        assert!(!is_disabled("tane", &disabled));
    }

    #[test]
    fn format_status_contains_header() {
        let disabled = vec![];
        let output = format_status(&disabled);
        assert!(output.contains("blnvim-ng v"));
        assert!(output.contains("Plugin"));
        assert!(output.contains("State"));
    }

    #[test]
    fn format_status_shows_disabled() {
        let disabled = vec!["niji".to_string()];
        let output = format_status(&disabled);
        for line in output.lines() {
            if line.contains("niji") {
                assert!(line.contains("off"), "niji should show as off: {line}");
            }
        }
    }

    #[test]
    fn format_status_shows_enabled() {
        let disabled = vec![];
        let output = format_status(&disabled);
        for line in output.lines() {
            if line.contains("koori") {
                assert!(line.contains("on"), "koori should show as on: {line}");
            }
        }
    }

    #[test]
    fn format_status_shows_libs() {
        let disabled = vec![];
        let output = format_status(&disabled);
        for line in output.lines() {
            if line.contains("tane") {
                assert!(line.contains("lib"), "tane should show as lib: {line}");
            }
        }
    }

    #[test]
    fn format_status_count_line() {
        let disabled = vec!["niji".to_string(), "suji".to_string()];
        let output = format_status(&disabled);
        assert!(
            output.contains("21/23 plugins enabled"),
            "should show 21/23 when 2 disabled: {output}"
        );
    }

    #[test]
    fn version_constant_matches_cargo() {
        assert_eq!(VERSION, "0.1.0");
    }

    #[test]
    fn manifest_contains_all_expected_plugins() {
        let expected = [
            "tane", "waku", "kigi", "kakitori", "furui", "ishizue", "shikake",
            "kamon", "koori", "tejun", "tasuki", "narabi", "katachi", "niji",
            "suji", "shirube", "fuda", "hayashi", "shiboru", "dougu", "emaki",
            "hokan", "migaku", "kotowari", "shiren", "kozue", "ayumi", "watari",
        ];
        let names: Vec<&str> = MANIFEST.iter().map(|e| e.name).collect();
        for name in &expected {
            assert!(
                names.contains(name),
                "manifest missing expected plugin: {name}"
            );
        }
    }

    #[test]
    fn format_status_all_enabled_count() {
        let disabled = vec![];
        let output = format_status(&disabled);
        assert!(
            output.contains("23/23 plugins enabled"),
            "should show 23/23 when none disabled: {output}"
        );
    }
}
