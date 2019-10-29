use crate::errors::LocaleError;
use crate::parser::ParserError;

use tinystr::TinyStr8;

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub struct PrivateExtensionList(Vec<TinyStr8>);

fn parse_value(t: &[u8]) -> Result<TinyStr8, ParserError> {
    let s = TinyStr8::from_bytes(t).map_err(|_| ParserError::InvalidSubtag)?;
    if t.is_empty() || t.len() > 8 || !s.is_ascii_alphanumeric() {
        return Err(ParserError::InvalidSubtag);
    }

    Ok(s.to_ascii_lowercase())
}

impl PrivateExtensionList {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn add_tag<S: AsRef<[u8]>>(&mut self, tag: S) -> Result<(), LocaleError> {
        self.0.push(parse_value(tag.as_ref())?);
        self.0.sort();
        Ok(())
    }

    pub(crate) fn try_from_iter<'a>(
        iter: &mut impl Iterator<Item = &'a [u8]>,
    ) -> Result<Self, ParserError> {
        let mut pext = Self::default();

        for subtag in iter {
            pext.0.push(parse_value(subtag)?);
        }
        pext.0.sort();

        Ok(pext)
    }
}

impl std::fmt::Display for PrivateExtensionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        f.write_str("-x")?;

        for subtag in &self.0 {
            write!(f, "-{}", subtag)?;
        }
        Ok(())
    }
}
