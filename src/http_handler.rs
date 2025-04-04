use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use warp::Filter;
use serde::de::DeserializeOwned;

use serde::Serialize;

mod http_requests;
use http_requests::*;

use crate::lobby::Lobby;


pub struct HttpHandler<'a> {
    lobbies: Arc<RwLock<HashMap<u32, Arc<RwLock<Lobby<'a>>>>>>,

}


fn json_body<'a, T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone 
where T: DeserializeOwned + Serialize + Clone + Send
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn create_new_account() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&uuid::Uuid::now_v7()))
}


async fn try_login(creds: LoginAttempt) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&creds.uuid))
}


async fn get_all_lobbies() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&Vec::<LobbyListItem>::new()))
}


async fn process_lobby_action(action: LobbyAction) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&1))
}


pub async fn run_server() {
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<LoginAttempt>())
        .and_then(try_login);

    let create_account = warp::get()
        .and(warp::path("create-account"))
        .and(warp::path::end())
        .and_then(create_new_account);


    let lobby_list = warp::get()
        .and(warp::path("list-all-lobbies"))
        .and(warp::path::end())
        .and_then(get_all_lobbies);

    let lobby_action = warp::post()
        .and(warp::path("lobby-action"))
        .and(warp::path::end())
        .and(json_body::<LobbyAction>())
        .and_then(process_lobby_action);

    warp::serve(login.or(create_account)).run(([127, 0, 0, 1], 5050)).await;
}