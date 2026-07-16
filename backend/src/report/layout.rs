use chrono::Local;
use printpdf::*;

use super::chart::{draw_bar_chart, extract_scores};
use super::info::{draw_additional_info, draw_footer_disclaimer, get_report_title};

/// Generate a PDF report for a given personality test result.
///
/// * `test_type` — one of: big_five, mbti, enneagram, disc, dark_triad, human_design
/// * `result` — the JSON result from the scoring function
/// * `language` — "en" or "es"
pub fn generate_report(test_type: &str, result: &serde_json::Value, language: &str) -> Vec<u8> {
    let title = format!("PsicoMetrics — {}", get_report_title(test_type, language));
    let (doc, page_idx, layer_idx) =
        PdfDocument::new(&title, Mm(210.0), Mm(297.0), "report");

    let font = match doc.add_builtin_font(BuiltinFont::Helvetica) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("Failed to load Helvetica font: {e}");
            return Vec::new();
        }
    };
    let font_bold = match doc.add_builtin_font(BuiltinFont::HelveticaBold) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("Failed to load HelveticaBold font: {e}");
            return Vec::new();
        }
    };
    let font_italic = match doc.add_builtin_font(BuiltinFont::HelveticaOblique) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("Failed to load HelveticaOblique font: {e}");
            return Vec::new();
        }
    };

    let layer = doc.get_page(page_idx).get_layer(layer_idx);
    let mut y = Mm(272.0);

    // ── Title ──
    let _ = layer.use_text(&title, 20.0, Mm(20.0), y, &font_bold);
    y -= Mm(10.0);

    // ── Date ──
    let date_prefix = if language == "en" {
        "Generated on: "
    } else {
        "Generado el: "
    };
    let date_str = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let _ = layer.use_text(
        &format!("{date_prefix}{date_str}"),
        9.0,
        Mm(20.0),
        y,
        &font,
    );
    y -= Mm(14.0);

    // ── Horizontal rule ──
    layer.set_fill_color(Color::Rgb(Rgb::new(0.75, 0.75, 0.75, None)));
    layer.add_rect(Rect::new(Mm(20.0), y - Mm(0.5), Mm(190.0), y + Mm(0.25)));
    y -= Mm(8.0);

    // ── Profile summary ──
    let summary_label = if language == "en" {
        "Profile Summary"
    } else {
        "Resumen del Perfil"
    };
    let _ = layer.use_text(summary_label, 14.0, Mm(20.0), y, &font_bold);
    y -= Mm(8.0);

    let summary = result
        .get("profile_summary")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let wrapped = textwrap::fill(summary, 85);
    for line in wrapped.lines() {
        if y < Mm(40.0) {
            break;
        }
        let _ = layer.use_text(line, 9.0, Mm(22.0), y, &font);
        y -= Mm(4.5);
    }
    y -= Mm(6.0);

    // ── Score chart ──
    let scores = extract_scores(test_type, result);
    if !scores.is_empty() {
        let chart_label = if language == "en" {
            "Score Breakdown"
        } else {
            "Desglose de Puntuaciones"
        };
        let _ = layer.use_text(chart_label, 14.0, Mm(20.0), y, &font_bold);
        y -= Mm(10.0);

        draw_bar_chart(&layer, &scores, &mut y, &font, &font_bold);
    }

    // ── Test-specific info ──
    y -= Mm(4.0);
    draw_additional_info(test_type, result, &layer, &mut y, &font, &font_bold, language);

    // ── Footer disclaimer ──
    draw_footer_disclaimer(&layer, &font_italic, language);

    // ── Serialise ──
    match doc.save_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to save PDF to bytes: {e}");
            Vec::new()
        }
    }
}
