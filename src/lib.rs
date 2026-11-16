// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-animation-tool: animation tool component model.
//!
//! Real implementation: animations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Animation {
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
pub enum AnimationToolError {
    EmptyAnimationName,
    DuplicateAnimation(String),
    EmptyEventSubject,
}

impl fmt::Display for AnimationToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAnimationName => write!(f, "animation name must not be empty"),
            Self::DuplicateAnimation(id) => write!(f, "duplicate animation {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AnimationToolError {}

pub struct AnimationTool {
    pub name: String,
    animations: BTreeMap<String, Animation>,
    events: BTreeMap<String, Event>,
}

impl AnimationTool {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            animations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_animation(&mut self, item: Animation) -> Result<(), AnimationToolError> {
        if item.name.trim().is_empty() {
            return Err(AnimationToolError::EmptyAnimationName);
        }
        let id = item.id.clone();
        if self.animations.contains_key(&id) {
            return Err(AnimationToolError::DuplicateAnimation(id.clone()));
        }
        self.animations.insert(id, item);
        Ok(())
    }

    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    pub fn get_animation(&self, id: &str) -> Option<&Animation> {
        self.animations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AnimationToolError> {
        if event.subject.trim().is_empty() {
            return Err(AnimationToolError::EmptyEventSubject);
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

    fn sample_animation(id: &str, name: &str) -> Animation {
        Animation {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = AnimationTool::new("AnimationTool");
        assert_eq!(m.name, "AnimationTool");
        assert_eq!(m.animation_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_animation() {
        let mut m = AnimationTool::new("AnimationTool");
        m.add_animation(sample_animation("a1", "Alpha")).unwrap();
        assert_eq!(m.animation_count(), 1);
        assert!(m.get_animation("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = AnimationTool::new("AnimationTool");
        assert!(m.add_animation(sample_animation("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = AnimationTool::new("AnimationTool");
        m.add_animation(sample_animation("x", "One")).unwrap();
        assert!(m.add_animation(sample_animation("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = AnimationTool::new("AnimationTool");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
