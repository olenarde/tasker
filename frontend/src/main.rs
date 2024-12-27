
// #[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::extract::Request;
    use axum::body::Body;
    use axum::middleware::{self, Next};
    use axum::response::{Redirect, Response};
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, generate_route_list_with_exclusions, LeptosRoutes};
    use frontend::app::*;
    
    use frontend::server::utils::routes::routes;

    async fn guard(
        mut request: Request<Body>,
        next: Next
    ) -> Result<Response, Redirect> {
    
        Err(Redirect::to("/"))
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;


    let protected_routes = generate_route_list_with_exclusions(App, Some(vec![
        // Add here routes, that are NOT protected (mean exceptions)
        "/".to_owned()
    ]));

    let unprotected_routes = generate_route_list_with_exclusions(App, Some(vec![
        // Add here routes, that MUST BE protected (mean expections for unprotected)
        "/profile".to_owned()
    ]));

    let protected_router = Router::new()
        .leptos_routes(&leptos_options, protected_routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .route_layer(middleware::from_fn(guard));

    let unprotected_router = Router::new()
        .leptos_routes(&leptos_options, unprotected_routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        });

    let app = Router::new()
        .merge(protected_router)
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
