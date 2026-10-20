// Gerador mínimo de PDF só para os testes (cópia de `chamados-app::pdf`), incluído por `#[path]`.
#![allow(dead_code)]
//! Texto simples em PDF, para anexar a um chamado o que veio como texto (o SUAP não aceita `.txt`).
//!
//! Um gerador mínimo, sem dependências: páginas A4 só com texto em Helvetica (codificação
//! WinAnsi), com quebra de linha por palavras e várias páginas quando o texto é longo. Caracteres
//! que a WinAnsi não tem viram `?`.

const PAGE_WIDTH: u32 = 595;
const PAGE_HEIGHT: u32 = 842;
const MARGIN: u32 = 56;
const BODY_SIZE: u32 = 11;
const TITLE_SIZE: u32 = 14;
const LEADING: u32 = 14;
/// Characters per line: the text width over the average width of a Helvetica character at 11 pt.
const COLUMNS: usize = 86;
const LINES_PER_PAGE: usize = ((PAGE_HEIGHT - 2 * MARGIN) / LEADING) as usize;

/// The byte WinAnsi uses for `c`, or `?` when it has none.
fn win_ansi(c: char) -> u8 {
    match c {
        '\u{20}'..='\u{7e}' | '\u{a0}'..='\u{ff}' => c as u8,
        '€' => 0x80,
        '…' => 0x85,
        '‘' => 0x91,
        '’' => 0x92,
        '“' => 0x93,
        '”' => 0x94,
        '•' => 0x95,
        '–' => 0x96,
        '—' => 0x97,
        _ => b'?',
    }
}

/// A PDF string literal with the bytes of `line` (parentheses and backslashes escaped).
fn literal(line: &str) -> Vec<u8> {
    let mut out = vec![b'('];
    for byte in line.chars().map(win_ansi) {
        if matches!(byte, b'(' | b')' | b'\\') {
            out.push(b'\\');
        }
        out.push(byte);
    }
    out.push(b')');
    out
}

/// Breaks `text` into lines of at most [`COLUMNS`] characters, at spaces when possible.
fn wrap(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.replace('\r', "").replace('\t', "    ").split('\n') {
        let mut line = String::new();
        for word in paragraph.split(' ') {
            let mut word = word.to_owned();
            while word.chars().count() > COLUMNS {
                if !line.is_empty() {
                    lines.push(std::mem::take(&mut line));
                }
                let head: String = word.chars().take(COLUMNS).collect();
                word = word.chars().skip(COLUMNS).collect();
                lines.push(head);
            }
            let needed =
                line.chars().count() + usize::from(!line.is_empty()) + word.chars().count();
            if needed > COLUMNS && !line.is_empty() {
                lines.push(std::mem::take(&mut line));
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(&word);
        }
        lines.push(line);
    }
    lines
}

/// The content stream of one page: the title (first page only) and its lines of text.
fn page_stream(title: Option<&str>, lines: &[String]) -> Vec<u8> {
    let top = PAGE_HEIGHT - MARGIN - BODY_SIZE;
    let mut stream = format!("BT\n{MARGIN} {top} Td\n{LEADING} TL\n").into_bytes();
    if let Some(title) = title {
        stream.extend(format!("/F2 {TITLE_SIZE} Tf\n").into_bytes());
        stream.extend(literal(title));
        stream.extend(b" Tj\nT*\nT*\n");
    }
    stream.extend(format!("/F1 {BODY_SIZE} Tf\n").into_bytes());
    for line in lines {
        stream.extend(literal(line));
        stream.extend(b" Tj\nT*\n");
    }
    stream.extend(b"ET");
    stream
}

/// A PDF with `text` (and `title` in bold above it, when given).
pub fn text_to_pdf(title: Option<&str>, text: &str) -> Vec<u8> {
    let lines = wrap(text.trim_end_matches(['\r', '\n']));
    // The title takes three lines of the first page (itself and two of spacing).
    let first_capacity = LINES_PER_PAGE - if title.is_some() { 3 } else { 0 };
    let mut pages: Vec<(Option<&str>, &[String])> = Vec::new();
    let split = first_capacity.min(lines.len());
    pages.push((title, &lines[..split]));
    for chunk in lines[split..].chunks(LINES_PER_PAGE) {
        pages.push((None, chunk));
    }

    // Objects: 1 catalog, 2 page tree, 3 and 4 fonts, then a page and its content per page.
    let mut objects: Vec<Vec<u8>> = vec![Vec::new(); 4 + 2 * pages.len()];
    let kids: Vec<String> = (0..pages.len())
        .map(|n| format!("{} 0 R", 5 + 2 * n))
        .collect();
    objects[0] = b"<< /Type /Catalog /Pages 2 0 R >>".to_vec();
    objects[1] = format!(
        "<< /Type /Pages /Kids [{}] /Count {} >>",
        kids.join(" "),
        pages.len()
    )
    .into_bytes();
    objects[2] =
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>"
            .to_vec();
    objects[3] =
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold /Encoding /WinAnsiEncoding >>"
            .to_vec();
    for (n, (page_title, page_lines)) in pages.iter().enumerate() {
        let (page, content) = (4 + 2 * n, 5 + 2 * n);
        objects[page] = format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {PAGE_WIDTH} {PAGE_HEIGHT}] \
             /Resources << /Font << /F1 3 0 R /F2 4 0 R >> >> /Contents {} 0 R >>",
            content + 1
        )
        .into_bytes();
        let stream = page_stream(*page_title, page_lines);
        let mut body = format!("<< /Length {} >>\nstream\n", stream.len()).into_bytes();
        body.extend(stream);
        body.extend(b"\nendstream");
        objects[content] = body;
    }

    let mut pdf = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend(format!("{} 0 obj\n", index + 1).into_bytes());
        pdf.extend(object);
        pdf.extend(b"\nendobj\n");
    }
    let xref = pdf.len();
    pdf.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).into_bytes());
    for offset in offsets {
        pdf.extend(format!("{offset:010} 00000 n \n").into_bytes());
    }
    pdf.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .into_bytes(),
    );
    pdf
}
