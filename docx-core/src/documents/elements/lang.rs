use serde::Serialize;
use std::io::Write;

use crate::documents::BuildXML;
use crate::xml_builder::*;

/// `w:lang`: the languages a run's text is in, one per script class. Word and
/// LibreOffice choose spelling, hyphenation and line-breaking rules by it.
/// Values are BCP 47 tags such as `en-US`, `de-DE` or `it-IT`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lang {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub east_asia: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidi: Option<String>,
}

impl Lang {
    /// The language of Latin (and other non-East-Asian, non-complex) text.
    pub fn new(val: impl Into<String>) -> Lang {
        Lang {
            val: Some(val.into()),
            ..Default::default()
        }
    }

    pub fn east_asia(mut self, v: impl Into<String>) -> Self {
        self.east_asia = Some(v.into());
        self
    }

    pub fn bidi(mut self, v: impl Into<String>) -> Self {
        self.bidi = Some(v.into());
        self
    }
}

impl BuildXML for Lang {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .lang(
                self.val.as_ref(),
                self.east_asia.as_ref(),
                self.bidi.as_ref(),
            )?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_lang() {
        let b = Lang::new("de-DE").build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:lang w:val="de-DE" />"#);
    }

    #[test]
    fn test_lang_all_scripts() {
        let b = Lang::new("it-IT").east_asia("ja-JP").bidi("ar-SA").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:lang w:val="it-IT" w:eastAsia="ja-JP" w:bidi="ar-SA" />"#
        );
    }

    #[test]
    fn test_lang_json() {
        let l = Lang::new("en-US");
        assert_eq!(serde_json::to_string(&l).unwrap(), r#"{"val":"en-US"}"#);
    }
}
