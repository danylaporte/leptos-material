use crate::{composables::Prop, utils::apply_to};
use leptos::{component, view, IntoView, Scope, SignalGet};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn MDCTextField(cx: Scope, #[prop(into, optional)] label: Prop<String>) -> impl IntoView {
    let label_ref = apply_to(cx, attach_to);

    view! {
        cx,
        <label class="mdc-text-field mdc-text-field--filled" _ref=label_ref>
            <span class="mdc-text-field__ripple"></span>
            <input type="text" class="mdc-text-field__input" aria-labelledby="my-label"/>
            <span class="mdc-floating-label" id="my-label">{label.get()}</span>
            <span class="mdc-line-ripple"></span>
        </label>
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "mdc.textField.MDCTextField.attachTo")]
    fn attach_to(element: &HtmlElement);
}
