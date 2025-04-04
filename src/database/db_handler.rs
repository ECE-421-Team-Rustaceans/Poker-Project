/// DbHandler
///
/// This module contains code for interacting with a MongoDB database.
/// The intended schema for documents that can be stored are in db_struct.rs.
/// Information regarding a specific document type can be found in there.
/// Documents are stored in collections named the pluralized version of the
/// document names (e.g. Turn documents are stored in the Turns collection).
/// 
/// All documents use UUIDv7 for _id.
/// Generally, sub-collections and sub-documents are not used and fields are
/// dedicated to storing document IDs to create connections between documents.


use mongodb::{ action::CountDocuments, bson::{ doc, Document}, options::{ ClientOptions, ServerApi, ServerApiVersion }, results::{ DeleteResult, InsertManyResult, InsertOneResult, UpdateResult }, Client, Collection, Cursor};
use serde::{ de::DeserializeOwned, Serialize };
use uuid::Uuid;


extern crate bson;

/// DbClient enum
/// 
/// Datatype for client in DbHandler. This is used for creating a stub for
/// testing functions that connect to the database.
#[derive(Clone)]
pub enum DbClient {
    RealClient(Client),
    Dummy,
}


/// DbHandler struct
/// 
/// Main datatype for connecting to a mongodb database. This can be injected
/// into different datatypes to enable database support. Work to convert this
/// to an enum (for testing purposes) may be done later.
/// 
/// Simple CRUD operations are supported.
#[derive(Clone)]
pub struct DbHandler {
    client: DbClient,
    database_name: String,
}


impl DbHandler {
    /// Constructor for creating a new DbHandler using a uri and database name.
    pub async fn new(uri: String, db_name: String) -> mongodb::error::Result<Self> {
        let mut client_options = ClientOptions::parse(uri).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;
        return Ok(DbHandler {
            client: DbClient::RealClient(client),
            database_name: db_name,
        });
    }

    /// Constructor for a dummy DbHandler.
    /// Operations with dummy Dbhandlers do nothing.
    pub fn new_dummy() -> Self {
        DbHandler { 
            client: DbClient::Dummy, 
            database_name: "".to_string(),
        }
    }

    /// Adds one document to collection.
    pub async fn add_document<T>(&self, doc: T, collection_name: &str) -> Option<mongodb::error::Result<InsertOneResult>>
    where
        T: Serialize + Send + Sync 
    {
        match &self.client {
            DbClient::RealClient(client) => {
                let collection: Collection<T> = client.database(&self.database_name).collection(collection_name);
                Some(collection.insert_one(doc).await)
            },
            DbClient::Dummy => None,
        }
    }


    pub async fn count_documents<T>(&self, filter: Document, collection_name: &str) -> Option<mongodb::error::Result<u64>>
    where
        T: Send + Sync
    {
        match &self.client {
            DbClient::RealClient(client) => {
                let collection: Collection<T> = client.database(&self.database_name).collection(collection_name);
                Some(collection.count_documents(filter).await)
            },
            DbClient::Dummy => None,
        }
    }


    pub async fn get_documents<T>(&self, filter: Document, collection_name: &str) -> Option<mongodb::error::Result<Cursor<T>>>
    where
        T: DeserializeOwned + Send + Sync
    {
        match &self.client {
            DbClient::RealClient(client) => {
                let collection: Collection<T> = client.database(&self.database_name).collection(collection_name);
                Some(collection.find(filter).await)
            },
            DbClient::Dummy => None,
        }
    }

    pub async fn get_document<T>(&self, filter: Document, collection_name: &str) -> Option<mongodb::error::Result<Option<T>>>
    where
        T: DeserializeOwned + Send + Sync
    {
        match &self.client {
            DbClient::RealClient(client) => {
                let collection: Collection<T> = client.database(&self.database_name).collection(collection_name);
                Some(collection.find_one(filter).await)
            },
            DbClient::Dummy => None,
        }
    }

    /// Retrieves one document that matches id.
    pub async fn get_document_by_id<T>(&self, id: Uuid, collection_name: &str) -> Option<mongodb::error::Result<Option<T>>>
    where 
        T: DeserializeOwned + Send + Sync
    {
        match &self.client {
            DbClient::RealClient(_) => {
                self.get_document(doc! { "_id": id.simple().to_string() }, collection_name).await
            },
            DbClient::Dummy => None,
        }
    }

    /// Deletes document that matches id.
    pub async fn delete_document_by_id<T>(&self, id: Uuid, collection_name: &str) -> Option<mongodb::error::Result<DeleteResult>>
    where
        T: Send + Sync
    {
        match &self.client {
            DbClient::RealClient(client) => {
                let collection: Collection<T> = client.database(&self.database_name).collection(collection_name);
                Some(collection.delete_one(doc! { "_id": id.simple().to_string() }).await)
            },
            DbClient::Dummy => None,
        }
    }

    /// Updates certain fields in a document.
    pub async fn update_document_by_id<T>(&self, id: Uuid, update_fields: Document, collection_name: &str) -> Option<mongodb::error::Result<UpdateResult>>
    where
        T: Send + Sync
    {
        match &self.client {
            DbClient::RealClient(client) => {
                let collection: Collection<T> = client.database(&self.database_name).collection(collection_name);
                Some(collection.update_one(doc! { "_id": id.simple().to_string() }, update_fields).await)
            },
            DbClient::Dummy => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use test_context::{ test_context, AsyncTestContext };

    use super::*;
    use crate::database::db_structs::Account;


    struct Context {
        db: DbHandler,
        test_collection: String,
    }


    impl AsyncTestContext for Context {
        async fn setup() -> Self {
            let test_database = "ece421-poker-system-test";
            return Context {
                db: DbHandler::new("mongodb://localhost:27017/".to_string(), test_database.to_string()).await.unwrap(),
                test_collection: "Accounts".to_string()
            };
        }
    }


    #[test_context(Context)]
    #[tokio::test]
    #[ignore]
    async fn test_delete_document(ctx: &mut Context) {
        let new_id = Uuid::now_v7();
        let dummy_account = Account {
            _id: new_id,
        };
        let _ = ctx.db.add_document(dummy_account, &ctx.test_collection).await;
        match ctx.db.delete_document_by_id::<Account>(new_id, &ctx.test_collection).await.unwrap() {
            Ok(res) => assert_eq!(res.deleted_count, 1),
            Err(_) => assert!(false),
        };
    }

    #[test_context(Context)]
    #[tokio::test]
    #[ignore]
    async fn test_add_document(ctx: &mut Context) {
        let new_id = Uuid::now_v7();
        let dummy_account = Account {
            _id: new_id,
        };
        let res = ctx.db.add_document(dummy_account, &ctx.test_collection).await.unwrap().unwrap();
        let _ = ctx.db.delete_document_by_id::<Account>(new_id, &ctx.test_collection).await;
        assert_eq!(res.inserted_id.as_str().unwrap(), new_id.simple().to_string(), "Unknown inserted ID {}", res.inserted_id.as_str().unwrap());
    }

    #[test_context(Context)]
    #[tokio::test]
    #[ignore]
    async fn test_get_document(ctx: &mut Context) {
        let new_id = Uuid::now_v7();
        let dummy_account = Account {
            _id: new_id,
        };
        let _ = ctx.db.add_document(dummy_account, &ctx.test_collection).await;
        let doc: Account = ctx.db.get_document_by_id(new_id, &ctx.test_collection).await.unwrap().unwrap().unwrap();
        let _ = ctx.db.delete_document_by_id::<Account>(new_id, &ctx.test_collection).await;
        assert_eq!(doc._id, new_id);
    }
}