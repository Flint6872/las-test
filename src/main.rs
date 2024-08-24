use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use las_test::app::App;
        use las_test::fallback::file_and_error_handler;
        
        use las_test::user::User;
        use las_test::server::{AppState ,SurrealPool, AuthSession};

        use axum::{
            response::{Response, IntoResponse},
            routing::get,
            extract::State,
            http::Request,
            body::Body as AxumBody,
            Router,
        };
        use axum_session::{SessionConfig, SessionLayer, SessionStore, SessionSurrealPool};
        use axum_session_auth::{AuthSessionLayer, AuthConfig};
        use leptos::{logging::log, provide_context, get_configuration};
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use surrealdb::Surreal;
        use surrealdb::engine::remote::ws::{Ws, Client};
        use surrealdb::opt::auth::Root;
        // use surrealdb::engine::any;
        // use leptos::*;
    
        async fn leptos_routes_handler(auth_session: AuthSession, State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_route_with_context(app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.pool.clone());
            },
            App
        );
        handler(req).await.into_response()
        }
    }
}


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    //simple_logger::init_with_level(log::Level::Warn).expect("couldn't initialize logging");
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment


    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.expect("Unable to Connect to Database.");

	db.signin(Root {
		username: "root",
		password: "root",
	})
	.await.expect("Database signin failed.");

	db.use_ns("namespace").use_db("database").await.expect("Namespace or Database incorrect.");

    let pool = db.clone();

    // Auth section
    let session_config = SessionConfig::default();
    let auth_config = AuthConfig::<i64>::default();
    let session_store =
        SessionStore::new(Some(SessionSurrealPool::new(pool.clone())), session_config)
            .await
            .unwrap();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState{
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    let app = Router::new()
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .layer(AuthSessionLayer::<User, i64, SessionSurrealPool<Client>, SurrealPool>::new(Some(pool.clone()))
        .with_config(auth_config))
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}



#[cfg(feature = "ssr")]
async fn server_fn_handler(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    request: Request<AxumBody>
) -> impl IntoResponse {
    

handle_server_fns_with_context(
move || {
    provide_context(auth_session.clone());
    provide_context(app_state.pool.clone());
}, request
    ).await
}


#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
