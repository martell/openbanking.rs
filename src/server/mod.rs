// https://github.com/actix/examples/tree/master/hello-world
// https://github.com/actix/examples/blob/master/http-proxy/src/main.rs

use actix_files;
use actix_web;
use futures;
use log::{error, info};
use serde;

pub fn start(
    client: super::client::OpenBankingClient,
) -> std::thread::JoinHandle<std::result::Result<(), std::io::Error>> {
    use actix_web::{middleware, web, App, HttpServer};

    let thread = std::thread::spawn(move || {
        HttpServer::new(move || {
            App::new()
                .data(client.clone())
                // enable logger
                .wrap(middleware::Logger::default())
                .service(web::resource("/").route(web::get().to(authorise_callback)))
                .service(web::resource("/hello").to(|| "Hello world!"))
                .service(
                    web::resource("/api/conformancesuite/callback")
                        .route(web::post().to_async(api_authorise_callback)),
                )
                .service(web::resource("/conformancesuite/callback").to(authorise_callback))
        })
        .bind("127.0.0.1:8080")?
        .run()
    });

    thread
}

// When we navigate to this url:
//
// * https://modelobankauth2018.o3bank.co.uk:4101/auth?request=eyJhbGciOiJQUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IlF1RllCUkpuV2RJNl9OSEZnYW11WE5yNVIyMCJ9.eyJpc3MiOiIzZmM1MjhjZi1mYzg4LTQ2YzItOTMxNS1hOGNmODcyNDA3NWQiLCJzdWIiOiIzZmM1MjhjZi1mYzg4LTQ2YzItOTMxNS1hOGNmODcyNDA3NWQiLCJhdWQiOiJodHRwczovL21vZGVsb2JhbmthdXRoMjAxOC5vM2JhbmsuY28udWs6NDEwMSIsImV4cCI6MTU2MTk5MTU1MywiaWF0IjoxNTYxOTg5ODEzLCJqdGkiOiI5M2JkYjczZS00Y2QyLTQ2YjMtOWIzZC1jNTZiMGQ3ZDllY2UiLCJzY29wZSI6Im9wZW5pZCBhY2NvdW50cyIsImNsYWltcyI6eyJpZF90b2tlbiI6eyJhY3IiOnsidmFsdWUiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EiLCJlc3NlbnRpYWwiOnRydWV9LCJvcGVuYmFua2luZ19pbnRlbnRfaWQiOnsidmFsdWUiOiJhYWMtZjJlM2JhMTMtYWI5OC00OTExLWEyNzctMzgzMGM5YzQ3OGY1IiwiZXNzZW50aWFsIjp0cnVlfX0sInVzZXJpbmZvIjp7Im9wZW5iYW5raW5nX2ludGVudF9pZCI6eyJ2YWx1ZSI6ImFhYy1mMmUzYmExMy1hYjk4LTQ5MTEtYTI3Ny0zODMwYzljNDc4ZjUiLCJlc3NlbnRpYWwiOnRydWV9fX0sInJlZGlyZWN0X3VyaSI6Imh0dHBzOi8vMTI3LjAuMC4xOjg0NDMvY29uZm9ybWFuY2VzdWl0ZS9jYWxsYmFjayIsInN0YXRlIjoic3RhdGVfYWNjb3VudHMiLCJub25jZSI6IjVhNmIwZDc4MzJhOWZiNGY4MGYxMTcwYSIsImNsaWVudF9pZCI6IjNmYzUyOGNmLWZjODgtNDZjMi05MzE1LWE4Y2Y4NzI0MDc1ZCIsInJlc3BvbnNlX3R5cGUiOiJjb2RlIGlkX3Rva2VuIn0.bNY_6Z4sdNISIFPBcTwG3G3zXelUgU79P3Cd85qws0jcvGkZS518JuxgretNmtCAecn5KBDTWrJnCxPJesH_JRjY-_SJMVrI_chrvFLxI3oW_pVB0HfKj26hvNVyy9YOPtTW5xH9R3b3kJZHi9wbGNTugp3mXkPkAa80p0TBW1uZVVOr25SKs6hpOJRX4u24k4gzktJ3WvcH7vzN4IPjUDrX_XhCJ_RITRMKGmGxjplGYpojDTTU0ekZFfviLe46o75LXSzBpEK_V5eyGkh1pAZI9grDKkkT6L0yuGJ_aiyqMAdjzlIhRngi7wfEVkHc7wS5Zd-Jjgi3wDFyHm8Kmw&response_type=code+id_token&client_id=3fc528cf-fc88-46c2-9315-a8cf8724075d&state=state_accounts&nonce=5a6b0d7832a9fb4f80f1170a&scope=openid+accounts&redirect_uri=https%3A%2F%2F127.0.0.1%3A8443%2Fconformancesuite%2Fcallback
//
// A redirect happens, we then convert the `code`, `id_token`, `scope` and
// `state` (see `index.html`) in the query fragment (the # symbol) into a
// structure the backend can receive, since it is not possible to send fragments
// to the backend we need to process it using JavaScript code then forward it on
// to the backend. I.e., convert the query fragment to a form with the body
// content set to the values of the query fragment.
//
// Fragment (`response_type="code id_token"`):
//
// * https://127.0.0.1:8443/conformancesuite/callback#code=a052c795-742d-415a-843f-8a4939d740d1&scope=openid%20accounts&id_token=eyJ0eXAiOiJKV1QiLCJraWQiOiJGb2w3SXBkS2VMWm16S3RDRWdpMUxEaFNJek09IiwiYWxnIjoiRVMyNTYifQ.eyJzdWIiOiJtYmFuYSIsImF1ZGl0VHJhY2tpbmdJZCI6IjY5YzZkZmUzLWM4MDEtNGRkMi05Mjc1LTRjNWVhNzdjZWY1NS0xMDMzMDgyIiwiaXNzIjoiaHR0cHM6Ly9tYXRscy5hcy5hc3BzcC5vYi5mb3JnZXJvY2suZmluYW5jaWFsL29hdXRoMi9vcGVuYmFua2luZyIsInRva2VuTmFtZSI6ImlkX3Rva2VuIiwibm9uY2UiOiI1YTZiMGQ3ODMyYTlmYjRmODBmMTE3MGEiLCJhY3IiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EiLCJhdWQiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJjX2hhc2giOiIxbGt1SEFuaVJDZlZNS2xEc0pxTTNBIiwib3BlbmJhbmtpbmdfaW50ZW50X2lkIjoiQTY5MDA3Nzc1LTcwZGQtNGIyMi1iZmM1LTlkNTI0YTkxZjk4MCIsInNfaGFzaCI6ImZ0OWRrQTdTWXdlb2hlZXpjOGFHeEEiLCJhenAiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJhdXRoX3RpbWUiOjE1Mzk5NDM3NzUsInJlYWxtIjoiL29wZW5iYW5raW5nIiwiZXhwIjoxNTQwMDMwMTgxLCJ0b2tlblR5cGUiOiJKV1RUb2tlbiIsImlhdCI6MTUzOTk0Mzc4MX0.8bm69KPVQIuvcTlC-p0FGcplTV1LnmtacHybV2PTb2uEgMgrL3JNA0jpT2OYO73r3zPC41mNQlMDvVOUn78osQ&state=5a6b0d7832a9fb4f80f1170a
// * http://127.0.0.1:8080/conformancesuite/callback#code=a052c795-742d-415a-843f-8a4939d740d1&scope=openid%20accounts&id_token=eyJ0eXAiOiJKV1QiLCJraWQiOiJGb2w3SXBkS2VMWm16S3RDRWdpMUxEaFNJek09IiwiYWxnIjoiRVMyNTYifQ.eyJzdWIiOiJtYmFuYSIsImF1ZGl0VHJhY2tpbmdJZCI6IjY5YzZkZmUzLWM4MDEtNGRkMi05Mjc1LTRjNWVhNzdjZWY1NS0xMDMzMDgyIiwiaXNzIjoiaHR0cHM6Ly9tYXRscy5hcy5hc3BzcC5vYi5mb3JnZXJvY2suZmluYW5jaWFsL29hdXRoMi9vcGVuYmFua2luZyIsInRva2VuTmFtZSI6ImlkX3Rva2VuIiwibm9uY2UiOiI1YTZiMGQ3ODMyYTlmYjRmODBmMTE3MGEiLCJhY3IiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EiLCJhdWQiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJjX2hhc2giOiIxbGt1SEFuaVJDZlZNS2xEc0pxTTNBIiwib3BlbmJhbmtpbmdfaW50ZW50X2lkIjoiQTY5MDA3Nzc1LTcwZGQtNGIyMi1iZmM1LTlkNTI0YTkxZjk4MCIsInNfaGFzaCI6ImZ0OWRrQTdTWXdlb2hlZXpjOGFHeEEiLCJhenAiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJhdXRoX3RpbWUiOjE1Mzk5NDM3NzUsInJlYWxtIjoiL29wZW5iYW5raW5nIiwiZXhwIjoxNTQwMDMwMTgxLCJ0b2tlblR5cGUiOiJKV1RUb2tlbiIsImlhdCI6MTUzOTk0Mzc4MX0.8bm69KPVQIuvcTlC-p0FGcplTV1LnmtacHybV2PTb2uEgMgrL3JNA0jpT2OYO73r3zPC41mNQlMDvVOUn78osQ&state=5a6b0d7832a9fb4f80f1170a
pub fn authorise_callback() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("index.html")?
        .set_status_code(actix_web::http::StatusCode::NOT_FOUND))
}

// // Embeds the page that handles the callback in the binary. If you modify the
// file on disk // and reload the page, the new page is not served since it uses
// whatever the contents of // `index.html` was at compile-time.
// pub fn authorise_callback_static_page() ->
// actix_web::Result<actix_web::HttpResponse> {     Ok(actix_web::HttpResponse::
// Ok()         .content_type("text/html; charset=utf-8")
//         .body(include_str!("../index.html")))
// }

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Default,
    Clone,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
#[serde(deny_unknown_fields)]
pub struct RedirectFragment {
    pub code:     String,
    pub scope:    Option<String>,
    pub id_token: String,
    pub state:    String,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Default,
    Clone,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
#[serde(deny_unknown_fields)]
pub struct RedirectQuery {
    pub code:     String,
    pub scope:    Option<String>,
    pub id_token: String,
    pub state:    String,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Default,
    Clone,
    Hash,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
#[serde(deny_unknown_fields)]
pub struct RedirectError {
    pub error_description: String,
    pub error:             String,
    pub state:             String,
}

// This is the callback containing `code`, `id_token`, `scope` and `state` in a
// form instead of part of the the query fragment.
//
// /api/conformancesuite/callback?code=a052c795-742d-415a-843f-8a4939d740d1&
// scope=openid%20accounts&
// id_token=eyJ0eXAiOiJKV1QiLCJraWQiOiJGb2w3SXBkS2VMWm16S3RDRWdpMUxEaFNJek09IiwiYWxnIjoiRVMyNTYifQ.
// eyJzdWIiOiJtYmFuYSIsImF1ZGl0VHJhY2tpbmdJZCI6IjY5YzZkZmUzLWM4MDEtNGRkMi05Mjc1LTRjNWVhNzdjZWY1NS0xMDMzMDgyIiwiaXNzIjoiaHR0cHM6Ly9tYXRscy5hcy5hc3BzcC5vYi5mb3JnZXJvY2suZmluYW5jaWFsL29hdXRoMi9vcGVuYmFua2luZyIsInRva2VuTmFtZSI6ImlkX3Rva2VuIiwibm9uY2UiOiI1YTZiMGQ3ODMyYTlmYjRmODBmMTE3MGEiLCJhY3IiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EiLCJhdWQiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJjX2hhc2giOiIxbGt1SEFuaVJDZlZNS2xEc0pxTTNBIiwib3BlbmJhbmtpbmdfaW50ZW50X2lkIjoiQTY5MDA3Nzc1LTcwZGQtNGIyMi1iZmM1LTlkNTI0YTkxZjk4MCIsInNfaGFzaCI6ImZ0OWRrQTdTWXdlb2hlZXpjOGFHeEEiLCJhenAiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJhdXRoX3RpbWUiOjE1Mzk5NDM3NzUsInJlYWxtIjoiL29wZW5iYW5raW5nIiwiZXhwIjoxNTQwMDMwMTgxLCJ0b2tlblR5cGUiOiJKV1RUb2tlbiIsImlhdCI6MTUzOTk0Mzc4MX0.
// 8bm69KPVQIuvcTlC-p0FGcplTV1LnmtacHybV2PTb2uEgMgrL3JNA0jpT2OYO73r3zPC41mNQlMDvVOUn78osQ&
// state=5a6b0d7832a9fb4f80f1170a /api/conformancesuite/callback#
// code=a052c795-742d-415a-843f-8a4939d740d1&scope=openid%20accounts&
// id_token=eyJ0eXAiOiJKV1QiLCJraWQiOiJGb2w3SXBkS2VMWm16S3RDRWdpMUxEaFNJek09IiwiYWxnIjoiRVMyNTYifQ.
// eyJzdWIiOiJtYmFuYSIsImF1ZGl0VHJhY2tpbmdJZCI6IjY5YzZkZmUzLWM4MDEtNGRkMi05Mjc1LTRjNWVhNzdjZWY1NS0xMDMzMDgyIiwiaXNzIjoiaHR0cHM6Ly9tYXRscy5hcy5hc3BzcC5vYi5mb3JnZXJvY2suZmluYW5jaWFsL29hdXRoMi9vcGVuYmFua2luZyIsInRva2VuTmFtZSI6ImlkX3Rva2VuIiwibm9uY2UiOiI1YTZiMGQ3ODMyYTlmYjRmODBmMTE3MGEiLCJhY3IiOiJ1cm46b3BlbmJhbmtpbmc6cHNkMjpzY2EiLCJhdWQiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJjX2hhc2giOiIxbGt1SEFuaVJDZlZNS2xEc0pxTTNBIiwib3BlbmJhbmtpbmdfaW50ZW50X2lkIjoiQTY5MDA3Nzc1LTcwZGQtNGIyMi1iZmM1LTlkNTI0YTkxZjk4MCIsInNfaGFzaCI6ImZ0OWRrQTdTWXdlb2hlZXpjOGFHeEEiLCJhenAiOiI1NGY2NDMwOS00MzNkLTQ2MTAtOTVkMi02M2QyZjUyNTM0MTIiLCJhdXRoX3RpbWUiOjE1Mzk5NDM3NzUsInJlYWxtIjoiL29wZW5iYW5raW5nIiwiZXhwIjoxNTQwMDMwMTgxLCJ0b2tlblR5cGUiOiJKV1RUb2tlbiIsImlhdCI6MTUzOTk0Mzc4MX0.
// 8bm69KPVQIuvcTlC-p0FGcplTV1LnmtacHybV2PTb2uEgMgrL3JNA0jpT2OYO73r3zPC41mNQlMDvVOUn78osQ&
// state=5a6b0d7832a9fb4f80f1170a
pub fn api_authorise_callback(
    request: actix_web::HttpRequest, params: actix_web::web::Form<RedirectFragment>,
    client: actix_web::web::Data<super::client::OpenBankingClient>,
) -> impl futures::IntoFuture<Item = String, Error = ()> {
    // ) -> impl futures::Future<Item = String, Error = ()> {
    use futures::future::Future;

    info!("request={:?}", request);

    let code = params.code.clone();
    let scope = params.scope.clone();
    let id_token = params.id_token.clone();
    let state = params.state.clone();

    client.post_token_exchange(code, scope, id_token, state).map_err(|error| {
        error!("post_token_exchange failed, error={}", error);
        ()
    })

    // let exchange_token = client.post_token_exchange("".into(), "".into(),
    // "".into(), "".into()); info!("exchange_token={:?}", exchange_token);

    // let body = format!(
    //     "code={}, scope={}, id_token={}, state={}",
    //     params.code, params.scope, params.id_token, params.state
    // );
    // let response =
    // actix_web::HttpResponse::Ok().content_type("text/plain").body(body);
    // Ok(response)
}

// https://github.com/actix/examples/blob/master/form/src/main.rs
// https://github.com/actix/actix-web/issues/228
