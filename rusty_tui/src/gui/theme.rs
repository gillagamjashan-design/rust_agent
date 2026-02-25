use egui::Color32;

// Tokyo Night color palette
pub const BG: Color32 = Color32::from_rgb(26, 27, 38);        // #1a1b26
pub const FG: Color32 = Color32::from_rgb(192, 202, 245);     // #c0caf5
pub const CYAN: Color32 = Color32::from_rgb(122, 162, 247);   // #7aa2f7
pub const GREEN: Color32 = Color32::from_rgb(158, 206, 106);  // #9ece6a
pub const YELLOW: Color32 = Color32::from_rgb(224, 175, 104); // #e0af68
pub const GRAY: Color32 = Color32::from_rgb(65, 72, 104);     // #414868
pub const BRIGHT_CYAN: Color32 = Color32::from_rgb(125, 207, 255); // #7dcfff
pub const RED: Color32 = Color32::from_rgb(247, 118, 142);    // #f7768e

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Set dark mode
    style.visuals.dark_mode = true;

    // Background colors
    style.visuals.window_fill = BG;
    style.visuals.panel_fill = BG;
    style.visuals.extreme_bg_color = BG;

    // Text colors
    style.visuals.override_text_color = Some(FG);

    // Widget colors
    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(36, 40, 59);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(41, 46, 66);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(51, 56, 76);
    style.visuals.widgets.active.bg_fill = BRIGHT_CYAN;

    // Selection color
    style.visuals.selection.bg_fill = CYAN.linear_multiply(0.3);

    // Window rounding
    style.visuals.window_rounding = 8.0.into();
    style.visuals.widgets.noninteractive.rounding = 4.0.into();

    ctx.set_style(style);
}
