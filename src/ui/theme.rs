//! The single source of truth for the Aizen TUI palette — the **moonlight** identity: one calm
//! silver-blue accent + a small, restrained set of semantic colours (ok / error / warn / link) + a
//! code-syntax sub-palette. Everything is 256-colour (universally supported; no truecolor
//! dependency), routed through `console::style` so `NO_COLOR` and non-TTY output are auto-stripped.
//!
//! Discipline: NOT a rainbow. The silver moonlight carries brand + structure (prompt, gutter, tool
//! names, headings) — Aizen "holds the moon", so the whole UI is moonlit, not gilded. **Gold is
//! reserved** for one thing: the `⚡ yolo` mode chip + warnings (the one warm spot that should pull
//! the eye). The other semantic colours appear only where they MEAN something (green = success/added,
//! salmon = error/removed, blue = links/inline-code). Anything else is neutral grey.
//!
//! Use the helpers (`accent`, `ok`, `err`, …) instead of scattering raw `color256(..)` calls so the
//! palette can be retuned in one place.
//!
//! Mapped from the claude.ai/design "Aizen CLI" spec:
//!   moonlight #c3ccd8 (≈ 252, ACCENT) · dim silver #b6c0cf (≈ 248, ACCENT_DIM) · gold #d8b46a
//!   (≈ 179, WARN — yolo only) · green #5fbf7f (≈ 71, OK) · salmon #c98a82 (≈ 174, ERR) ·
//!   faint #56544c (≈ 240, FAINT). The PetalMark + wordmark are silver-white on the dark ground.

use console::{style, StyledObject};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// ── core palette (256-colour indices) ───────────────────────────────────────────
/// Moonlight silver — brand + structure (prompt arrow, assistant gutter, tool names, headings).
pub const ACCENT: u8 = 252;
/// Dim silver — secondary moonlight: tool arguments/values, the `◆ smart` chip, quiet rules/borders.
pub const ACCENT_DIM: u8 = 248;
/// Neutral grey for secondary text (the old `.dim()` role, but a defined shade).
pub const MUTED: u8 = 245;
/// Very faint grey — separators, the code-block rule, timestamps.
pub const FAINT: u8 = 240;

// ── semantic (used ONLY where the colour carries meaning) ────────────────────────
/// Success / confirmation / added.
pub const OK: u8 = 71;
/// Error / failure / removed — a soft "noir" salmon (#c98a82), not a glaring red.
pub const ERR: u8 = 174;
/// Warning / caution — the reserved warm gold (#d8b46a): the `⚡ yolo` chip + cautions, nothing else.
pub const WARN: u8 = 179;
/// Links + inline code (a calm blue, distinct from the gold accent).
pub const LINK: u8 = 110;

// ── code-syntax sub-palette (light, best-effort highlighter) ─────────────────────
pub const CODE_KEYWORD: u8 = 176; // soft mauve
pub const CODE_STRING: u8 = 108; // sage green
pub const CODE_NUMBER: u8 = 110; // blue
pub const CODE_COMMENT: u8 = 244; // grey
pub const CODE_RULE: u8 = 240; // the left │ / box border

// ── helpers (return StyledObject so callers can still chain .bold()/.italic()) ───
// NOTE: these read the LIVE palette (not the MOONLIGHT consts above), so `/theme`
// recolours every caller at once — markdown body, menus, slash output, not just the
// retained frame. Consts stay as the compiled-in default only.
pub fn accent<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(accent_idx())
}
pub fn accent_dim<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(accent_dim_idx())
}
pub fn muted<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(muted_idx())
}
pub fn faint<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(faint_idx())
}
pub fn ok<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(ok_idx())
}
pub fn err<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(err_idx())
}
pub fn warn<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(warn_idx())
}
pub fn link<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(link_idx())
}
pub fn code_keyword<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(code_keyword_idx())
}
pub fn code_string<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(code_string_idx())
}
pub fn code_number<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(code_number_idx())
}
pub fn code_comment<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(code_comment_idx())
}
pub fn code_rule<D: Display>(d: D) -> StyledObject<D> {
    style(d).color256(code_rule_idx())
}

/// A selectable TUI palette — opencode-style theme record.
///
/// Fields mirror the semantic roles the renderer actually uses. Values are 256-colour
/// indices (universally supported, no truecolor dependency), so custom `~/.aizen/themes/*.json`
/// files stay in the same space as the built-in constants above.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThemePalette {
    pub name: String,
    #[serde(default = "default_accent")]
    pub accent: u8,
    #[serde(default = "default_accent_dim")]
    pub accent_dim: u8,
    #[serde(default = "default_muted")]
    pub muted: u8,
    #[serde(default = "default_faint")]
    pub faint: u8,
    #[serde(default = "default_ok")]
    pub ok: u8,
    #[serde(default = "default_err")]
    pub err: u8,
    #[serde(default = "default_warn")]
    pub warn: u8,
    #[serde(default = "default_link")]
    pub link: u8,
    #[serde(default = "default_code_keyword")]
    pub code_keyword: u8,
    #[serde(default = "default_code_string")]
    pub code_string: u8,
    #[serde(default = "default_code_number")]
    pub code_number: u8,
    #[serde(default = "default_code_comment")]
    pub code_comment: u8,
    #[serde(default = "default_code_rule")]
    pub code_rule: u8,
}

fn default_accent() -> u8 {
    ACCENT
}
fn default_accent_dim() -> u8 {
    ACCENT_DIM
}
fn default_muted() -> u8 {
    MUTED
}
fn default_faint() -> u8 {
    FAINT
}
fn default_ok() -> u8 {
    OK
}
fn default_err() -> u8 {
    ERR
}
fn default_warn() -> u8 {
    WARN
}
fn default_link() -> u8 {
    LINK
}
fn default_code_keyword() -> u8 {
    CODE_KEYWORD
}
fn default_code_string() -> u8 {
    CODE_STRING
}
fn default_code_number() -> u8 {
    CODE_NUMBER
}
fn default_code_comment() -> u8 {
    CODE_COMMENT
}
fn default_code_rule() -> u8 {
    CODE_RULE
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self {
            name: "moonlight".to_string(),
            accent: ACCENT,
            accent_dim: ACCENT_DIM,
            muted: MUTED,
            faint: FAINT,
            ok: OK,
            err: ERR,
            warn: WARN,
            link: LINK,
            code_keyword: CODE_KEYWORD,
            code_string: CODE_STRING,
            code_number: CODE_NUMBER,
            code_comment: CODE_COMMENT,
            code_rule: CODE_RULE,
        }
    }
}

impl ThemePalette {
    #[allow(clippy::too_many_arguments)]
    fn named(
        name: &str,
        accent: u8,
        accent_dim: u8,
        muted: u8,
        faint: u8,
        ok: u8,
        err: u8,
        warn: u8,
        link: u8,
        code_keyword: u8,
        code_string: u8,
        code_number: u8,
        code_comment: u8,
        code_rule: u8,
    ) -> Self {
        Self {
            name: name.to_string(),
            accent,
            accent_dim,
            muted,
            faint,
            ok,
            err,
            warn,
            link,
            code_keyword,
            code_string,
            code_number,
            code_comment,
            code_rule,
        }
    }
}

/// Built-in themes. `moonlight` is the current default; the rest are opencode-inspired
/// 256-colour approximations (tokyonight / catppuccin / gruvbox / nord / matrix / one-dark).
/// Each theme also carries its own code-syntax sub-palette so `/theme matrix` turns code
/// blocks green-on-black too, not just the frame.
pub fn builtin_themes() -> Vec<ThemePalette> {
    vec![
        ThemePalette::default(),
        // tokyonight: blue accent, muted greys, warm yellow warn
        ThemePalette::named(
            "tokyonight",
            111,
            103,
            244,
            239,
            114,
            210,
            221,
            75,
            183,
            114,
            75,
            244,
            239,
        ),
        // catppuccin mocha-ish: lavender accent, sage ok, peach warn
        ThemePalette::named(
            "catppuccin",
            183,
            146,
            244,
            239,
            114,
            210,
            222,
            110,
            183,
            114,
            222,
            244,
            239,
        ),
        // gruvbox dark: warm beige accent, green ok, orange warn, red err
        ThemePalette::named(
            "gruvbox", 223, 187, 244, 239, 142, 167, 208, 109, 208, 142, 214, 244, 239,
        ),
        // nord: frost blue accent, dim polar greys
        ThemePalette::named(
            "nord", 153, 109, 245, 240, 114, 210, 222, 110, 110, 114, 110, 245, 240,
        ),
        // matrix: hacker green-on-black — EVERYTHING green so the change is unmistakable
        ThemePalette::named(
            "matrix", 46, 34, 244, 238, 46, 196, 226, 51, 46, 46, 46, 34, 34,
        ),
        // one-dark: cool blue accent, green ok, red err
        ThemePalette::named(
            "one-dark", 75, 67, 244, 239, 114, 203, 221, 75, 183, 114, 75, 244, 239,
        ),
    ]
}

/// All available theme names: built-ins first, then custom `~/.aizen/themes/*.json` files.
pub fn list_themes() -> Vec<String> {
    let mut names: Vec<String> = builtin_themes().iter().map(|t| t.name.clone()).collect();
    for custom in custom_theme_names() {
        if !names.iter().any(|n| n.eq_ignore_ascii_case(&custom)) {
            names.push(custom);
        }
    }
    names
}

fn themes_dir() -> std::path::PathBuf {
    crate::core::config::aizen_home().join("themes")
}

fn custom_theme_names() -> Vec<String> {
    let dir = themes_dir();
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(&dir) else {
        return out;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
            out.push(stem.to_string());
        }
    }
    out.sort();
    out
}

/// Resolve a theme by name: built-in (case-insensitive) → custom JSON file → `None`.
pub fn resolve_theme(name: &str) -> Option<ThemePalette> {
    let want = name.trim();
    if want.is_empty() {
        return None;
    }
    if let Some(b) = builtin_themes()
        .into_iter()
        .find(|t| t.name.eq_ignore_ascii_case(want))
    {
        return Some(b);
    }
    let path = themes_dir().join(format!("{want}.json"));
    let raw = std::fs::read_to_string(&path).ok()?;
    let mut pal: ThemePalette = serde_json::from_str(&raw).ok()?;
    if pal.name.trim().is_empty() {
        pal.name = want.to_string();
    }
    Some(pal)
}

// ── live palette (cached; never hits disk in the draw path) ──────────────────
static CURRENT: once_cell::sync::Lazy<std::sync::RwLock<ThemePalette>> =
    once_cell::sync::Lazy::new(|| std::sync::RwLock::new(ThemePalette::default()));

/// Currently active palette (cloned; cheap — 9 bytes + name).
pub fn current_palette() -> ThemePalette {
    CURRENT.read().map(|g| g.clone()).unwrap_or_default()
}

/// Activate a theme without persisting it; used by UI presets that save once after both parts apply.
pub fn set_theme_live(name: &str) -> Option<ThemePalette> {
    let pal = resolve_theme(name)?;
    if let Ok(mut g) = CURRENT.write() {
        *g = pal.clone();
    }
    Some(pal)
}

/// Activate a theme by name. Returns the palette on success, `None` when unknown.
/// Persists the choice into `cli-config.json` (`theme`) so it survives restarts.
pub fn set_theme(name: &str) -> Option<ThemePalette> {
    let pal = resolve_theme(name)?;
    if let Ok(mut g) = CURRENT.write() {
        *g = pal.clone();
    }
    // Best-effort persist; a failure never blocks the live switch.
    let mut cfg = crate::core::cli_config::load();
    cfg.theme = Some(pal.name.clone());
    let _ = crate::core::cli_config::save(&cfg);
    Some(pal)
}

/// Load the persisted theme (`cli-config.json` → `theme`) into the live cache.
/// Called once at REPL startup; unknown names fall back to moonlight.
pub fn apply_saved_theme() {
    let name = crate::core::cli_config::load()
        .theme
        .unwrap_or_else(|| "moonlight".to_string());
    if let Some(pal) = resolve_theme(&name) {
        if let Ok(mut g) = CURRENT.write() {
            *g = pal;
        }
    }
}

/// Dynamic indices — use these in NEW paint code so `/theme` takes effect.
/// Existing `theme::ACCENT`-style const uses keep compiling as the moonlight default.
pub fn accent_idx() -> u8 {
    current_palette().accent
}
pub fn accent_dim_idx() -> u8 {
    current_palette().accent_dim
}
pub fn muted_idx() -> u8 {
    current_palette().muted
}
pub fn faint_idx() -> u8 {
    current_palette().faint
}
pub fn ok_idx() -> u8 {
    current_palette().ok
}
pub fn err_idx() -> u8 {
    current_palette().err
}
pub fn warn_idx() -> u8 {
    current_palette().warn
}
pub fn link_idx() -> u8 {
    current_palette().link
}
pub fn code_keyword_idx() -> u8 {
    current_palette().code_keyword
}
pub fn code_string_idx() -> u8 {
    current_palette().code_string
}
pub fn code_number_idx() -> u8 {
    current_palette().code_number
}
pub fn code_comment_idx() -> u8 {
    current_palette().code_comment
}
pub fn code_rule_idx() -> u8 {
    current_palette().code_rule
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_colours_are_distinct() {
        // A regression guard: if two semantic roles collapse onto the same index the UI loses
        // meaning. Accent/ok/err/link/warn must all differ.
        let all = [ACCENT, OK, ERR, WARN, LINK];
        for (i, a) in all.iter().enumerate() {
            for b in &all[i + 1..] {
                assert_ne!(a, b, "two semantic colours share index {a}");
            }
        }
    }

    #[test]
    fn helpers_render_to_nonempty() {
        // Under the test harness colours may be stripped (no TTY); the text must still be present.
        assert!(accent("x").to_string().contains('x'));
        assert!(ok("done").to_string().contains("done"));
    }
}
