use crate::components::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let check = create_rw_signal(cx, false);

    view! {
        cx,
        <h1>"Material with Leptos"</h1>

        <MdcCard>
            <h3>"Card sample"</h3>
            <MdcTextField label="text field"/>

            <MdcButton label="text button"/>

            <MDCFormField label="CheckBox">
                <MDCCheckbox value=check/>
            </MDCFormField>
            <MDCFormField label="Indeterminate">
                <MDCCheckbox value=false indeterminate=check/>
            </MDCFormField>
            <MDCFormField label="Disabled">
                <MDCCheckbox disabled=true value=check/>
            </MDCFormField>
        </MdcCard>
    }
}
