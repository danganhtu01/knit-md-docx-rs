//! K-009: a .docx written by this crate is byte-stable.
//!
//! `document.xml.rels` once took its hyperlink relationships from a `HashMap`, and
//! `docProps/custom.xml` its properties from another, so the same document packed to different
//! bytes on every run. These tests pack one document more than once, in this process and in
//! fresh ones (a hash seed is drawn per process), and compare the bytes.

extern crate docx_rs;

use std::io::{Cursor, Read};
use std::process::Command;

use base64::Engine;
use docx_rs::*;

/// The hyperlinks of the fixed document, in document order. The numbers are not in numeric or
/// in text order, so only the document's own order can produce the expected list.
const LINK_NUMBERS: [usize; 8] = [10, 2, 33, 1, 7, 100, 4, 21];

/// Custom properties added in an order that is neither sorted nor reversed.
const PROPERTIES: [&str; 8] = [
    "zeta", "alpha", "mid", "omega", "beta", "gamma", "delta", "epsilon",
];

const ONE_PIXEL_PNG: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==";

fn rid(n: usize) -> String {
    format!("rIdHyperlink{n}")
}

fn url(n: usize) -> String {
    format!("https://example.com/page/{n}?a=1&b=2")
}

/// A link with a chosen relationship id: `Hyperlink::new` draws its id from a counter that
/// runs on for the life of the process, so two documents built in one process never share ids.
fn link_with_rid(n: usize) -> Hyperlink {
    let mut h = Hyperlink::new(url(n), HyperlinkType::External)
        .add_run(Run::new().add_text(format!("link {n}")));
    if let HyperlinkData::External { rid: id, .. } = &mut h.link {
        *id = rid(n);
    }
    h
}

fn paragraph_of(links: &[usize]) -> Paragraph {
    links.iter().fold(Paragraph::new(), |p, n| {
        p.add_hyperlink(link_with_rid(*n))
            .add_run(Run::new().add_text(" "))
    })
}

/// Eight links over a paragraph, a table cell and a paragraph again, with the first link of the
/// second paragraph written a second time (a document read from a file reuses one relationship
/// id for every link to one address), and properties in a fixed non-sorted order.
fn fixed_document() -> Docx {
    let table = Table::new(vec![TableRow::new(vec![
        TableCell::new().add_paragraph(paragraph_of(&[7, 100])),
    ])]);
    let docx = Docx::new()
        .add_paragraph(paragraph_of(&[10, 2, 33, 1]))
        .add_table(table)
        .add_paragraph(paragraph_of(&[4, 21, 2]));
    PROPERTIES.iter().fold(docx, |d, name| {
        d.custom_property(*name, format!("value of {name}"))
    })
}

fn pack(docx: Docx) -> Vec<u8> {
    let mut buf = Cursor::new(Vec::new());
    docx.build().pack(&mut buf).expect("pack");
    buf.into_inner()
}

fn part(bytes: &[u8], name: &str) -> String {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).expect("a zip");
    let mut file = zip.by_name(name).expect("the part is in the package");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("a utf-8 part");
    s
}

/// The values of one attribute, in the order they appear in an XML part.
fn attribute_values(xml: &str, attribute: &str) -> Vec<String> {
    let needle = format!(" {attribute}=\"");
    xml.split(needle.as_str())
        .skip(1)
        .filter_map(|rest| rest.split('"').next())
        .map(str::to_string)
        .collect()
}

fn hyperlink_ids(bytes: &[u8]) -> Vec<String> {
    attribute_values(&part(bytes, "word/_rels/document.xml.rels"), "Id")
        .into_iter()
        .filter(|id| id.starts_with("rIdHyperlink"))
        .collect()
}

#[test]
fn hyperlink_relationships_follow_the_document_order() {
    let expected: Vec<String> = LINK_NUMBERS.iter().map(|n| rid(*n)).collect();
    // Each build makes fresh hash maps, so a seed-ordered list would not hold for all of them.
    for _ in 0..8 {
        assert_eq!(hyperlink_ids(&pack(fixed_document())), expected);
    }
}

#[test]
fn a_relationship_id_met_twice_is_written_once() {
    let ids = hyperlink_ids(&pack(fixed_document()));
    assert_eq!(ids.iter().filter(|id| **id == rid(2)).count(), 1);
    assert_eq!(ids.len(), LINK_NUMBERS.len());
}

#[test]
fn custom_properties_are_written_in_name_order() {
    let mut sorted: Vec<String> = PROPERTIES.iter().map(|s| s.to_string()).collect();
    sorted.sort();
    for _ in 0..8 {
        let custom = part(&pack(fixed_document()), "docProps/custom.xml");
        assert_eq!(attribute_values(&custom, "name"), sorted);
        let pids: Vec<String> = (2..2 + sorted.len()).map(|n| n.to_string()).collect();
        assert_eq!(attribute_values(&custom, "pid"), pids);
    }
}

#[test]
fn one_document_packs_to_equal_bytes_in_one_process() {
    let first = pack(fixed_document());
    for _ in 0..8 {
        assert!(
            pack(fixed_document()) == first,
            "a second pack of one document differs from the first"
        );
    }
}

#[test]
fn the_zip_carries_one_fixed_timestamp() {
    let bytes = pack(fixed_document());
    let mut zip = zip::ZipArchive::new(Cursor::new(&bytes[..])).expect("a zip");
    let first = zip.by_index(0).expect("an entry").last_modified();
    assert_eq!(first, Some(zip::DateTime::default()));
    for i in 1..zip.len() {
        let entry = zip.by_index(i).expect("an entry");
        assert_eq!(entry.last_modified(), first, "entry {}", entry.name());
    }
}

/// The document of the separate-process test: real `Hyperlink::new` ids and the other parts
/// whose order could come from a map, so a map's seed would show in the bytes.
fn rich_document() -> Docx {
    let png = base64::engine::general_purpose::STANDARD
        .decode(ONE_PIXEL_PNG)
        .expect("base64 png");
    let hyperlink = |n: usize| {
        Hyperlink::new(url(n), HyperlinkType::External)
            .add_run(Run::new().add_text(format!("link {n}")))
    };
    let mut docx = Docx::new()
        .add_style(Style::new("Heading1", StyleType::Paragraph).name("Heading 1"))
        .add_table_of_contents(
            TableOfContents::new()
                .heading_styles_range(1, 3)
                .hyperlink()
                .alias("table of contents"),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Heading"))
                .style("Heading1"),
        )
        .header(Header::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("head"))))
        .footer(Footer::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("foot"))))
        .add_paragraph(
            Paragraph::new()
                .add_comment_start(
                    Comment::new(1)
                        .author("author")
                        .date("2026-01-01T00:00:00Z")
                        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("note"))),
                )
                .add_run(Run::new().add_text("commented"))
                .add_comment_end(1),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("numbered"))
                .numbering(NumberingId::new(2), IndentLevel::new(0)),
        )
        .add_abstract_numbering(AbstractNumbering::new(2).add_level(Level::new(
            0,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%1."),
            LevelJc::new("left"),
        )))
        .add_numbering(Numbering::new(2, 2))
        .add_paragraph(
            Paragraph::new().add_run(Run::new().add_text("pic").add_image(Pic::new(&png))),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_footnote_reference(
            Footnote::new().add_content(Paragraph::new().add_run(Run::new().add_text("fn"))),
        )));
    for group in [[5usize, 1, 9], [3, 8, 2]] {
        let p = group
            .iter()
            .fold(Paragraph::new(), |p, n| p.add_hyperlink(hyperlink(*n)));
        docx = docx.add_paragraph(p);
    }
    PROPERTIES.iter().fold(docx, |d, name| {
        d.custom_property(*name, format!("value of {name}"))
    })
}

const CHILD_OUT: &str = "K009_CHILD_OUT";
const CHILD_TEST: &str = "child_writes_package";

/// Run by `separate_processes_write_equal_bytes`, in a process of its own; on its own it does
/// nothing.
#[test]
fn child_writes_package() {
    if let Ok(path) = std::env::var(CHILD_OUT) {
        std::fs::write(path, pack(rich_document())).expect("write the package");
    }
}

#[test]
fn separate_processes_write_equal_bytes() {
    let exe = std::env::current_exe().expect("this test binary");
    let dir = std::env::temp_dir();
    let mut packages: Vec<Vec<u8>> = vec![];
    for i in 0..3 {
        let out = dir.join(format!("k009-{}-{i}.docx", std::process::id()));
        let _ = std::fs::remove_file(&out);
        let status = Command::new(&exe)
            .args(["--exact", CHILD_TEST, "--test-threads=1"])
            .env(CHILD_OUT, &out)
            .output()
            .expect("run the test binary again");
        assert!(status.status.success(), "child {i} failed: {status:?}");
        let bytes = std::fs::read(&out).expect("the child wrote its package");
        let _ = std::fs::remove_file(&out);
        assert!(!bytes.is_empty());
        packages.push(bytes);
    }
    // The rels part first, so a failure names the part and not 40 kB of zip.
    let rels: Vec<String> = packages
        .iter()
        .map(|b| part(b, "word/_rels/document.xml.rels"))
        .collect();
    assert_eq!(rels[0], rels[1]);
    assert_eq!(rels[0], rels[2]);
    let custom: Vec<String> = packages
        .iter()
        .map(|b| part(b, "docProps/custom.xml"))
        .collect();
    assert_eq!(custom[0], custom[1]);
    assert_eq!(custom[0], custom[2]);
    assert!(packages[0] == packages[1], "package 0 and 1 differ");
    assert!(packages[0] == packages[2], "package 0 and 2 differ");
}
