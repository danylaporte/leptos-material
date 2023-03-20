use crate::{composables::Prop, utils::apply_to};
use leptos::{component, html::Ul, view, Fragment, IntoView, NodeRef, Scope, SignalGet};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn MDCList(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    let list_ref = list_ref(cx);

    view! {
        cx,
        <ul class="mdc-deprecated-list"
            role="listbox"
            _ref=list_ref>
            {children(cx)}
        </ul>
    }
}

#[component]
pub fn MDCListItem(cx: Scope, #[prop(into)] line1: Prop<String>) -> impl IntoView {
    view! {
        cx,
        <li class="mdc-deprecated-list-item">
            <span class="mdc-deprecated-list-item__ripple"></span>
            <span class="mdc-deprecated-list-item__text">{line1.get()}</span>
        </li>
    }
}

#[component]
pub fn MDCList2(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    let list_ref = list_ref(cx);

    view! {
        cx,
        <ul class="mdc-deprecated-list mdc-deprecated-list--two-line"
            role="listbox"
            _ref=list_ref>
            {children(cx)}
        </ul>
    }
}

#[component]
pub fn MDCList2Item(
    cx: Scope,
    #[prop(into)] line1: Prop<String>,
    #[prop(into)] line2: Prop<String>,
) -> impl IntoView {
    view! {
        cx,
        <li class="mdc-deprecated-list-item">
            <span class="mdc-deprecated-list-item__ripple"></span>
            <span class="mdc-deprecated-list-item__text">
                <span class="mdc-deprecated-list-item__primary-text">{line1.get()}</span>
                <span class="mdc-deprecated-list-item__secondary-text">{line2.get()}</span>
            </span>
        </li>
    }
}

#[component]
pub fn MDCListDivider(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <li role="separator" class="mdc-deprecated-list-divider"></li>
    }
}

fn list_ref(cx: Scope) -> NodeRef<Ul> {
    apply_to(cx, |e| {
        attach_to(&e);
    })
}

#[wasm_bindgen]
extern "C" {
    type MDCList;

    #[wasm_bindgen(js_name = "mdc.list.MDCList.attachTo")]
    fn attach_to(element: &HtmlElement) -> MDCList;
}
