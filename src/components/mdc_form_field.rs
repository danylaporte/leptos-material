use crate::composables::Prop;
use js_sys::Reflect;
use leptos::{component, view, Fragment, IntoView, Scope, SignalGet};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[component]
pub fn MDCFormField(
    cx: Scope,
    children: Box<dyn Fn(Scope) -> Fragment>,
    #[prop(into)] label: Prop<String>,
) -> impl IntoView {
    view! {
        cx,
        <div class="mdc-form-field">
            {children(cx)}
            <label>{move || label.get()}</label>
        </div>
    }
}

fn find_mdc_form_field_from_child(v: &Element) -> Option<Element> {
    let mut v = v.clone();

    while let Some(p) = v.parent_element() {
        if p.class_list().contains("mdc-form-field") {
            return Some(p);
        }

        v = p;
    }

    None
}

pub(crate) fn link_with_form_field(linked_element: &Element, id: &str, linked_instance: &JsValue) {
    if let Some(form_field) = find_mdc_form_field_from_child(linked_element) {
        if let Some(label) = form_field.get_elements_by_tag_name("label").item(0) {
            let _ = label.set_attribute("for", id);
        }

        let form_field = attach_form_field(&form_field);
        let _ = Reflect::set(&form_field, &"input".into(), linked_instance);
    }
}

#[wasm_bindgen]
extern "C" {
    type FormField;

    #[wasm_bindgen(js_name = "mdc.formField.MDCFormField.attachTo")]
    fn attach_form_field(element: &Element) -> FormField;
}
