use crate::composables::Prop;
use leptos::{component, view, Fragment, IntoView, Scope, SignalGet};
use std::cmp::min;

#[component]
pub fn MDCElevation(
    cx: Scope,
    children: Box<dyn Fn(Scope) -> Fragment>,
    #[prop(into, optional)] value: Prop<u8>,
) -> impl IntoView {
    view! {
        cx,
        <div class=move|| format!("mdc-elevation--z{}", min(value.get(), 24))>{children(cx)}</div>
    }
}
