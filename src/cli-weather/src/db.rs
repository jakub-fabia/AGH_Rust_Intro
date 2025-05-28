use mongodb::{
    bson::{doc, to_document},
    options::{ClientOptions, IndexOptions},
    Client, Collection, IndexModel,
};
use crate::models::WeatherData;

pub struct MongoDb {
    collection: Collection<WeatherData>,
}

impl MongoDb {
    pub async fn new(connection_str: &str, db_name: &str, collection_name: &str) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(connection_str).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(db_name);
        let collection = database.collection::<WeatherData>(collection_name);

        // Ensure unique index on location.name and current.last_updated
        let index_model = IndexModel::builder()
            .keys(doc! { "location.name": 1, "current.last_updated": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        collection.create_index(index_model, None).await?;

        Ok(Self { collection })
    }

    pub async fn insert_if_new(&self, weather: &WeatherData) -> mongodb::error::Result<()> {
        let query = doc! {
            "location.name": &weather.location.name,
            "current.last_updated": &weather.current.last_updated,
        };

        if self.collection.find_one(query, None).await?.is_none() {
            self.collection.insert_one(weather, None).await?;
        }

        Ok(())
    }

    pub async fn get_by_location_and_time(&self, location: &str, time: &str) -> mongodb::error::Result<Option<WeatherData>> {
        let query = doc! {
            "location.name": location,
            "current.last_updated": time,
        };

        self.collection.find_one(query, None).await
    }
}
