use super::*;
use crate::{composables::Prop, utils::apply_to};
use leptos::{component, view, IntoView, Scope, SignalGet};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, MouseEvent};

#[component]
pub fn MDCButton(
    cx: Scope,
    #[prop(into, optional)] label: Prop<String>,
    #[prop(into, optional)] pre_icon: Prop<String>,
    click: Box<dyn FnMut(MouseEvent)>,
) -> impl IntoView {
    let btn_ref = apply_to(cx, |e| {
        attach_to(&e);
        add_mdc_card_action(&e);
    });

    let mut click = click;

    view! {
        cx,
        <div class="mdc-touch-target-wrapper">
            <button class="mdc-button mdc-button--touch" _ref=btn_ref on:click=move |e| click(e)>
                <span class="mdc-button__ripple"></span>
                <span class="mdc-button__touch"></span>
                <MDCIcon name=pre_icon.get()/>
                <span class="mdc-button__label">{label.get()}</span>
            </button>
        </div>
    }
}

fn add_mdc_card_action(element: &HtmlElement) {
    if element
        .parent_element()
        .and_then(|p| p.parent_element())
        .map_or(false, |p| p.class_name().contains("mdc-card"))
    {
        let _ = element
            .class_list()
            .add_2("mdc-card__action", "mdc-card__action--button");
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "mdc.ripple.MDCRipple.attachTo")]
    fn attach_to(element: &HtmlElement);
}
