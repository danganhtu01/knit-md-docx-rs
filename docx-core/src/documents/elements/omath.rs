use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::{XMLBuilder, XmlEvent};

/// A single node in an OMML (Office Math Markup Language) expression tree.
///
/// These map directly onto the `m:` elements Word uses for native equations.
/// Build a tree of these and wrap it in an [`OMath`] to emit a real Word
/// equation object (not just styled text), which Word's equation editor can
/// open and edit.
#[derive(Debug, Clone, PartialEq)]
pub enum OMathElement {
    /// A literal math run: `<m:r><m:t>…</m:t></m:r>`.
    Run(String),
    /// Superscript: `<m:sSup>` with base `<m:e>` and `<m:sup>`.
    SuperScript {
        base: Vec<OMathElement>,
        sup: Vec<OMathElement>,
    },
    /// Subscript: `<m:sSub>` with base `<m:e>` and `<m:sub>`.
    SubScript {
        base: Vec<OMathElement>,
        sub: Vec<OMathElement>,
    },
    /// Combined sub+superscript: `<m:sSubSup>`.
    SubSuperScript {
        base: Vec<OMathElement>,
        sub: Vec<OMathElement>,
        sup: Vec<OMathElement>,
    },
    /// Fraction: `<m:f>` with `<m:num>` over `<m:den>`.
    Fraction {
        numerator: Vec<OMathElement>,
        denominator: Vec<OMathElement>,
    },
    /// Radical: `<m:rad>` with optional `<m:deg>` and `<m:e>` (radicand).
    /// `degree == None` renders a plain square root (`degHide`).
    Radical {
        degree: Option<Vec<OMathElement>>,
        radicand: Vec<OMathElement>,
    },
    /// N-ary operator (sum, product, integral): `<m:nary>` with an operator
    /// glyph, optional `<m:sub>`/`<m:sup>` limits, and a body `<m:e>`.
    Nary {
        operator: String,
        sub: Vec<OMathElement>,
        sup: Vec<OMathElement>,
        body: Vec<OMathElement>,
    },
    /// Delimited group (parentheses/brackets/braces): `<m:d>`.
    Delimited {
        begin: String,
        end: String,
        body: Vec<OMathElement>,
    },
    /// Named function (`sin`, `lim`, …): `<m:func>` with `<m:fName>` and `<m:e>`.
    Function {
        name: Vec<OMathElement>,
        body: Vec<OMathElement>,
    },
}

fn build_elements<W: Write>(
    mut b: XMLBuilder<W>,
    elements: &[OMathElement],
) -> crate::xml::writer::Result<XMLBuilder<W>> {
    for e in elements {
        b = XMLBuilder::from(e.build_to(b.into_inner()?)?);
    }
    Ok(b)
}

/// Emit `<m:e>elements…</m:e>` (an "argument" slot used by most OMML elements).
fn build_arg<W: Write>(
    b: XMLBuilder<W>,
    tag: &str,
    elements: &[OMathElement],
) -> crate::xml::writer::Result<XMLBuilder<W>> {
    let b = b.write(XmlEvent::start_element(tag))?;
    let b = build_elements(b, elements)?;
    b.close()
}

impl BuildXML for OMathElement {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let b = XMLBuilder::from(stream);
        let b = match self {
            OMathElement::Run(text) => b
                .write(XmlEvent::start_element("m:r"))?
                .write(XmlEvent::start_element("m:t").attr("xml:space", "preserve"))?
                // Character data must be escaped: the shared writer emits text
                // verbatim (perform_escaping == false), so `<`, `>`, `&` in math
                // would otherwise produce malformed XML and a Word repair prompt.
                .write(crate::escape::escape(text).as_str())?
                .close()? // m:t
                .close()?, // m:r
            OMathElement::SuperScript { base, sup } => {
                let b = b.write(XmlEvent::start_element("m:sSup"))?;
                let b = build_arg(b, "m:e", base)?;
                let b = build_arg(b, "m:sup", sup)?;
                b.close()?
            }
            OMathElement::SubScript { base, sub } => {
                let b = b.write(XmlEvent::start_element("m:sSub"))?;
                let b = build_arg(b, "m:e", base)?;
                let b = build_arg(b, "m:sub", sub)?;
                b.close()?
            }
            OMathElement::SubSuperScript { base, sub, sup } => {
                let b = b.write(XmlEvent::start_element("m:sSubSup"))?;
                let b = build_arg(b, "m:e", base)?;
                let b = build_arg(b, "m:sub", sub)?;
                let b = build_arg(b, "m:sup", sup)?;
                b.close()?
            }
            OMathElement::Fraction {
                numerator,
                denominator,
            } => {
                let b = b.write(XmlEvent::start_element("m:f"))?;
                let b = build_arg(b, "m:num", numerator)?;
                let b = build_arg(b, "m:den", denominator)?;
                b.close()?
            }
            OMathElement::Radical { degree, radicand } => {
                let b = b.write(XmlEvent::start_element("m:rad"))?;
                // radPr: hide the degree slot for a plain square root.
                let b = b.write(XmlEvent::start_element("m:radPr"))?;
                let b = b
                    .write(
                        XmlEvent::start_element("m:degHide")
                            .attr("m:val", if degree.is_some() { "0" } else { "1" }),
                    )?
                    .close()?; // m:degHide
                let b = b.close()?; // m:radPr
                let b = match degree {
                    Some(deg) => build_arg(b, "m:deg", deg)?,
                    // Word still expects an (empty) deg slot when hidden.
                    None => b.write(XmlEvent::start_element("m:deg"))?.close()?,
                };
                let b = build_arg(b, "m:e", radicand)?;
                b.close()?
            }
            OMathElement::Nary {
                operator,
                sub,
                sup,
                body,
            } => {
                let b = b.write(XmlEvent::start_element("m:nary"))?;
                // naryPr: operator glyph; hide unused sub/sup slots.
                let b = b.write(XmlEvent::start_element("m:naryPr"))?;
                // Attribute values are written verbatim by the writer too, so a
                // delimiter/operator glyph of `<`/`>`/`&` must be escaped.
                let chr = crate::escape::escape(operator);
                let b = b
                    .write(XmlEvent::start_element("m:chr").attr("m:val", chr.as_str()))?
                    .close()?;
                let b = b
                    .write(
                        XmlEvent::start_element("m:subHide")
                            .attr("m:val", if sub.is_empty() { "1" } else { "0" }),
                    )?
                    .close()?;
                let b = b
                    .write(
                        XmlEvent::start_element("m:supHide")
                            .attr("m:val", if sup.is_empty() { "1" } else { "0" }),
                    )?
                    .close()?;
                let b = b.close()?; // m:naryPr
                let b = build_arg(b, "m:sub", sub)?;
                let b = build_arg(b, "m:sup", sup)?;
                let b = build_arg(b, "m:e", body)?;
                b.close()?
            }
            OMathElement::Delimited { begin, end, body } => {
                let b = b.write(XmlEvent::start_element("m:d"))?;
                let b = b.write(XmlEvent::start_element("m:dPr"))?;
                // Empty `m:val=""` is valid and means "invisible delimiter"
                // (e.g. `\left. \right.`); escape so a literal `<`/`>` glyph is
                // well-formed.
                let beg = crate::escape::escape(begin);
                let end_ = crate::escape::escape(end);
                let b = b
                    .write(XmlEvent::start_element("m:begChr").attr("m:val", beg.as_str()))?
                    .close()?;
                let b = b
                    .write(XmlEvent::start_element("m:endChr").attr("m:val", end_.as_str()))?
                    .close()?;
                let b = b.close()?; // m:dPr
                let b = build_arg(b, "m:e", body)?;
                b.close()?
            }
            OMathElement::Function { name, body } => {
                let b = b.write(XmlEvent::start_element("m:func"))?;
                let b = build_arg(b, "m:fName", name)?;
                let b = build_arg(b, "m:e", body)?;
                b.close()?
            }
        };
        b.into_inner()
    }
}

/// A native Word math equation (`<m:oMath>`).
///
/// Holds a tree of [`OMathElement`]s. Construct directly with [`OMath::new`] for
/// a single literal run, or with [`OMath::from_elements`] for a structured
/// expression. Set [`OMath::display`] for a block (centred) equation wrapped in
/// `<m:oMathPara>`; otherwise it is inline.
#[derive(Debug, Clone, PartialEq)]
pub struct OMath {
    pub elements: Vec<OMathElement>,
    pub display: bool,
}

impl OMath {
    /// A single literal run of math text.
    pub fn new(content: impl Into<String>) -> Self {
        OMath {
            elements: vec![OMathElement::Run(content.into())],
            display: false,
        }
    }

    /// A structured expression from an explicit element tree.
    pub fn from_elements(elements: Vec<OMathElement>) -> Self {
        OMath {
            elements,
            display: false,
        }
    }

    /// Mark this as a display (block) equation.
    pub fn display(mut self) -> Self {
        self.display = true;
        self
    }
}

impl Serialize for OMath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("OMath", 1)?;
        t.serialize_field("display", &self.display)?;
        t.end()
    }
}

impl BuildXML for OMath {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let mut b = XMLBuilder::from(stream);
        if self.display {
            b = b.write(XmlEvent::start_element("m:oMathPara"))?;
        }
        b = b.write(XmlEvent::start_element("m:oMath"))?;
        b = build_elements(b, &self.elements)?;
        b = b.close()?; // m:oMath
        if self.display {
            b = b.close()?; // m:oMathPara
        }
        b.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::documents::BuildXML;
    use std::str;

    #[test]
    fn test_omath_run() {
        let b = OMath::new("E=mc").build();
        let s = str::from_utf8(&b).unwrap();
        assert!(s.contains("<m:oMath>"), "oMath element");
        assert!(s.contains("<m:r><m:t xml:space=\"preserve\">E=mc</m:t></m:r>"));
        assert!(!s.contains("m:oMathPara"), "inline by default");
    }

    #[test]
    fn test_omath_superscript() {
        let math = OMath::from_elements(vec![OMathElement::SuperScript {
            base: vec![OMathElement::Run("x".into())],
            sup: vec![OMathElement::Run("2".into())],
        }]);
        let s = str::from_utf8(&math.build()).unwrap().to_string();
        assert!(s.contains("<m:sSup>"));
        assert!(s.contains("<m:e>"));
        assert!(s.contains("<m:sup>"));
    }

    #[test]
    fn test_omath_fraction_display() {
        let math = OMath::from_elements(vec![OMathElement::Fraction {
            numerator: vec![OMathElement::Run("1".into())],
            denominator: vec![OMathElement::Run("2".into())],
        }])
        .display();
        let s = str::from_utf8(&math.build()).unwrap().to_string();
        assert!(s.contains("<m:oMathPara>"));
        assert!(s.contains("<m:f>"));
        assert!(s.contains("<m:num>"));
        assert!(s.contains("<m:den>"));
    }

    #[test]
    fn test_omath_radical_sqrt_hides_degree() {
        let math = OMath::from_elements(vec![OMathElement::Radical {
            degree: None,
            radicand: vec![OMathElement::Run("2".into())],
        }]);
        let s = str::from_utf8(&math.build()).unwrap().to_string();
        assert!(s.contains("<m:rad>"));
        assert!(s.contains("m:degHide"));
        assert!(s.contains("m:val=\"1\""), "sqrt hides the degree");
    }

    #[test]
    fn test_omath_nary_sum() {
        let math = OMath::from_elements(vec![OMathElement::Nary {
            operator: "\u{2211}".into(),
            sub: vec![OMathElement::Run("i=1".into())],
            sup: vec![OMathElement::Run("n".into())],
            body: vec![OMathElement::Run("i".into())],
        }]);
        let s = str::from_utf8(&math.build()).unwrap().to_string();
        assert!(s.contains("<m:nary>"));
        assert!(s.contains("m:chr"));
        assert!(s.contains("<m:sub>"));
        assert!(s.contains("<m:sup>"));
    }

    #[test]
    fn test_omath_run_escapes_xml_metacharacters() {
        // `<`, `>`, `&` in math text must be escaped or document.xml is
        // malformed and Word offers to repair (dropping the equation).
        let s = str::from_utf8(&OMath::new("a < b & c > d").build())
            .unwrap()
            .to_string();
        assert!(s.contains("a &lt; b &amp; c &gt; d"), "got: {s}");
        assert!(!s.contains("a < b"), "raw < must not survive");
    }

    #[test]
    fn test_omath_delimiter_attr_is_escaped() {
        // A user-supplied delimiter glyph of `<`/`>` must be escaped in m:val.
        let math = OMath::from_elements(vec![OMathElement::Delimited {
            begin: "<".into(),
            end: ">".into(),
            body: vec![OMathElement::Run("x".into())],
        }]);
        let s = str::from_utf8(&math.build()).unwrap().to_string();
        assert!(s.contains("m:val=\"&lt;\""), "begChr escaped: {s}");
        assert!(s.contains("m:val=\"&gt;\""), "endChr escaped");
    }
}
