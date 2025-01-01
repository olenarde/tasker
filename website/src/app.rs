use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos::task::spawn_local;

use crate::views::{
    auth::{login::LoginPage, register::RegisterPage},
    unprotected::home::HomePage,
    protected::profile::ProfilePage
};

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
                    <Route path=StaticSegment("/login") view=LoginPage/>
                    <Route path=StaticSegment("/register") view=RegisterPage/>
                </Routes>
            </main>
        </Router>
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
