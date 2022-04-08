use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, middleware::Logger};
use actix_web::{dev::ServiceRequest, Error, guard};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use serde::{Deserialize, Serialize};
use url::Url;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    eprintln!("{:?}", credentials);
    Ok(req)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(alias = "code")]
    Code,
    #[serde(alias = "token")]
    Token,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    /*client_id: u64,*/
    response_type: ResponseType,
    #[serde(rename = "scope")]
    maybe_scope: Option<String>,
    #[serde(rename = "redirect_uri")]
    maybe_redirect_uri: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AuthResponse<'t> {
    pub token_type: &'t str,
    pub access_token: &'t str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<&'t str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<&'t str>,
}

async fn login<'t>(auth_request: AuthRequest) -> Result<HttpResponse, ServerError<'t>> {

    let access_token = "access_token";
    let maybe_refresh_token = None;
    let maybe_scope = auth_request.maybe_scope;

   match auth_request.response_type {
       ResponseType::Code => {

           let redirect_uri = auth_request.maybe_redirect_uri.ok_or(ServerError::InputError("redirect_uri is required when code is requested"))?;

           let mut location_uri = Url::parse(&redirect_uri)
               .expect("Failed to parse base search url");

           location_uri
               .query_pairs_mut()
               .append_pair("access_token", access_token)
               .append_pair("token_type", "code");

           if let Some(scope) = maybe_scope {
               location_uri.query_pairs_mut().append_pair("scope", scope.as_str());
           }

           if let Some(refresh_token) = maybe_refresh_token {
               location_uri.query_pairs_mut().append_pair("refresh_token", refresh_token);
           }

           let result = HttpResponse::Found()
               .header("Location", location_uri.as_str())
               .body(format!("Redirecting to {}", location_uri.as_str()));

           Ok(result)
       }

       ResponseType::Token => {

           let response = AuthResponse {
               token_type: "Bearer",
               access_token: access_token,
               refresh_token: maybe_refresh_token,
               scope: maybe_scope.as_ref().map(|s| s.as_str()),
           };

           let result = HttpResponse::Ok()
               .header("Cache-Control", "no-store")
               .json(response);

           Ok(result)
       }
   }
}

async fn login_form<'t>(query: web::Form<AuthRequest>) -> Result<HttpResponse, ServerError<'t>> {
    login(query.into_inner()).await
}

async fn login_json<'t>(query: web::Json<AuthRequest>) -> Result<HttpResponse, ServerError<'t>> {
    login(query.into_inner()).await
}

#[derive(Debug)]
enum ServerError<'t> {
    InputError(&'t str),
    //ArgonauticError,
    //DieselError,
    //EnvironmentError,
    //R2D2Error,
    //UserError(String)
}

impl<'t> std::fmt::Display for ServerError<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Test")
    }
}

impl<'t> actix_web::error::ResponseError for ServerError<'t> {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::InputError(msg) => HttpResponse::BadRequest().body(format!("Input error: {}", msg)),
            //ServerError::ArgonauticError => HttpResponse::InternalServerError().json("Argonautica Error."),
            //ServerError::DieselError => HttpResponse::InternalServerError().json("Diesel Error."),
            //ServerError::EnvironmentError => HttpResponse::InternalServerError().json("Environment Error."),
            //ServerError::UserError(data) => HttpResponse::InternalServerError().json(data)
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {

        let secured = web::scope("/secured")
            .wrap(HttpAuthentication::bearer(ok_validator))
            .service(hello);

        let auth_login = web::resource("/auth/login")
                .route(web::post().guard(guard::Header("Content-Type", "application/x-www-form-urlencoded")).to(login_form))
                .route(web::post().guard(guard::Header("Content-Type", "application/json")).to(login_json));

        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(hello)
            .service(echo)
            .service(secured)
            .service(auth_login)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8000")?
        .workers(4)
        .run()
        .await
}
