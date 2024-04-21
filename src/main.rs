use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    Condition, CreateCollection, Filter, SearchPoints, VectorParams, VectorsConfig, with_payload_selector::SelectorOptions, PayloadExcludeSelector,
    WithPayloadSelector,
};
use serde_json::json;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

#[tokio::main]
async fn main() -> Result<()> {

    //move rust-bert code to seperate function that writes to json files

        // Set-up sentence embeddings model
    //note bert uses cosine similarity in its training its embeddings so use cosine metric for retrieval
    // let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
    //     .create_model()?;

    // // Define input
    // let sentences = ["A brave alliance of underground freedom fighters has challenged the tyranny and oppression of the awesome GALACTIC EMPIRE.",
    //  "The dog and the cat are playing", "The dog and the cat are friends."];
    // let _num_sentences = sentences.len();


    // // Generate Embeddings
    // let embeddings = model.encode(&sentences)?;
    // let embedding_size: u64 = (*embeddings.get(0).unwrap()).len() as u64;
    // dbg!(embedding_size);

    let embedding_size: u64 = 10;
    dbg!(embedding_size);
    
    // Example of top level client
    // You may also use tonic-generated client from `src/qdrant.rs`
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);
    // collections_list = ListCollectionsResponse {
    //     collections: [
    //         CollectionDescription {
    //             name: "test",
    //         },
    //     ],
    //     time: 1.78e-6,
    // }

    let collection_name = "embedding_db";
    client.delete_collection(collection_name).await?;

    client
    .create_collection(&CreateCollection {
        collection_name: collection_name.into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: embedding_size,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }).await?;

    let collection_info = client.collection_info(collection_name).await?;
    //dbg!(collection_info);








    let payload_0: Payload = json!(
        {
            "Work": "Star Wars: A New Hope",
            "Favorite": true,
            "Director": "George Lucas",
            "Installation": {
                "Cannonical": 4,
                "Serial": 1
            }
        }
    )
        .try_into()
        .unwrap();

    let point_0 = PointStruct::new(0,vec![1., -1., 1., -1., 1., -1., 1., -1., 1., -1.], payload_0);

    let payload_1: Payload = json!(
        {
            "Title": "Star Wars: Revenge of the Sith",
            "Favorite": true,
            "Director": "George Lucas",
            "Installation": {
                "Cannonical": 3,
                "Serial": 6
            }
        }
    )
        .try_into()
        .unwrap(); 

    let point_1 = PointStruct::new(1,vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.] , payload_1);

    let payload_2: Payload = json!(
        {
            "Title": "Star Wars: Attack of the Clones",
            "Favorite": true,
            "Director": "George Lucas",
            "Installation": {
                "Cannonical": 2,
                "Serial": 5
            }
        }
    )
        .try_into()
        .unwrap(); 

    let point_2 = PointStruct::new(2,vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.1] , payload_2);

    let points = vec![point_1, point_2, point_0];

    client
        .upsert_points(collection_name, None, points, None)
        .await?;









    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![1.; embedding_size as usize],
            limit: 2,
            with_payload: Some(false.into()),
            ..Default::default()
        })
        .await?;
    
    dbg!(&search_result);
    // search_result = SearchResponse {
    //     result: [
    //         ScoredPoint {
    //             id: Some(
    //                 PointId {
    //                     point_id_options: Some(
    //                         Num(
    //                             0,
    //                         ),
    //                     ),
    //                 },
    //             ),
    //             payload: {
    //                 "bar": Value {
    //                     kind: Some(
    //                         IntegerValue(
    //                     12,
    //                     ),
    //                     ),
    //                 },
    //                 "foo": Value {
    //                     kind: Some(
    //                         StringValue(
    //                     "Bar",
    //                     ),
    //                     ),
    //                 },
    //             },
    //             score: 1.0000001,
    //             version: 0,
    //             vectors: None,
    //         },
    //     ],
    //     time: 9.5394e-5,
    // }

    //let found_point = search_result.result.into_iter();//.next().unwrap();
    //dbg!(found_point);

    Ok(())
}