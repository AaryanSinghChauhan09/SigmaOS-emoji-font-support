// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-emoji-font-support: emoji font support component model.
//!
//! Real implementation: fonts, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmojiFontSupportError {
    EmptyFontName,
    DuplicateFont(String),
    EmptyEventSubject,
}

impl fmt::Display for EmojiFontSupportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFontName => write!(f, "font name must not be empty"),
            Self::DuplicateFont(id) => write!(f, "duplicate font {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EmojiFontSupportError {}

pub struct EmojiFontSupport {
    pub name: String,
    fonts: BTreeMap<String, Font>,
    events: BTreeMap<String, Event>,
}

impl EmojiFontSupport {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fonts: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_font(&mut self, item: Font) -> Result<(), EmojiFontSupportError> {
        if item.name.trim().is_empty() {
            return Err(EmojiFontSupportError::EmptyFontName);
        }
        let id = item.id.clone();
        if self.fonts.contains_key(&id) {
            return Err(EmojiFontSupportError::DuplicateFont(id.clone()));
        }
        self.fonts.insert(id, item);
        Ok(())
    }

    pub fn font_count(&self) -> usize {
        self.fonts.len()
    }

    pub fn get_font(&self, id: &str) -> Option<&Font> {
        self.fonts.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EmojiFontSupportError> {
        if event.subject.trim().is_empty() {
            return Err(EmojiFontSupportError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_font(id: &str, name: &str) -> Font {
        Font {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EmojiFontSupport::new("EmojiFontSupport");
        assert_eq!(m.name, "EmojiFontSupport");
        assert_eq!(m.font_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_font() {
        let mut m = EmojiFontSupport::new("EmojiFontSupport");
        m.add_font(sample_font("a1", "Alpha")).unwrap();
        assert_eq!(m.font_count(), 1);
        assert!(m.get_font("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EmojiFontSupport::new("EmojiFontSupport");
        assert!(m.add_font(sample_font("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EmojiFontSupport::new("EmojiFontSupport");
        m.add_font(sample_font("x", "One")).unwrap();
        assert!(m.add_font(sample_font("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EmojiFontSupport::new("EmojiFontSupport");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
