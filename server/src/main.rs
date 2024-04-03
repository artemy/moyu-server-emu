use crate::common::services::file::FileService;
use crate::common::services::mqtt::MqttService;
use axum::extract::{FromRef, Request};
use axum::routing::{get, post};
use axum::Router;
use common::services::device::DeviceService;
use common::services::history::ChatHistoryService;
use common::services::user::UserService;
use dotenv::dotenv;
use endpoints::alicloud::resource::ali_cloud;
use endpoints::app::device::resource::{
    get_device_id_by_verify_code, get_language_list, get_latest_version, get_models_list,
    push_update_version, set_language, sound_network,
};
use endpoints::app::user::resource::{
    bind, get_device_info_by_user_id, get_history_by_user_id, get_user_id_by_imei, logout, unbind,
};
use endpoints::app::version::resource::android;
use endpoints::outer::device::resource::{
    get_device_id_by_mac, network, upload_version, verify_code,
};
use endpoints::outer::speech::resource::download_audio;
use endpoints::outer::speech::resource::{qa, translate};
use endpoints::qa::qa_html;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use migration::{Migrator, MigratorTrait};
use openai::openai::OpenAiClient;
use paho_mqtt::{ConnectOptionsBuilder, CreateOptionsBuilder, SslOptionsBuilder};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_service::Service;

mod common;
mod endpoints;
mod opus;

#[tokio::main]
#[warn(unused_extern_crates)]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("openai::=DEBUG,moyu_server_emulator=DEBUG")
        .init();

    let db = initialize_db_client().await;
    let app_state = AppState {
        user_service: UserService::new(db.clone()),
        device_service: DeviceService::new(db.clone()),
        history_service: ChatHistoryService::new(db),
        mqtt: initialize_mqtt_service().await,
        file_service: initialize_file_service(),
        openai: initialize_openai_client(),
    };

    let outer_device = Router::new()
        .route("/getDeviceIdByMac", post(get_device_id_by_mac))
        .route("/network", get(network))
        .route("/uploadVersion", get(upload_version))
        .route("/getVerifyCodeByDeviceId", post(verify_code));

    let outer = Router::new()
        .nest("/device", outer_device)
        .route("/qa/xiaoai", post(qa))
        .route("/semanticanalysis/indexNew", post(translate));

    let alicloud_routes = Router::new().route("/devicename", post(ali_cloud));

    let app_user = Router::new()
        .route("/getUserIdByImei", post(get_user_id_by_imei))
        .route("/getHistoryByUserid", post(get_history_by_user_id))
        .route("/getDeviceInfoByUserid", post(get_device_info_by_user_id))
        .route("/bind", get(bind))
        .route("/unBind", get(unbind))
        .route("/logout", post(logout));

    let app_device = Router::new()
        .route(
            "/getDeviceIdByVerifyCode",
            get(get_device_id_by_verify_code),
        )
        .route("/getLatestVersion", get(get_latest_version))
        .route("/pushUpdateVersion", get(push_update_version))
        .route("/getLanguageList", get(get_language_list))
        .route("/setLanguage", get(set_language))
        .route("/getModelsList", get(get_models_list))
        .route("/soundNetWork", get(sound_network));

    let app_version = Router::new().route("/android", get(android));

    let root = Router::new()
        .nest("/outer", outer)
        .nest("/auth", alicloud_routes)
        .nest("/user", app_user)
        .nest("/device", app_device)
        .nest("/appversion", app_version)
        .route("/audio/{file_name}", get(download_audio))
        .route("/qa.html", get(qa_html))
        .with_state(app_state);

    serve(root).await;
}

async fn serve(root: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    log::info!("listening on {}", addr);

    let hyper_service =
        hyper::service::service_fn(move |request: Request<Incoming>| root.clone().call(request));

    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);
        let hyper_service = hyper_service.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(io, hyper_service)
                .with_upgrades()
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn initialize_db_client() -> DatabaseConnection {
    let db_connection_str = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://moyu:moyu@localhost/moyu".to_string());
    let mut opt = ConnectOptions::new(db_connection_str);
    opt.max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(3));

    let db = Database::connect(opt)
        .await
        .expect("can't connect to database");
    Migrator::up(&db, None).await.unwrap();
    db
}

async fn initialize_mqtt_service() -> MqttService {
    let mqtt_url = env::var("MQTT_URL").expect("MQTT_URL is not defined");

    let conn_opts = ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .ssl_options(
            SslOptionsBuilder::default()
                .enable_server_cert_auth(false)
                .finalize(),
        )
        .finalize();

    let mqtt = CreateOptionsBuilder::default()
        .server_uri(mqtt_url)
        .client_id("moyu-server-emulator")
        .create_client()
        .unwrap();

    mqtt.connect(conn_opts).await.unwrap();
    MqttService::new(mqtt)
}

fn initialize_file_service() -> FileService {
    let output_dir = env::var("FILE_OUTPUT_DIR")
        .unwrap_or_else(|_| env::temp_dir().to_str().unwrap().to_string());
    FileService::new(output_dir)
}

fn initialize_openai_client() -> OpenAiClient {
    let openai_key = env::var("OPENAI_KEY").expect("OPENAI_KEY is not defined");

    OpenAiClient::new(openai_key)
}

#[derive(Clone, FromRef)]
struct AppState {
    pub user_service: UserService,
    pub device_service: DeviceService,
    pub history_service: ChatHistoryService,
    pub mqtt: MqttService,
    pub file_service: FileService,
    pub openai: OpenAiClient,
}
