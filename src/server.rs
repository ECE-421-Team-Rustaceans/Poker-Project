use std::sync::{Arc, RwLock, Mutex};
use std::collections::HashMap;

use warp::{Filter, http::StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use bson::doc;
use uuid::Uuid;

mod http_requests;
use http_requests::*;
use crate::database::db_handler::{self, DbHandler};
use crate::lobby::Lobby;
use crate::database::db_structs::Account;


fn json_body<'a, T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone 
where T: DeserializeOwned + Serialize + Clone + Send
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


#[derive(Clone)]
pub struct ServerState<'a> {
    db_handler: DbHandler,
    lobbies: Arc<RwLock<HashMap<u32, Arc<RwLock<Lobby<'a>>>>>>,
}


impl<'a> ServerState<'a> {
    pub fn new(db_handler: DbHandler) -> Self {
        Self {
            db_handler: db_handler,
            lobbies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

}

async fn create_new_account(state: ServerState<'_>) -> Result<impl warp::Reply, warp::Rejection> {
    let new_account_id = Uuid::now_v7().simple().to_string();
    match state.db_handler.add_document(doc! {
        "_id": new_account_id.clone()
    }, "Accounts").await {
        None => Ok(warp::reply::json(&new_account_id)),
        Some(res) => {
            match res {
                Ok(_) => {
                    println!("Successfully created new account {}", new_account_id);
                    Ok(warp::reply::json(&json!({"new_account_id": new_account_id})))
                },
                Err(e) => {
                    println!("Error while create new account: {}", e);
                    Err(warp::reject())
                }
            }
        },
    }
}


async fn try_login(state: ServerState<'_>, creds: LoginAttempt) -> Result<impl warp::Reply, warp::Rejection> {
    match state.db_handler.get_document::<Account>(doc! { "_id": creds.uuid.clone() }, "Accounts").await {
        None => Ok(warp::reply::json(&creds.uuid)),
        Some(res) => match res {
            Ok(res2) => match res2 {
                None => Err(warp::reject()),
                Some(_) => Ok(warp::reply::json(&creds.uuid)),
            },
            Err(e) => {
                println!("Error while attempting login: {}", e);
                Err(warp::reject())
            }
        }
    }
}


async fn get_all_lobbies(state: ServerState<'_>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut lobbyListItems = Vec::new();
    for (lobby_id, lobby_ptr) in state.lobbies.read().unwrap().iter() {
        let lobby = lobby_ptr.read().unwrap();
        lobbyListItems.push(LobbyListItem {
            lobby_id: *lobby_id,
            status: lobby.status(),
            user_count: lobby.count_users(),
            game_type: lobby.game_type()
        })
    }
    Ok(warp::reply::json(&lobbyListItems))
}


async fn process_lobby_action(state: ServerState<'_>, action: LobbyAction) -> Result<impl warp::Reply, warp::Rejection> {
    match action.action_type {
        LobbyActionType::Create => {
            let mut lobbies = state.lobbies.write().unwrap();
            let next_lobby_id = {
                let mut max_lobby_id: u32 = 0;
                for (lobby_id, _) in lobbies.iter() {
                    if *lobby_id > max_lobby_id {
                        max_lobby_id = *lobby_id;
                    }
                }
                max_lobby_id
            } + 1;
            lobbies.insert(next_lobby_id, Arc::new(RwLock::new(Lobby::new(next_lobby_id, action.game_type).await)));
        },
        LobbyActionType::Join => {

        },
        LobbyActionType::Leave => {

        },
        LobbyActionType::Start => {
            todo!()
        }
    }
    Ok(warp::reply::json(&1))
}


pub async fn run_server() {
    let db_handler = match DbHandler::new("mongodb://localhost:27017/".to_string(), "test".to_string()).await {
        Ok(handler) => handler,
        Err(e) => {
            println!("Server initializing dummy due to error while initializing database: {}", e);
            DbHandler::new_dummy()
        }
    };

    let state = ServerState::new(db_handler);
    let clone_state = {
        let state_clone = state.clone();
        move || state_clone.clone()
    };
    let login = warp::post()
        .map(clone_state.clone())
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<LoginAttempt>())
        .and_then(try_login);


    let create_account = warp::get()
        .map(clone_state.clone())
        .and(warp::path("create-account"))
        .and(warp::path::end())
        .and_then(create_new_account);

    let lobby_list = warp::get()
        .map(clone_state.clone())
        .and(warp::path("list-all-lobbies"))
        .and(warp::path::end())
        .and_then(get_all_lobbies);

    let lobby_action = warp::post()
        .map(clone_state.clone())
        .and(warp::path("lobby-action"))
        .and(warp::path::end())
        .and(json_body::<LobbyAction>())
        .and_then(process_lobby_action);

    warp::serve(login.or(create_account)).run(([127, 0, 0, 1], 5050)).await;

}