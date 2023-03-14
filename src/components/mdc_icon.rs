use crate::composables::{NodeRefReady, Prop};
use leptos::{
    component, create_node_ref, html::I, view, HtmlElement, IntoView, NodeRef, Scope, Show,
    ShowProps,
};

#[component]
pub fn MDCIcon(cx: Scope, #[prop(into)] name: Prop<String>) -> impl IntoView {
    let name2 = name.clone();
    let node_ref = compute_class(cx);

    view! {
        cx,
        <Show when=move || !name2().is_empty() fallback=|_| ()>
            <i class="material-icons" _ref=node_ref aria-hidden="true">{name()}</i>
        </Show>
    }
}

fn compute_class(cx: Scope) -> NodeRef<I> {
    let node_ref = create_node_ref(cx);

    node_ref.on_ready(cx, |icon: HtmlElement<I>| {
        if let Some(parent) = icon.parent_element() {
            let tag = parent.tag_name();

            // customize classes based on the location of the icon.
            match tag.as_str() {
                "BUTTON" => {
                    let _ = icon.class_list().add_1("mdc-button__icon");
                }
                _ => {}
            }
        }
    });

    node_ref
}
