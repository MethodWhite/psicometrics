use printpdf::*;

pub(crate) fn get_report_title(test_type: &str, lang: &str) -> String {
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

/// Draw test-specific textual info (type code, dominant type, etc.).
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_additional_info(
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

/// Draw the footer disclaimer lines.
pub(crate) fn draw_footer_disclaimer(
    layer: &PdfLayerReference,
    font_italic: &IndirectFontRef,
    language: &str,
) {
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
    let _ = layer.use_text(disc1, 7.0, Mm(20.0), disc_footer_y, font_italic);
    let _ = layer.use_text(disc2, 7.0, Mm(20.0), disc_footer_y - Mm(4.0), font_italic);
}
