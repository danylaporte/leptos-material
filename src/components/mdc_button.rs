use super::*;
use crate::{composables::Prop, utils::apply_to};
use leptos::{component, view, IntoView, Scope};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, MouseEvent};

#[component]
pub fn MdcButton(
    cx: Scope,
    #[prop(into, optional)] label: Prop<String>,
    #[prop(into, optional)] pre_icon: Prop<String>,
    click: Box<dyn FnMut(MouseEvent)>,
) -> impl IntoView {
    let btn_ref = apply_to(cx, attach_to);
    let mut click = click;

    view! {
        cx,
        <div class="mdc-touch-target-wrapper">
            <button class="mdc-button mdc-button--touch" _ref=btn_ref on:click=move |e| click(e)>
                <span class="mdc-button__ripple"></span>
                <span class="mdc-button__touch"></span>
                <MDCIcon name=pre_icon/>
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
