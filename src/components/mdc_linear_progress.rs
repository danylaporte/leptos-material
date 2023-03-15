use crate::{composables::Prop, utils::apply_to};
use js_sys::Number;
use leptos::{component, create_effect, view, IntoView, Scope, SignalGet};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[component]
pub fn MDCLinearProgress(
    cx: Scope,
    /// value should be between 0.0 and 1.0
    #[prop(into)]
    value: Prop<f64>,
) -> impl IntoView {
    let progress_ref = apply_to(cx, move |e| {
        let progress = attach_to(&e);

        create_effect(cx, move |_| {
            progress.set_progress(Number::from(value.get()))
        });
    });

    view! {
        cx,
        <div role="progressbar"
            class="mdc-linear-progress"
            aria-label="Example Progress Bar"
            aria-valuemin="0"
            aria-valuemax="1"
            _ref=progress_ref>

            <div class="mdc-linear-progress__buffer">
                <div class="mdc-linear-progress__buffer-bar"></div>
                <div class="mdc-linear-progress__buffer-dots"></div>
            </div>
            <div class="mdc-linear-progress__bar mdc-linear-progress__primary-bar">
                <span class="mdc-linear-progress__bar-inner"></span>
            </div>
            <div class="mdc-linear-progress__bar mdc-linear-progress__secondary-bar">
                <span class="mdc-linear-progress__bar-inner"></span>
            </div>
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    type LinearProgress;

    #[wasm_bindgen(js_name = "mdc.linearProgress.MDCLinearProgress.attachTo")]
    fn attach_to(element: &Element) -> LinearProgress;

    #[wasm_bindgen(method, js_name = progress, setter)]
    fn set_progress(this: &LinearProgress, value: Number);

}
