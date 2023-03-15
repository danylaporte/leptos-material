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
    let progress = create_rw_signal(cx, 0.0);

    view! {
        cx,
        <h1>"Material with Leptos"</h1>

        <MDCCard>
            <h3>"Card sample"</h3>
            <MDCTextField label="text field"/>

            <MDCButton label="text button" pre_icon="settings" click=Box::new(move |_| progress.set(progress.get() + 0.1))/>

            <MDCFormField label="Switch">
                <MDCSwitch value=check/>
            </MDCFormField>

            <MDCFormField label="CheckBox">
                <MDCCheckbox value=check/>
            </MDCFormField>
            <MDCFormField label="Indeterminate">
                <MDCCheckbox value=false indeterminate=check/>
            </MDCFormField>
            <MDCFormField label="Disabled">
                <MDCCheckbox disabled=true value=check/>
            </MDCFormField>

            <MDCLinearProgress value=progress/>
        </MDCCard>
    }
}
