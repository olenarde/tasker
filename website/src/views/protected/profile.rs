use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn ProfilePage() -> impl IntoView {

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

// TEST BACKEND FUNCTION, WILL BE REFACTORED SOON
#[server(
    name = SomeStructName,
    endpoint = "load_db_string"
)]
pub async fn load_db_string() -> Result<String, ServerFnError> {

    use crate::app::ssr::{AppState, db};

    let state = expect_context::<AppState>();
    let db = db(&state.conn).await?;
    let conn = state.conn;

    

    Ok(conn)
}