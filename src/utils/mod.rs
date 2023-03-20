mod mdc_id;

use leptos::NodeRef;
use leptos::{create_node_ref, html::ElementDescriptor, set_timeout, Scope};
pub use mdc_id::{mdc_id, MDCId};
use std::ops::Deref;
use std::time::Duration;
use wasm_bindgen::JsCast;

pub(crate) fn apply_to<E: Clone + ElementDescriptor, T>(
    cx: Scope,
    apply: impl Fn(&T) + 'static,
) -> NodeRef<E>
where
    E: Deref + 'static,
    <leptos::HtmlElement<E> as Deref>::Target: JsCast,
    T: Clone + JsCast + 'static,
{
    let node_ref = create_node_ref::<E>(cx);

    node_ref.on_load(cx, |e| {
        let e = e.unchecked_ref::<T>().clone();
        set_timeout(move || apply(&e), Duration::ZERO)
    });

    node_ref
}
