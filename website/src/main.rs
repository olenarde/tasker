
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::extract::Request;
    use axum::body::Body;
    use axum::middleware::{self, Next};
    use axum::response::{Redirect, Response};
    use axum::Router;
    use axum::routing::{get, post};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, generate_route_list_with_exclusions, LeptosRoutes, handle_server_fns};
    use clap::Parser;

    use website::app::*;
    use website::server::utils::clap::Opts;
    use website::app::ssr::AppState;

    // This GUARD function will be moved somewhere else and refactored soon
    async fn guard(
        mut request: Request<Body>,
        next: Next
    ) -> Result<Response, Redirect> {
    
        Err(Redirect::to("/"))
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;


    let protected_paths = generate_route_list_with_exclusions(App, Some(vec![
        // Add here routes, that are NOT protected (mean exceptions)

        // unprotected routes
        "/".to_owned(),

        // unprotected API calls
    ]));

    let unprotected_paths = generate_route_list_with_exclusions(App, Some(vec![
        // Add here routes, that MUST BE protected (mean expections for unprotected)

        // protected routes
        "/profile".to_owned(),
        // protected API calls
        "/api/load_db_string".to_owned()
    ]));
    
    // THIS STATE WITH READ FROM ENV WILL BE REFACTORED SOON
    // STATE GOES HERE
    let opts = Opts::parse();
    // let conn = Database::connect(opts.conn).await.unwrap();

    let app_state = AppState {
        // leptos_options: leptos_options.clone(),
        // db: conn,
        conn: opts.conn
    };

    let context = move || provide_context(app_state.clone());

    let protected_router = Router::new()
        .leptos_routes_with_context(&leptos_options, protected_paths, context.clone(), {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        });
        
    let unprotected_router = Router::new()
        .leptos_routes_with_context(&leptos_options, unprotected_paths, context, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        });

    let app = Router::new()
        .merge(protected_router)
        .route_layer(middleware::from_fn(guard))
        .merge(unprotected_router)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);
    
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
