use super::mdc_form_field::link_with_form_field;
use crate::{
    composables::Prop,
    utils::{apply_to, mdc_id},
};
use leptos::{component, create_effect, view, IntoView, Scope, SignalGet, SignalUpdate};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn MDCSwitch(
    cx: Scope,
    #[prop(into, optional)] disabled: Prop<bool>,
    #[prop(into)] value: Prop<bool>,
) -> impl IntoView {
    let id = mdc_id();

    let swt_ref = apply_to(cx, move |element: &HtmlElement| {
        let swt = attach_switch(element);

        link_with_form_field(element, id, &swt);

        create_effect(cx, move |_| {
            swt.set_disabled(disabled.get());
            swt.set_selected(value.get());
        });
    });

    view! {
        cx,
        <button id=id class="mdc-switch mdc-switch--unselected" type="button" role="switch" aria-checked="false" _ref=swt_ref on:click=move|_| value.update(|v| *v = !*v)>
            <div class="mdc-switch__track"></div>
            <div class="mdc-switch__handle-track">
                <div class="mdc-switch__handle">
                <div class="mdc-switch__shadow">
                    <div class="mdc-elevation-overlay"></div>
                </div>
                <div class="mdc-switch__ripple"></div>
                <div class="mdc-switch__icons">
                    <svg class="mdc-switch__icon mdc-switch__icon--on" viewBox="0 0 24 24">
                        <path d="M19.69,5.23L8.96,15.96l-4.23-4.23L2.96,13.5l6,6L21.46,7L19.69,5.23z" />
                    </svg>
                    <svg class="mdc-switch__icon mdc-switch__icon--off" viewBox="0 0 24 24">
                        <path d="M20 13H4v-2h16v2z" />
                    </svg>
                </div>
                </div>
            </div>
            <span class="mdc-switch__focus-ring-wrapper">
                <div class="mdc-switch__focus-ring"></div>
            </span>
        </button>
    }
}

#[wasm_bindgen]
extern "C" {
    type Switch;

    #[wasm_bindgen(js_name = "mdc.switchControl.MDCSwitch.attachTo")]
    fn attach_switch(element: &HtmlElement) -> Switch;

    #[wasm_bindgen(method, js_name = "disabled", setter)]
    fn set_disabled(this: &Switch, value: bool);

    #[wasm_bindgen(method, js_name = "selected", setter)]
    fn set_selected(this: &Switch, value: bool);
}
