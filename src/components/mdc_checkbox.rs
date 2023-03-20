use super::mdc_form_field::link_with_form_field;
use crate::{
    composables::{write_checked, Prop},
    utils::{apply_to, mdc_id},
};
use leptos::{component, create_effect, view, IntoView, MaybeSignal, Scope, SignalGet};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn MDCCheckbox(
    cx: Scope,
    #[prop(into, optional)] disabled: MaybeSignal<bool>,
    #[prop(into, optional)] indeterminate: Prop<bool>,
    #[prop(into)] value: Prop<bool>,
) -> impl IntoView {
    let id = mdc_id();

    let chk_ref = apply_to(cx, move |element: &HtmlElement| {
        let chk = attach_checkbox(element);

        link_with_form_field(element, id, &chk);
        create_effect(cx, move |_| chk.set_indeterminate(indeterminate.get()));
    });

    view! {
        cx,
        <div class="mdc-touch-target-wrapper">
            <div class="mdc-checkbox mdc-checkbox--touch" _ref=chk_ref>
                <input type="checkbox"
                    class="mdc-checkbox__native-control"
                    id=id
                    on:input=write_checked(value)
                    prop:checked=move || value.get()
                    prop:disabled=move || disabled.get()
                />
                <div class="mdc-checkbox__background">
                <svg class="mdc-checkbox__checkmark"
                    viewBox="0 0 24 24">
                    <path class="mdc-checkbox__checkmark-path"
                        fill="none"
                        d="M1.73,12.91 8.1,19.28 22.79,4.59"/>
                </svg>
                <div class="mdc-checkbox__mixedmark"></div>
                </div>
                <div class="mdc-checkbox__ripple"></div>
            </div>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    type Checkbox;

    #[wasm_bindgen(js_name = "mdc.checkbox.MDCCheckbox.attachTo")]
    fn attach_checkbox(element: &HtmlElement) -> Checkbox;

    #[wasm_bindgen(method, js_name = "indeterminate", getter)]
    fn indeterminate(this: &Checkbox) -> bool;

    #[wasm_bindgen(method, js_name = "indeterminate", setter)]
    fn set_indeterminate(this: &Checkbox, value: bool);
}
