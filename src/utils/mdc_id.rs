use leptos::{Attribute, IntoAttribute, Scope};
use std::{
    fmt::{self, Debug, Display, Formatter},
    sync::atomic::{AtomicU64, Ordering::Relaxed},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct MDCId(u64);

impl Debug for MDCId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for MDCId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "mdc-{}", self.0)
    }
}

impl From<MDCId> for u64 {
    fn from(value: MDCId) -> Self {
        value.0
    }
}

impl IntoAttribute for MDCId {
    fn into_attribute(self, _cx: Scope) -> Attribute {
        Attribute::String(self.to_string())
    }

    fn into_attribute_boxed(self: Box<Self>, _cx: Scope) -> Attribute {
        Attribute::String(self.to_string())
    }
}

pub fn mdc_id() -> MDCId {
    static ID: AtomicU64 = AtomicU64::new(0);
    MDCId(ID.fetch_add(1, Relaxed))
}
