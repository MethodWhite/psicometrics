"""PDF report generation for psicometric test results.

Uses reportlab to produce an A4 PDF with title, date, scores,
bar charts, summary text, test-specific information, and a
disclaimer footer.
"""

from __future__ import annotations

import io
from datetime import date
from typing import Any

from reportlab.lib import colors
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import inch, mm
from reportlab.platypus import (
    Frame,
    Image,
    PageTemplate,
    Paragraph,
    SimpleDocTemplate,
    Spacer,
    Table,
    TableStyle,
)

PAGE_WIDTH, PAGE_HEIGHT = A4
MARGIN = 20 * mm

TEST_NAMES: dict[str, dict[str, str]] = {
    "big_five": {
        "es": "Big Five / OCEAN",
        "en": "Big Five / OCEAN",
    },
    "mbti": {
        "es": "MBTI — Myers-Briggs",
        "en": "MBTI — Myers-Briggs",
    },
    "disc": {
        "es": "Perfil DISC",
        "en": "DISC Profile",
    },
    "enneagram": {
        "es": "Enneagrama",
        "en": "Enneagram",
    },
    "eq": {
        "es": "Inteligencia Emocional",
        "en": "Emotional Intelligence",
    },
    "dark_triad": {
        "es": "Tríada Oscura",
        "en": "Dark Triad",
    },
    "attachment_style": {
        "es": "Estilo de Apego",
        "en": "Attachment Style",
    },
    "love_languages": {
        "es": "Lenguajes del Amor",
        "en": "Love Languages",
    },
    "career_aptitude": {
        "es": "Aptitud Profesional",
        "en": "Career Aptitude",
    },
    "via_strengths": {
        "es": "Fortalezas VIA",
        "en": "VIA Character Strengths",
    },
    "human_design": {
        "es": "Diseño Humano",
        "en": "Human Design",
    },
}

BIG_FIVE_FACTOR_LABELS: dict[str, dict[str, str]] = {
    "es": {
        "O": "Apertura",
        "C": "Responsabilidad",
        "E": "Extraversión",
        "A": "Amabilidad",
        "N": "Neuroticismo",
    },
    "en": {
        "O": "Openness",
        "C": "Conscientiousness",
        "E": "Extraversion",
        "A": "Agreeableness",
        "N": "Neuroticism",
    },
}


def _build_header_story(story: list, test_name: str, lang: str) -> None:
    styles = getSampleStyleSheet()
    title_style = ParagraphStyle(
        "ReportTitle", parent=styles["Title"],
        fontSize=20, leading=24, spaceAfter=6,
        textColor=colors.HexColor("#2c3e50"),
    )
    story.append(Paragraph(test_name, title_style))
    date_str = date.today().strftime("%d/%m/%Y")
    subtitle = ParagraphStyle(
        "DateLine", parent=styles["Normal"],
        fontSize=10, textColor=colors.gray,
    )
    if lang == "es":
        story.append(Paragraph(f"Fecha del reporte: {date_str}", subtitle))
    else:
        story.append(Paragraph(f"Report date: {date_str}", subtitle))
    story.append(Spacer(1, 6 * mm))


def _build_big_five_chart(story: list, scores: dict[str, float],
                          lang: str) -> None:
    """Add a Big Five bar chart as a styled table."""
    labels = BIG_FIVE_FACTOR_LABELS.get(lang, BIG_FIVE_FACTOR_LABELS["en"])
    factors = ["O", "C", "E", "A", "N"]
    bar_colors = ["#3498db", "#2ecc71", "#e74c3c", "#f39c12", "#9b59b6"]

    data = [
        [Paragraph(
            f"<b>{labels[f]}</b>", getSampleStyleSheet()["Normal"]
        ) for f in factors],
    ]
    # Bar row: simulate bars with colored cells of proportional width
    bar_row: list = []
    for i, f in enumerate(factors):
        val = scores.get(f, 0)
        pct = max(5, min(100, val))  # clamp for display
        bar_cell = (
            f'<font color="{bar_colors[i]}">'
            f'{"█" * int(pct / 5)}</font>'
            f'<br/><b>{val}</b>'
        )
        bar_row.append(Paragraph(bar_cell, getSampleStyleSheet()["Normal"]))
    data.append(bar_row)

    col_widths = [PAGE_WIDTH / 6] * 5
    t = Table(data, colWidths=col_widths)
    t.setStyle(TableStyle([
        ("ALIGN", (0, 0), (-1, -1), "CENTER"),
        ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
        ("TOPPADDING", (0, 0), (-1, -1), 4),
        ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
        ("FONTSIZE", (0, 0), (-1, -1), 9),
    ]))
    story.append(t)
    story.append(Spacer(1, 4 * mm))


def _build_summary_text(story: list, result: dict[str, Any],
                        test_type: str, lang: str) -> None:
    """Add a textual summary of the results."""
    styles = getSampleStyleSheet()
    body = ParagraphStyle(
        "BodyText2", parent=styles["Normal"],
        fontSize=10, leading=14,
    )

    summary = result.get("profile_summary") or result.get("summary", "")
    if summary:
        # Clean up any markdown bold into reportlab <b> tags
        cleaned = summary.replace("**", "<b>").replace("**", "</b>")
        # Handle remaining markdown bold pairs (odd number edge case)
        cleaned = cleaned.replace("**", "<b>")
        story.append(Paragraph(cleaned, body))
        story.append(Spacer(1, 4 * mm))

    # Test-specific info sections
    if test_type == "big_five":
        _big_five_detail(story, result, lang)
    elif test_type == "mbti":
        _mbti_detail(story, result, lang)


def _big_five_detail(story: list, result: dict[str, Any],
                     lang: str) -> None:
    """Big Five per-factor breakdown."""
    styles = getSampleStyleSheet()
    body = ParagraphStyle("Small", parent=styles["Normal"],
                          fontSize=9, leading=12)
    labels = BIG_FIVE_FACTOR_LABELS.get(lang, BIG_FIVE_FACTOR_LABELS["en"])
    scores = result.get("scores", {})
    facets = result.get("facets", {})

    if lang == "es":
        story.append(Paragraph("<b>Desglose por Factor</b>", body))
    else:
        story.append(Paragraph("<b>Per-Factor Breakdown</b>", body))

    for f in ["O", "C", "E", "A", "N"]:
        score = scores.get(f, 0)
        text = f"<b>{labels[f]}:</b> {score}/100"
        # Show top facets if available
        facet_texts = []
        for f_key, f_val in facets.items():
            if f_key.startswith(f) and isinstance(f_val, (int, float)):
                facet_texts.append(f"{f_key}: {f_val}")
        if facet_texts:
            text += " — " + ", ".join(facet_texts[:3])
        story.append(Paragraph(text, body))
    story.append(Spacer(1, 3 * mm))


def _mbti_detail(story: list, result: dict[str, Any],
                 lang: str) -> None:
    """MBTI type and dichotomy breakdown."""
    styles = getSampleStyleSheet()
    body = ParagraphStyle("Small", parent=styles["Normal"],
                          fontSize=9, leading=12)
    mbti_type = result.get("type", "N/A")
    scores = result.get("scores", {})

    if lang == "es":
        story.append(Paragraph(f"<b>Tipo MBTI:</b> {mbti_type}", body))
        story.append(Paragraph("<b>Puntajes por Dicotomía</b>", body))
    else:
        story.append(Paragraph(f"<b>MBTI Type:</b> {mbti_type}", body))
        story.append(Paragraph("<b>Dichotomy Scores</b>", body))

    for dim in ["EI", "SN", "TF", "JP"]:
        val = scores.get(dim, {})
        if isinstance(val, dict):
            e = val.get("E", 0)
            i = val.get("I", 0)
            story.append(Paragraph(f"{dim}: E={e}, I={i}", body))
        elif isinstance(val, (int, float)):
            story.append(Paragraph(f"{dim}: {val}", body))
    story.append(Spacer(1, 3 * mm))


def _build_footer(canvas: object, doc: object) -> None:
    """Footer with disclaimer."""
    canvas.saveState()
    canvas.setFont("Helvetica", 7)
    canvas.setFillColor(colors.Color(0.4, 0.4, 0.4))

    disclaimer = (
        "Psicometrics — Este reporte es solo para fines informativos "
        "y educativos. No constituye un diagnóstico profesional. "
        "| Psicometrics — This report is for informational and "
        "educational purposes only. It does not constitute a "
        "professional diagnosis."
    )
    w = PAGE_WIDTH - 2 * MARGIN
    canvas.drawCentredString(PAGE_WIDTH / 2, 12 * mm, disclaimer)

    page_num = f"Página {doc.page}"
    canvas.drawRightString(PAGE_WIDTH - MARGIN, 12 * mm, page_num)
    canvas.restoreState()


def generate_report(
    test_type: str,
    result: dict[str, Any],
    lang: str = "es",
) -> bytes:
    """Generate an A4 PDF report for a psicometric test result.

    Returns
    -------
    bytes of the generated PDF.
    """
    buf = io.BytesIO()
    doc = SimpleDocTemplate(
        buf,
        pagesize=A4,
        leftMargin=MARGIN,
        rightMargin=MARGIN,
        topMargin=MARGIN,
        bottomMargin=MARGIN + 10 * mm,
    )

    frame = Frame(
        MARGIN, MARGIN,
        PAGE_WIDTH - 2 * MARGIN,
        PAGE_HEIGHT - 2 * MARGIN - 8 * mm,
        id="main",
    )
    template = PageTemplate(id="report", frames=[frame],
                            onPage=_build_footer)
    doc.addPageTemplates([template])

    story: list = []

    test_name = (
        TEST_NAMES.get(test_type, {})
        .get(lang, f"Test: {test_type}")
    )
    _build_header_story(story, test_name, lang)

    # Bar chart for Big Five scores
    if test_type == "big_five" and "scores" in result:
        _build_big_five_chart(story, result["scores"], lang)

    # Summary and test-specific info
    _build_summary_text(story, result, test_type, lang)

    doc.build(story)
    pdf_bytes = buf.getvalue()
    buf.close()
    return pdf_bytes
