use super::*;
use crate::{
    composables::{Handler, Prop},
    utils::apply_to,
};
use leptos::{component, view, IntoView, Scope, SignalGet, SignalWith};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, MouseEvent};

#[component]
pub fn MDCButton(
    cx: Scope,
    #[prop(into, optional)] style: Prop<ButtonStyle>,
    #[prop(into, optional)] label: Prop<String>,
    #[prop(into, optional)] pre_icon: Prop<String>,
    #[prop(into, optional)] click: Handler<MouseEvent>,
) -> impl IntoView {
    let btn_ref = apply_to(cx, |e| {
        attach_to(&e);
        add_mdc_card_action(&e);
    });

    let pre_icon2 = pre_icon.clone();

    let class = move || {
        [
            "mdc-button mdc-button--touch",
            style.with(|v| v.class()),
            pre_icon2.with(|v| {
                if v.is_empty() {
                    ""
                } else {
                    "mdc-button--icon-leading"
                }
            }),
        ]
        .into_iter()
        .fold(String::new(), |mut acc, item| {
            if !acc.is_empty() && !item.is_empty() {
                acc.push(' ');
            }
            acc.push_str(item);
            acc
        })
    };

    view! {
        cx,
        <div class="mdc-touch-target-wrapper">
            <button class=class _ref=btn_ref on:click=move |e| click.call(e)>
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

#[derive(Clone, Copy)]
pub enum ButtonStyle {
    Text,
    Outlined,
    Contained,
}

impl ButtonStyle {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Contained => "mdc-button--raised",
            Self::Outlined => "mdc-button--outlined",
            Self::Text => "",
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Contained
    }
}
