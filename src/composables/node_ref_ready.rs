use leptos::{html::ElementDescriptor, set_timeout, HtmlElement, NodeRef, Scope};
use std::time::Duration;

pub trait NodeRefReady<T: ElementDescriptor> {
    /// Invoke the function `f` when the element is put in the DOM.
    fn on_ready<F: FnOnce(HtmlElement<T>) + 'static>(self, cx: Scope, f: F);
}

impl<T: Clone + ElementDescriptor> NodeRefReady<T> for NodeRef<T> {
    fn on_ready<F: FnOnce(HtmlElement<T>) + 'static>(self, cx: Scope, f: F) {
        self.on_load(cx, |e| set_timeout(|| f(e), Duration::ZERO));
    }
}
