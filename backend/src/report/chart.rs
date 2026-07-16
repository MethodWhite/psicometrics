use printpdf::*;

/// Extract (label, score_0_100) pairs from the result JSON.
pub(crate) fn extract_scores(test_type: &str, result: &serde_json::Value) -> Vec<(String, f64)> {
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
pub(crate) fn draw_bar_chart(
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
