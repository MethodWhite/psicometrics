use chrono::Local;
use printpdf::*;

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
    let disc1 = if language == "en" {
        "This report is for educational and self-awareness purposes only."
    } else {
        "Este informe es solo para fines educativos y de autoconocimiento."
    };
    let disc2 = if language == "en" {
        "It does not replace a professional psychological evaluation. © PsicoMetrics"
    } else {
        "No reemplaza una evaluación psicológica profesional. © PsicoMetrics"
    };
    let disc_footer_y = Mm(14.0);
    layer.set_fill_color(Color::Rgb(Rgb::new(0.5, 0.5, 0.5, None)));
    layer.add_rect(Rect::new(
        Mm(20.0),
        disc_footer_y + Mm(8.0),
        Mm(190.0),
        disc_footer_y + Mm(8.3),
    ));
    let _ = layer.use_text(disc1, 7.0, Mm(20.0), disc_footer_y, &font_italic);
    let _ = layer.use_text(disc2, 7.0, Mm(20.0), disc_footer_y - Mm(4.0), &font_italic);

    // ── Serialise ──
    match doc.save_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to save PDF to bytes: {e}");
            Vec::new()
        }
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn get_report_title(test_type: &str, lang: &str) -> String {
    match (test_type, lang) {
        ("big_five", "en") => "Big Five Personality Test Report",
        ("big_five", _) => "Informe de Personalidad Big Five",
        ("mbti", "en") => "MBTI Personality Type Report",
        ("mbti", _) => "Informe de Tipo de Personalidad MBTI",
        ("enneagram", "en") => "Enneagram Personality Report",
        ("enneagram", _) => "Informe de Personalidad Eneagrama",
        ("disc", "en") => "DISC Personality Profile Report",
        ("disc", _) => "Informe de Perfil DISC",
        ("dark_triad", "en") => "Dark Triad Traits Report",
        ("dark_triad", _) => "Informe de Rasgos de la Tríada Oscura",
        ("human_design", "en") => "Human Design Report",
        ("human_design", _) => "Informe de Diseño Humano",
        _ => "PsicoMetrics Report",
    }
    .to_string()
}

/// Extract (label, score_0_100) pairs from the result JSON.
fn extract_scores(test_type: &str, result: &serde_json::Value) -> Vec<(String, f64)> {
    match test_type {
        "big_five" => {
            let names: &[(&str, &str)] = &[
                ("O", "Openness"),
                ("C", "Conscientiousness"),
                ("E", "Extraversion"),
                ("A", "Agreeableness"),
                ("N", "Neuroticism"),
            ];
            names
                .iter()
                .filter_map(|(k, label)| {
                    result["scores"]
                        .get(*k)
                        .and_then(|v| v.as_f64())
                        .map(|s| (label.to_string(), s))
                })
                .collect()
        }
        "mbti" => {
            let dims: &[(&str, &str, &str)] = &[
                ("EI", "Extraversion", "Introversion"),
                ("SN", "Sensing", "Intuition"),
                ("TF", "Thinking", "Feeling"),
                ("JP", "Judging", "Perceiving"),
            ];
            dims.iter()
                .filter_map(|(key, pole_a, _pole_b)| {
                    result["percentages"]
                        .get(*key)
                        .and_then(|v| v.as_f64())
                        .map(|pct_a| {
                            let dominant = if pct_a >= 50.0 { *pole_a } else { _pole_b };
                            let display_pct =
                                if pct_a >= 50.0 { pct_a } else { 100.0 - pct_a };
                            (format!("{dominant}"), display_pct)
                        })
                })
                .collect()
        }
        "enneagram" => (1..=9)
            .filter_map(|t| {
                result["scores"]
                    .get(t.to_string())
                    .and_then(|v| v.as_f64())
                    .map(|s| (format!("Type {t}"), s))
            })
            .collect(),
        "disc" => {
            let names: &[(&str, &str)] = &[
                ("D", "Dominance"),
                ("I", "Influence"),
                ("S", "Steadiness"),
                ("C", "Compliance"),
            ];
            names
                .iter()
                .filter_map(|(k, label)| {
                    result["scores"]
                        .get(*k)
                        .and_then(|v| v.as_f64())
                        .map(|s| (label.to_string(), s))
                })
                .collect()
        }
        "dark_triad" => {
            let names: &[(&str, &str)] = &[
                ("machiavellianism", "Machiavellianism"),
                ("narcissism", "Narcissism"),
                ("psychopathy", "Psychopathy"),
            ];
            let mut out: Vec<_> = names
                .iter()
                .filter_map(|(k, label)| {
                    result["scores"]
                        .get(*k)
                        .and_then(|v| v.as_f64())
                        .map(|s| (label.to_string(), s))
                })
                .collect();
            if let Some(core) = result["dark_core"].as_f64() {
                out.push(("Dark Core".to_string(), core));
            }
            out
        }
        _ => vec![],
    }
}

/// Draw horizontal bar chart on the current layer, advancing `y` downward.
#[allow(clippy::too_many_arguments)]
fn draw_bar_chart(
    layer: &PdfLayerReference,
    scores: &[(String, f64)],
    y: &mut Mm,
    font: &IndirectFontRef,
    _font_bold: &IndirectFontRef,
) {
    let bar_max_w = 120.0_f32;
    let bar_h = 6.5_f32;
    let gap = 1.5_f32;
    let label_x = Mm(22.0);
    let bar_start_x = Mm(55.0);
    let val_start_x = bar_start_x + Mm(bar_max_w) + Mm(3.0);

    for (name, score) in scores {
        if *y < Mm(35.0) {
            break;
        }

        let _ = layer.use_text(name, 8.0, label_x, *y, font);

        layer.set_fill_color(Color::Rgb(Rgb::new(0.88, 0.90, 0.94, None)));
        layer.add_rect(Rect::new(
            bar_start_x,
            *y,
            bar_start_x + Mm(bar_max_w),
            *y + Mm(bar_h),
        ));

        let fill_ratio = ((score / 100.0).clamp(0.0, 1.0)) as f32;
        let fill_w = bar_max_w * fill_ratio;
        if fill_w > 0.5 {
            layer.set_fill_color(Color::Rgb(Rgb::new(0.15, 0.40, 0.72, None)));
            layer.add_rect(Rect::new(
                bar_start_x,
                *y,
                bar_start_x + Mm(fill_w),
                *y + Mm(bar_h),
            ));
        }

        let _ = layer.use_text(&format!("{score:.0}%"), 7.5, val_start_x, *y, font);
        *y -= Mm(bar_h + gap);
    }
}

/// Draw test-specific textual info (type code, dominant type, etc.).
#[allow(clippy::too_many_arguments)]
fn draw_additional_info(
    test_type: &str,
    result: &serde_json::Value,
    layer: &PdfLayerReference,
    y: &mut Mm,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    language: &str,
) {
    match test_type {
        "mbti" => {
            let type_code = result.get("type_code").and_then(|v| v.as_str()).unwrap_or("");
            if !type_code.is_empty() && *y > Mm(40.0) {
                let label = if language == "en" {
                    "Your MBTI Type:"
                } else {
                    "Tu Tipo MBTI:"
                };
                let _ = layer.use_text(label, 12.0, Mm(20.0), *y, font_bold);
                *y -= Mm(7.0);
                let _ = layer.use_text(type_code, 16.0, Mm(24.0), *y, font);
                *y -= Mm(10.0);
            }
        }
        "enneagram" => {
            if let (Some(dom), Some(wing)) =
                (result["dominant_type"].as_u64(), result["wing"].as_u64())
            {
                if *y > Mm(40.0) {
                    let label = if language == "en" {
                        "Dominant Type:"
                    } else {
                        "Tipo Dominante:"
                    };
                    let _ = layer.use_text(label, 12.0, Mm(20.0), *y, font_bold);
                    *y -= Mm(7.0);
                    let _ = layer.use_text(&format!("{dom}w{wing}"), 16.0, Mm(24.0), *y, font);
                    *y -= Mm(10.0);
                }
            }
        }
        "disc" => {
            if let (Some(primary), Some(secondary)) = (
                result["primary_style"].as_str(),
                result["secondary_style"].as_str(),
            ) {
                if *y > Mm(40.0) {
                    let label = if language == "en" {
                        "Primary / Secondary Styles:"
                    } else {
                        "Estilos Primario / Secundario:"
                    };
                    let _ = layer.use_text(label, 12.0, Mm(20.0), *y, font_bold);
                    *y -= Mm(7.0);
                    let _ = layer.use_text(
                        &format!("{primary} / {secondary}"),
                        14.0,
                        Mm(24.0),
                        *y,
                        font,
                    );
                    *y -= Mm(10.0);
                }
            }
        }
        "dark_triad" => {
            if let Some(risk) = result["risk_level"].as_str() {
                if *y > Mm(40.0) {
                    let label = if language == "en" {
                        "Risk Level:"
                    } else {
                        "Nivel de Riesgo:"
                    };
                    let _ = layer.use_text(label, 12.0, Mm(20.0), *y, font_bold);
                    *y -= Mm(7.0);
                    let _ = layer.use_text(risk, 14.0, Mm(24.0), *y, font);
                    *y -= Mm(10.0);
                }
            }
        }
        "human_design" => {
            let fields: &[(&str, &str)] = &[
                ("type", if language == "en" { "Type" } else { "Tipo" }),
                (
                    "strategy",
                    if language == "en" {
                        "Strategy"
                    } else {
                        "Estrategia"
                    },
                ),
                (
                    "authority",
                    if language == "en" {
                        "Authority"
                    } else {
                        "Autoridad"
                    },
                ),
                (
                    "profile",
                    if language == "en" {
                        "Profile"
                    } else {
                        "Perfil"
                    },
                ),
            ];
            for &(json_key, label) in fields {
                if *y < Mm(40.0) {
                    break;
                }
                let _ = layer.use_text(label.to_string(), 10.0, Mm(20.0), *y, font_bold);
                *y -= Mm(5.0);
                let val = result.get(json_key).and_then(|v| v.as_str()).unwrap_or("");
                let _ = layer.use_text(val, 10.0, Mm(24.0), *y, font);
                *y -= Mm(8.0);
            }

            if let Some(centers) = result["centers"].as_object() {
                if *y > Mm(40.0) {
                    let clabel = if language == "en" {
                        "Defined Centers"
                    } else {
                        "Centros Definidos"
                    };
                    let _ = layer.use_text(clabel, 12.0, Mm(20.0), *y, font_bold);
                    *y -= Mm(7.0);
                    let mut defined: Vec<String> = centers
                        .iter()
                        .filter(|(_, v)| v.as_bool().unwrap_or(false))
                        .map(|(k, _)| k.clone())
                        .collect();
                    defined.sort();
                    let line = defined.join(", ");
                    let wrapped = textwrap::fill(&line, 75);
                    for l in wrapped.lines() {
                        if *y < Mm(35.0) {
                            break;
                        }
                        let _ = layer.use_text(l, 9.0, Mm(24.0), *y, font);
                        *y -= Mm(4.5);
                    }
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_big_five_pdf() {
        let result = serde_json::json!({
            "scores": {"O": 72.0, "C": 45.0, "E": 58.0, "A": 80.0, "N": 30.0},
            "profile_summary": "Openness: high creative, imaginative, enjoys new ideas. Conscientiousness: balanced. Extraversion: balanced. Agreeableness: high cooperative, cares about others. Neuroticism: low emotionally stable."
        });
        let pdf = generate_report("big_five", &result, "en");
        assert!(pdf.len() > 1000);
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn test_generate_mbti_pdf() {
        let result = serde_json::json!({
            "type_code": "INFJ",
            "scores": {"EI": 75.0, "SN": 20.0, "TF": 35.0, "JP": 80.0},
            "percentages": {"EI": 75.0, "SN": 20.0, "TF": 35.0, "JP": 80.0},
            "profile_summary": "The Advocate. Type INFJ."
        });
        let pdf = generate_report("mbti", &result, "en");
        assert!(pdf.len() > 1000);
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn test_generate_spanish_pdf() {
        let result = serde_json::json!({
            "scores": {"O": 65.0, "C": 50.0, "E": 72.0, "A": 40.0, "N": 55.0},
            "profile_summary": "Apertura: alta creativo. Responsabilidad: equilibrada. Extraversión: alta. Amabilidad: equilibrada. Neuroticismo: equilibrado."
        });
        let pdf = generate_report("big_five", &result, "es");
        assert!(pdf.len() > 1000);
        assert_eq!(&pdf[..5], b"%PDF-");
    }

    #[test]
    fn test_generate_enneagram_pdf() {
        let result = serde_json::json!({
            "dominant_type": 4,
            "wing": 5,
            "scores": {
                "1": 30.0, "2": 45.0, "3": 40.0, "4": 85.0, "5": 70.0,
                "6": 35.0, "7": 50.0, "8": 25.0, "9": 55.0
            },
            "profile_summary": "Type 4w5. Highest scores: Type 4 (85%), Type 5 (70%)."
        });
        let pdf = generate_report("enneagram", &result, "en");
        assert!(pdf.len() > 1000);
        assert_eq!(&pdf[..5], b"%PDF-");
    }
}
