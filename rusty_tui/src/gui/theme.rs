use egui::{Color32, Rounding, Shadow, Stroke};
use super::messages::Role;

// Tokyo Night color palette
pub const BG: Color32 = Color32::from_rgb(26, 27, 38);        // #1a1b26
pub const FG: Color32 = Color32::from_rgb(192, 202, 245);     // #c0caf5
pub const CYAN: Color32 = Color32::from_rgb(122, 162, 247);   // #7aa2f7
pub const GREEN: Color32 = Color32::from_rgb(158, 206, 106);  // #9ece6a
pub const YELLOW: Color32 = Color32::from_rgb(224, 175, 104); // #e0af68
pub const GRAY: Color32 = Color32::from_rgb(65, 72, 104);     // #414868
pub const BRIGHT_CYAN: Color32 = Color32::from_rgb(125, 207, 255); // #7dcfff
pub const RED: Color32 = Color32::from_rgb(247, 118, 142);    // #f7768e

// Message bubble backgrounds
pub const USER_BUBBLE_BG: Color32 = Color32::from_rgb(45, 55, 72);      // Darker, bluer
pub const ASSISTANT_BUBBLE_BG: Color32 = Color32::from_rgb(36, 40, 59);  // Lighter, purplish
pub const SYSTEM_BUBBLE_BG: Color32 = Color32::from_rgb(52, 47, 42);     // Warm brown

// Code blocks
pub const CODE_BLOCK_BG: Color32 = Color32::from_rgb(20, 21, 30);        // Very dark
pub const CODE_BLOCK_BORDER: Color32 = Color32::from_rgb(65, 72, 104);   // Subtle border

// Shadows
pub const SHADOW_LIGHT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 15);

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

    // Widget colors - enhanced hover states
    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(36, 40, 59);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(41, 46, 66);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(60, 65, 85);  // Brighter
    style.visuals.widgets.active.bg_fill = BRIGHT_CYAN;

    // Selection color
    style.visuals.selection.bg_fill = CYAN.linear_multiply(0.3);

    // Spacing
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);

    // Rounding - consistent 8px for buttons
    style.visuals.window_rounding = 8.0.into();
    style.visuals.widgets.noninteractive.rounding = 8.0.into();
    style.visuals.widgets.inactive.rounding = 8.0.into();
    style.visuals.widgets.hovered.rounding = 8.0.into();
    style.visuals.widgets.active.rounding = 8.0.into();

    ctx.set_style(style);
}

/// Creates a frame for message bubbles with role-specific styling
pub fn message_bubble_frame(role: &Role) -> egui::Frame {
    let bg_color = match role {
        Role::User => USER_BUBBLE_BG,
        Role::Assistant => ASSISTANT_BUBBLE_BG,
        Role::System => SYSTEM_BUBBLE_BG,
    };

    egui::Frame::none()
        .fill(bg_color)
        .rounding(Rounding::same(12.0))
        .inner_margin(egui::Margin::symmetric(16.0, 12.0))
        .shadow(Shadow {
            offset: egui::vec2(0.0, 2.0),
            blur: 8.0,
            spread: 0.0,
            color: SHADOW_LIGHT,
        })
}

/// Creates a frame for message bubbles with alpha transparency for animations
pub fn message_bubble_frame_alpha(role: &Role, alpha: f32) -> egui::Frame {
    let base_color = match role {
        Role::User => USER_BUBBLE_BG,
        Role::Assistant => ASSISTANT_BUBBLE_BG,
        Role::System => SYSTEM_BUBBLE_BG,
    };

    // Apply alpha to background color
    let bg_color = Color32::from_rgba_premultiplied(
        (base_color.r() as f32 * alpha) as u8,
        (base_color.g() as f32 * alpha) as u8,
        (base_color.b() as f32 * alpha) as u8,
        (255.0 * alpha) as u8,
    );

    egui::Frame::none()
        .fill(bg_color)
        .rounding(Rounding::same(12.0))
        .inner_margin(egui::Margin::symmetric(16.0, 12.0))
        .shadow(Shadow {
            offset: egui::vec2(0.0, 2.0),
            blur: 8.0,
            spread: 0.0,
            color: SHADOW_LIGHT,
        })
}

/// Creates a frame for code blocks
pub fn code_block_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(CODE_BLOCK_BG)
        .stroke(Stroke::new(1.0, CODE_BLOCK_BORDER))
        .rounding(Rounding::same(8.0))
        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
}

/// Creates a frame for the input area
pub fn input_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Color32::from_rgb(36, 40, 59))
        .rounding(Rounding::same(10.0))
        .inner_margin(egui::Margin::symmetric(12.0, 8.0))
}

/// Creates a frame for the header
pub fn header_frame() -> egui::Frame {
    egui::Frame::none()
        .fill(Color32::from_rgb(30, 32, 45))
        .stroke(Stroke::new(1.0, Color32::from_rgb(45, 48, 65)))
        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
}
