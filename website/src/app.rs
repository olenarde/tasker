use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos::task::spawn_local;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/profile") view=ProfilePage/>
                    // <Route path="/profile" view=ProfilePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
#[allow(non_snake_case)]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
#[allow(non_snake_case)]
fn ProfilePage() -> impl IntoView {

    let (st, set_st) = signal("empty".to_string());

    view! {
        <h1>{"Profile Page"}</h1>
        <button on:click=move |_| {
            spawn_local(async move {
                let result = load_db_string().await.unwrap_or_else(|e| e.to_string());
                set_st.set(result);
            });
        }>
            {st}
        </button>
    }
}

#[cfg(feature = "ssr")]
pub mod ssr {

    use leptos::prelude::ServerFnError;
    use sqlx::{Connection, PgConnection};

    #[derive(Clone)]
    pub struct AppState {
        pub conn: String,
    }

    pub async fn db(conn: &String) -> Result<PgConnection, ServerFnError> {
        Ok(PgConnection::connect(conn).await?)
    }
}

// TEST BACKEND FUNCTION, WILL BE REFACTORED SOON
#[server(
    name = SomeStructName,
    endpoint = "load_db_string"
)]
pub async fn load_db_string() -> Result<String, ServerFnError> {

    let state = expect_context::<ssr::AppState>();
    let db = ssr::db(&state.conn).await?;
    let conn = state.conn;

    

    Ok(conn)
}