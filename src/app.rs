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
            <h3>"Buttons"</h3>

            <MDCButton
                label="Outline button with icon"
                pre_icon="settings"
                style=ButtonStyle::Outlined/>

            <MDCButton
                label="Text button"
                style=ButtonStyle::Text/>

            <MDCButton
                label="Contained button"
                style=ButtonStyle::Contained/>
        </MDCCard>

        <MDCCard>
            <h3>"Card sample"</h3>
            <MDCTextField label="text field"/>

            <MDCButton
                label="text button"
                pre_icon="settings"
                style=ButtonStyle::Outlined
                click=Box::new(move |_| progress.set(progress.get() + 0.1))/>

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

        <MDCCard>
            <h3>"Single Line List"</h3>

            <MDCList>
                <MDCListItem line1="Item 1"/>
                <MDCListItem line1="Item 2"/>
                <MDCListItem line1="Item 3"/>
                <MDCListDivider/>
                <MDCListItem line1="Others" />
            </MDCList>
        </MDCCard>

        <MDCCard>
            <h3>"Two Line List"</h3>

            <MDCList2>
                <MDCList2Item line1="Item 1" line2="sub title" />
                <MDCList2Item line1="Item 2" line2="sub title" />
                <MDCList2Item line1="Item 3" line2="sub title" />
                <MDCListDivider/>
                <MDCList2Item line1="Others" line2="sub title" />
            </MDCList2>
        </MDCCard>
    }
}
