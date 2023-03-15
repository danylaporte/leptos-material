use leptos::{component, view, Fragment, IntoView, Scope};

#[component]
pub fn MDCCard(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    view! {
        cx,
        <div class="mdc-card">{children(cx)}</div>
    }
}

#[component]
pub fn MdCardActions(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    // TODO: use a load ref to propagate correct class on content.
    view! {
        cx,
        <div class="mdc-card__actions">{children(cx)}</div>
    }
}
