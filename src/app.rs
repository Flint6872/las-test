
use crate::routes::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::*;

#[component]
pub fn App() -> impl IntoView {

    provide_meta_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Stylesheet id="leptos" href="/pkg/test.css"/>
        <Title text="Welcome to Leptos Surrealdb Axum Session Auth"/>
        <Router>
            <header>
                <A href="/">
                    <h1>"Auth"</h1>
                </A>
                <CompLogin />
            </header>
            <hr/>
            <main>
                <Routes>
                    // Route
                    <Route path="" view=HomePage/>
                    <CompLoginRoutes />

                </Routes>
            </main>
        </Router>
    }
}
