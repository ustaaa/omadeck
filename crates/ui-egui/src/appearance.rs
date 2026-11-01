//! Transient UI-only appearance. The presentation/font database/renderers are never modified.
use crate::omarchy::{FontFace, Palette, blend};
use crate::theme::Tokens;
use egui::{Color32, FontData, FontDefinitions, FontFamily};

fn palette_id() -> egui::Id {
    egui::Id::unique("omadeck.appearance.palette")
}
fn fonts_id() -> egui::Id {
    egui::Id::unique("omadeck.appearance.fonts")
}

pub(crate) fn refresh(ctx: &egui::Context, palette: Option<Palette>, fonts: Option<(FontFace, FontFace)>, allow_fonts: bool) -> (bool, bool) {
    let old_palette = ctx.data(|d| d.get_temp::<Palette>(palette_id()));
    let old_fonts = ctx.data(|d| d.get_temp::<(FontFace, FontFace)>(fonts_id()));
    let changed = (old_palette != palette, allow_fonts && old_fonts != fonts);
    if changed.0 {
        ctx.data_mut(|d| {
            if let Some(p) = palette {
                d.insert_temp(palette_id(), p);
            } else {
                d.remove::<Palette>(palette_id());
            }
        });
    }
    if changed.1 {
        ctx.data_mut(|d| {
            if let Some(f) = fonts {
                d.insert_temp(fonts_id(), f);
            } else {
                d.remove::<(FontFace, FontFace)>(fonts_id());
            }
        });
    }
    changed
}

pub(crate) fn tokens(ctx: &egui::Context, mut t: Tokens) -> Tokens {
    let Some(p) = ctx.data(|d| d.get_temp::<Palette>(palette_id())) else { return t };
    t.dark = p.dark;
    t.chrome = p.background;
    t.card = p.surface;
    t.canvas = p.background;
    t.text = readable(p.foreground, p.surface, 4.5);
    t.text_dim = readable(p.muted, p.surface, 4.5);
    t.text_faint = blend(p.surface, t.text, 0.6);
    t.accent = readable(p.accent, p.surface, 4.5);
    t.accent_text = readable(p.background, t.accent, 4.5);
    t.contextual = t.accent;
    t.text_field = p.background;
    t.field_bg = p.background;
    t.hover = blend(p.surface, t.text, 0.08);
    t.pressed = blend(p.surface, t.text, 0.16);
    t.selected = p.selection;
    t.border = blend(p.surface, t.text, 0.28);
    t.card_border = t.border;
    t.separator = t.border;
    t.control = blend(p.surface, t.text, 0.20);
    t.control_hover = t.hover;
    t.control_pressed = t.pressed;
    // Slide content and paper interaction tokens (caret, guides, handles) retain their baseline.
    t
}

fn luminance(c: Color32) -> f32 {
    let linear = |n: u8| {
        let n = f32::from(n) / 255.0;
        if n <= 0.04045 { n / 12.92 } else { ((n + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * linear(c.r()) + 0.7152 * linear(c.g()) + 0.0722 * linear(c.b())
}
fn contrast(a: Color32, b: Color32) -> f32 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}
fn readable(color: Color32, bg: Color32, minimum: f32) -> Color32 {
    if contrast(color, bg) >= minimum {
        return color;
    }
    let target = if contrast(Color32::WHITE, bg) > contrast(Color32::BLACK, bg) { Color32::WHITE } else { Color32::BLACK };
    for i in 1..=20 {
        let c = blend(color, target, i as f32 / 20.0);
        if contrast(c, bg) >= minimum {
            return c;
        }
    }
    target
}

pub(crate) fn fonts(ctx: &egui::Context, defs: &mut FontDefinitions) {
    let Some((normal, bold)) = ctx.data(|d| d.get_temp::<(FontFace, FontFace)>(fonts_id())) else { return };
    for (name, face, family) in
        [("omadeck-desktop-regular", normal, FontFamily::Name("omadeck-ui".into())), ("omadeck-desktop-bold", bold, FontFamily::Name("bold".into()))]
    {
        let mut data = FontData::from_owned((*face.bytes).clone());
        data.index = face.index;
        defs.font_data.insert(name.into(), std::sync::Arc::new(data));
        defs.families.entry(family).or_default().insert(0, name.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn palette_changes_chrome_but_keeps_paper_interaction_tokens() {
        let ctx = egui::Context::default();
        let p = Palette::parse("background='#101020'\nforeground='#eeeeff'\naccent='#bbaaee'", false).unwrap();
        assert_eq!(refresh(&ctx, Some(p), None, true), (true, false));
        let base = Tokens::light();
        let result = tokens(&ctx, base);
        assert_eq!(result.chrome, p.background);
        assert_eq!(result.caret, base.caret);
        assert_eq!(result.guide, base.guide);
        assert_eq!(result.handle_fill, base.handle_fill);
        assert!(contrast(result.text, result.card) >= 4.5);
        assert_eq!(refresh(&ctx, None, None, true), (true, false));
        assert_eq!(tokens(&ctx, base).chrome, base.chrome);
    }
    #[test]
    fn appearance_does_not_replace_document_proportional_font_chain() {
        let ctx = egui::Context::default();
        let face = FontFace { bytes: std::sync::Arc::new(vec![1, 2, 3]), index: 0 };
        refresh(&ctx, None, Some((face.clone(), face)), true);
        let mut defs = crate::theme::font_definitions(&[]);
        let before = defs.families.get(&FontFamily::Proportional).cloned();
        fonts(&ctx, &mut defs);
        assert_eq!(defs.families.get(&FontFamily::Proportional).cloned(), before);
        assert_eq!(defs.families.get(&FontFamily::Name("omadeck-ui".into())).unwrap()[0], "omadeck-desktop-regular");
    }
}
