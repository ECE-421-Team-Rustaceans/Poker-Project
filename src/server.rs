use std::sync::Arc;
use std::collections::HashMap;

use warp::filters::reply::WithHeader;
use warp::reply::Reply;
use warp::{Filter, http::Method};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use bson::doc;
use uuid::Uuid;
use tokio::sync::RwLock;

mod http_requests;
use http_requests::*;
use crate::database::db_handler::DbHandler;
use crate::input::server_input::ServerInput;
use crate::input::Input;
use crate::lobby::{self, Lobby};
use crate::database::db_structs::Account;
use crate::game_type::GameType;


fn json_body<'a, T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone 
where T: DeserializeOwned + Serialize + Clone + Send
{
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


#[derive(Clone)]
pub struct ServerState<I: Input + Send> {
    db_handler: DbHandler,
    lobbies: Arc<RwLock<HashMap<u32, Arc<RwLock<Lobby<I>>>>>>,
}


impl<I: Input + Send + Sync + 'static> ServerState<I> {
    pub fn new(db_handler: DbHandler) -> Self {
        Self {
            db_handler: db_handler,
            lobbies: Arc::new(RwLock::new(HashMap::new())),
        }
    }


    pub async fn add_lobby(&self, new_lobby: Lobby<I>) {
        let mut lobbies = self.lobbies.write().await;
        lobbies.insert(new_lobby.id(), Arc::new(RwLock::new(new_lobby)));
    }


    pub async fn get_new_lobby_id(&self) -> u32 {
        let lobbies = self.lobbies.read().await;
        let next_lobby_id = {
            let mut max_lobby_id: u32 = 0;
            for (lobby_id, _) in lobbies.iter() {
                if *lobby_id > max_lobby_id {
                    max_lobby_id = *lobby_id;
                }
            }
            max_lobby_id
        } + 1;
        next_lobby_id
    }


    pub async fn join_user(&self, user_id: Uuid, join_lobby_id: u32) -> Result<(), ()> {
        let lobbies = self.lobbies.read().await;
        for lobby_arc in lobbies.values() {
            let lobby = lobby_arc.read().await;
            match lobby.get_user(user_id) {
                Some(_) => return Err(()),
                None => (),
            }
        }

        return match lobbies.get(&join_lobby_id) {
            None => Err(()),
            Some(join_lobby_arc) => {
                let mut join_lobby = join_lobby_arc.write().await;
                join_lobby.join_user(user_id)
            },
        }
    }


    pub async fn leave_user(&self, user_id: Uuid, leave_lobby_id: u32) -> Result<(), ()> {
        let lobbies = self.lobbies.read().await;
        return match lobbies.get(&leave_lobby_id) {
            None => {
                println!("User {} cannot leave Lobby #{} because the lobby doesn't exist", user_id, leave_lobby_id);
                Err(())
            }
            Some(leave_lobby_arc) => {
                let mut leave_lobby = leave_lobby_arc.write().await;
                leave_lobby.leave_user(user_id)
            },
        };
    }

    pub async fn start_game(&self, lobby_id: u32) -> Result<(), ()> {
        let lobbies = self.lobbies.read().await;
        match lobbies.get(&lobby_id) {
            None => {
                println!("Start Lobby #{} because the lobby doesn't exist", lobby_id);
                Err(())
            },
            Some(start_lobby_arc) => {
                let start_lobby_arc_clone = start_lobby_arc.clone();
                println!("Before start_game thread spawn");
                tokio::spawn(async move {
                    let mut start_lobby = start_lobby_arc_clone.write().await;
                    start_lobby.start_game().await;
                });
                Ok(())
            }
        }
    }
}


fn add_allow_cors<R: Reply>(reply: R) -> warp::reply::WithHeader<R> {
    warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
}

async fn create_new_account<I: Input + Send + Sync>(state: ServerState<I>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Serving create-account request...");
    let new_account_id = Uuid::now_v7().simple().to_string();
    match state.db_handler.add_document(doc! {
        "_id": new_account_id.clone()
    }, "Accounts").await {
        None => Ok(add_allow_cors(warp::reply::json(&json!({ "new_account_id": new_account_id })))),
        Some(res) => {
            match res {
                Ok(_) => {
                    println!("Successfully created new account {}", new_account_id);
                    Ok(add_allow_cors(warp::reply::json(&json!({ "new_account_id": new_account_id }))))
                },
                Err(e) => {
                    println!("Error while create new account: {}", e);
                    Err(warp::reject())
                }
            }
        },
    }
}


async fn try_login<I: Input + Send + Sync>(state: ServerState<I>, creds: LoginAttempt) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", creds);
    match state.db_handler.get_document::<Account>(doc! { "_id": creds.uuid.clone() }, "Accounts").await {
        None => Ok(add_allow_cors(warp::reply::json(&json!({ "login_account_id": creds.uuid })))),
        Some(res) => match res {
            Ok(res2) => match res2 {
                None => Err(warp::reject()),
                Some(_) => Ok(add_allow_cors(warp::reply::json(&json!({ "login_account_id": creds.uuid })))),
            },
            Err(e) => {
                println!("Error while attempting login: {}", e);
                Err(warp::reject())
            }
        }
    }
}


async fn get_all_lobbies<I: Input + Send + Sync>(state: ServerState<I>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Retrieving lobbies...");
    let mut lobby_list_items = Vec::new();
    for (lobby_id, lobby_ptr) in state.lobbies.read().await.iter() {
        let lobby = lobby_ptr.read().await;
        lobby_list_items.push(LobbyListItem {
            lobby_id: *lobby_id,
            status: lobby.status(),
            user_count: lobby.count_users(),
            game_type: lobby.rules().to_game_type(),
        })
    }
    Ok(add_allow_cors(warp::reply::json(&lobby_list_items)))
}


async fn get_lobby_info<I: Input + Send + Sync>(state: ServerState<I>, lobby_id: u32) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Retrieving lobby #{}'s info...", lobby_id);
    let lobbies = state.lobbies.read().await;
    match lobbies.get(&lobby_id) {
        Some(lobby_arc) => {
            let lobby = lobby_arc.read().await;
            let active_users = lobby.active_players();
            let mut user_infos = Vec::new();
            for user in lobby.users().iter() {
                let mut is_active = false;
                for active in active_users {
                    if active.account_id() == *user {
                        is_active = true;
                    }
                }
                user_infos.push(LobbyUserInfo {
                    user_id: user.simple().to_string(),
                    is_active,
                })
            }

            Ok(add_allow_cors(warp::reply::json(&LobbyInfo {
                lobby_id,
                status: lobby.status(),
                users: user_infos,
                game_type: lobby.game_type(),
            })))
        },
        None => Err(warp::reject())
    }
}


async fn process_lobby_action<I: Input + Send + Sync + 'static>(state: ServerState<I>, action: LobbyAction) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Lobby action: {:?}", action);
    if let Ok(user_id) = Uuid::parse_str(&action.user_id) {
        match action.action_type {
            LobbyActionType::Create => {
                let next_lobby_id = state.get_new_lobby_id().await;
                println!("Creating lobby #{}", next_lobby_id);
                state.add_lobby(Lobby::new(next_lobby_id, action.game_type).await).await;
                Ok(add_allow_cors(warp::reply::json(&json!({
                    "new_lobby_id": next_lobby_id
                }))))
            },
            LobbyActionType::Join => {
                println!("User {} is joinning lobby #{}", user_id, action.lobby_id);
                match state.join_user(user_id, action.lobby_id).await {
                    Ok(()) => Ok(add_allow_cors(warp::reply::json(&json!({
                        "joinned_lobby_id": action.lobby_id
                    })))),
                    Err(()) => Err(warp::reject()),
                }
            },
            LobbyActionType::Leave => {
                //TODO: Clean up lobbies with zero users.
                match state.leave_user(user_id, action.lobby_id).await {
                    Err(()) => Err(warp::reject()),
                    Ok(()) => Ok(add_allow_cors(warp::reply::json(&json!({
                        "left_lobby_id": action.lobby_id
                    })))),
                }
            },
            LobbyActionType::Start => {
                Err(warp::reject())
                // match state.start_game(action.lobby_id).await {
                //     Ok(()) => Ok(add_allow_cors(warp::reply::json(&json!({
                //         "start_lobby_id": action.lobby_id,
                //     })))),
                //     Err(()) => Err(warp::reject()),
                // }
            }
        }
    } else {
        println!("Error parsing uuid while processing lobby-action.");
        Err(warp::reject())
    }
}


pub async fn run_server() {
    let db_handler = match DbHandler::new("mongodb://localhost:27017/".to_string(), "test".to_string()).await {
        Ok(handler) => handler,
        Err(e) => {
            println!("Server initializing dummy due to error while initializing database: {}", e);
            DbHandler::new_dummy()
        }
    };

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Allow-Origin", "Origin", "Accept", "X-Requested-With", "Content-Type"])
        .allow_methods(&[Method::GET, Method::POST]); 
    let state = ServerState::<ServerInput>::new(db_handler);
    state.add_lobby(Lobby::new(1, GameType::FiveCardDraw).await).await;
    state.add_lobby(Lobby::new(2, GameType::FiveCardDraw).await).await;
    state.add_lobby(Lobby::new(3, GameType::FiveCardDraw).await).await;
    state.add_lobby(Lobby::new(4, GameType::FiveCardDraw).await).await;

    let clone_state = {
        let state_clone = state.clone();
        move || state_clone.clone()
    };
    let login = warp::post()
        .map(clone_state.clone())
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body::<LoginAttempt>())
        .and_then(try_login).with(&cors);


    let create_account = warp::get()
        .map(clone_state.clone())
        .and(warp::path("create-account"))
        .and(warp::path::end())
        .and_then(create_new_account).with(&cors);

    let lobby_list = warp::get()
        .map(clone_state.clone())
        .and(warp::path("list-all-lobbies"))
        .and(warp::path::end())
        .and_then(get_all_lobbies).with(&cors);

    let lobby_info= warp::get()
        .map(clone_state.clone())
        .and(warp::path("lobby-info"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and_then(get_lobby_info).with(&cors);

    let lobby_action = warp::post()
        .map(clone_state.clone())
        .and(warp::path("lobby-action"))
        .and(warp::path::end())
        .and(json_body::<LobbyAction>())
        .and_then(process_lobby_action).with(&cors);

    warp::serve(lobby_action
        .or(login)
        .or(create_account)
        .or(lobby_list)
        .or(lobby_info)
    ).run(([127, 0, 0, 1], 5050)).await;

}
