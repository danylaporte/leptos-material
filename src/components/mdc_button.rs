use crate::{composables::Prop, utils::apply_to};
use leptos::{component, view, IntoView, Scope};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn MdcButton(cx: Scope, #[prop(into, optional)] label: Prop<String>) -> impl IntoView {
    let btn_ref = apply_to(cx, attach_to);

    view! {
        cx,
        <div class="mdc-touch-target-wrapper" _ref=btn_ref>
            <button class="mdc-button mdc-button--touch">
                <span class="mdc-button__ripple"></span>
                <span class="mdc-button__touch"></span>
                <span class="mdc-button__label">{label}</span>
            </button>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "mdc.ripple.MDCRipple.attachTo")]
    fn attach_to(element: &HtmlElement);
}
