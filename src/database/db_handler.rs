use mongodb::{ bson::{ doc, Document}, options::{ ClientOptions, ServerApi, ServerApiVersion }, results::{ InsertOneResult, DeleteResult, UpdateResult }, Client, Collection};
use serde::{ de::DeserializeOwned, Serialize };
use uuid::Uuid;

/// DbHandler struct
/// 
/// Main datatype for connecting to a mongodb database. This can be injected
/// into different datatypes to enable database support. Work to convert this
/// to an enum (for testing purposes) may be done later.
/// 
/// Simple CRUD operations are supported.
struct DbHandler {
    client: Client,
    database_name: String,
}


impl DbHandler {
    pub async fn new(uri: String, db_name: String) -> mongodb::error::Result<Self> {
        let mut client_options = ClientOptions::parse(uri).await?;
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;
        return Ok(DbHandler {
            client: client,
            database_name: db_name,
        });
    }


    pub async fn add_document<T>(&self, doc: T, collection_name: &str) -> mongodb::error::Result<InsertOneResult> 
    where
        T: Serialize + Send + Sync 
    {

        let collection: Collection<T> = self.client.database(&self.database_name).collection(collection_name);
        return collection.insert_one(doc).await;
    }


    pub async fn get_document_by_id<T>(&self, id: Uuid, collection_name: &str) -> mongodb::error::Result<Option<T>>
    where 
        T: DeserializeOwned + Send + Sync
    {
        let collection: Collection<T> = self.client.database(&self.database_name).collection(collection_name);
        return collection.find_one(doc! { "_id": id.simple().to_string() }).await;
    }


    pub async fn delete_document_by_id<T>(&self, id: Uuid, collection_name: &str) -> mongodb::error::Result<DeleteResult> 
    where
        T: Send + Sync
    {
        let collection: Collection<T> = self.client.database(&self.database_name).collection(collection_name);
        return collection.delete_one(doc! { "_id": id.simple().to_string() }).await;
    }


    pub async fn update_document_by_id<T>(&self, id: Uuid, update_fields: Document, collection_name: &str) -> mongodb::error::Result<UpdateResult> 
    where
        T: Send + Sync
    {
        let collection: Collection<T> = self.client.database(&self.database_name).collection(collection_name);
        return collection.update_one(doc! { "_id": id.simple().to_string() }, update_fields).await;
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
    async fn test_delete_document(ctx: &mut Context) {
        let new_id = Uuid::now_v7();
        let dummy_account = Account {
            _id: new_id,
        };
        let _ = ctx.db.add_document(dummy_account, &ctx.test_collection).await;
        match ctx.db.delete_document_by_id::<Account>(new_id, &ctx.test_collection).await {
            Ok(res) => assert_eq!(res.deleted_count, 1),
            Err(_) => assert!(false),
        };
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn test_add_document(ctx: &mut Context) {
        let new_id = Uuid::now_v7();
        let dummy_account = Account {
            _id: new_id,
        };
        let res = ctx.db.add_document(dummy_account, &ctx.test_collection).await.unwrap();
        let _ = ctx.db.delete_document_by_id::<Account>(new_id, &ctx.test_collection).await;
        assert_eq!(res.inserted_id.as_str().unwrap(), new_id.simple().to_string(), "Unknown inserted ID {}", res.inserted_id.as_str().unwrap());
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn test_get_document(ctx: &mut Context) {
        let new_id = Uuid::now_v7();
        let dummy_account = Account {
            _id: new_id,
        };
        let _ = ctx.db.add_document(dummy_account, &ctx.test_collection).await;
        let doc: Account = ctx.db.get_document_by_id(new_id, &ctx.test_collection).await.unwrap().unwrap();
        let _ = ctx.db.delete_document_by_id::<Account>(new_id, &ctx.test_collection).await;
        assert_eq!(doc._id, new_id);
    }
}