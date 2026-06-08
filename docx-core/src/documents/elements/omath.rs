use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::{XMLBuilder, XmlEvent};

/// A native Word math equation (`<m:oMath>`).
///
/// Wraps a plain-text expression in an OMML run using Word's equation font and
/// italic style. This does **not** perform LaTeX-to-OMML conversion — for simple
/// inline expressions (e.g. `E=mc²`, `a²+b²=c²`) the output renders correctly in
/// Word's equation editor. For structured fractions, integrals, etc., you would
/// need to build the OMML element tree manually.
///
/// Set `display = true` to get a block-level `<m:oMathPara>` wrapper (centred,
/// on its own line), or leave it `false` for inline use.
#[derive(Debug, Clone, PartialEq)]
pub struct OMath {
    /// The expression text placed inside `<m:t>`.
    pub content: String,
    /// If `true`, wrap in `<m:oMathPara>` for a display (block) equation.
    pub display: bool,
}

impl OMath {
    pub fn new(content: impl Into<String>) -> Self {
        OMath {
            content: content.into(),
            display: false,
        }
    }

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
        let mut t = serializer.serialize_struct("OMath", 2)?;
        t.serialize_field("content", &self.content)?;
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

        b = b
            // <m:oMath>
            .write(XmlEvent::start_element("m:oMath"))?
            //   <m:r>
            .write(XmlEvent::start_element("m:r"))?
            //     <m:rPr><m:sty m:val="i"/></m:rPr>
            .write(XmlEvent::start_element("m:rPr"))?
            .write(
                XmlEvent::start_element("m:sty").attr("m:val", "i"),
            )?
            .close()? // m:sty
            .close()? // m:rPr
            //     <m:t xml:space="preserve">...</m:t>
            .write(
                XmlEvent::start_element("m:t").attr("xml:space", "preserve"),
            )?
            .write(self.content.as_str())?
            .close()? // m:t
            .close()? // m:r
            .close()?; // m:oMath

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
    fn test_omath_inline() {
        let b = OMath::new("E=mc^2").build();
        let s = str::from_utf8(&b).unwrap();
        assert!(s.contains("<m:oMath>"), "oMath element");
        assert!(s.contains("E=mc^2"), "content preserved");
        assert!(!s.contains("m:oMathPara"), "no para wrapper for inline");
    }

    #[test]
    fn test_omath_display() {
        let b = OMath::new("x^2").display().build();
        let s = str::from_utf8(&b).unwrap();
        assert!(s.contains("<m:oMathPara>"), "para wrapper for display");
        assert!(s.contains("<m:oMath>"), "oMath element");
    }
}
