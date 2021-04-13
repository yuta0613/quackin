// use quackin::data::{Record, ReadOptions, read_custom_records, Field};
// quackin::data::Field::*;
extern crate csv;
extern crate sprs;

// pub mod data;
pub mod recommender;
pub mod metrics;

use data::*;
use recommender::*;
// use metrics::*;
use metrics::similarity::cosine;

// use Field::Other;
// use ReadError::Other;
// use std::io::ErrorKind::Other;


fn main (){
//     let options = ReadOptions::custom(vec![data::Field::UserID, data::Field::ItemID, data::Field::Rating, data::Field::Other], true, ',')
//     let records = read_custom_records("../data/mock_headers.csv", options);
//     let recommender = KnnUserRecommender::from_records(&records, cosine, 50);
    let records = read_records("data/mock.csv").unwrap();
    let recommender = KnnUserRecommender::from_records(&records, cosine, 5);
    
    let some_uir = vec![("user_2", "item_3", 2.5192531497347637),
                                    ("user_1", "item_3", 2.9524340130950657),
                                    ("user_6", "item_3", 2.767575112334526),
                                    ("user_4", "item_3", 2.7332710059168677),
                                    ("user_5", "item_3", 2.7369426258734384),
                                    ("user_8", "item_3", 2.9612309722134706),
                                    ("user_9", "item_3", 2.458585213496907)]
                    .into_iter()
                    .map(|(x, y, z)| (x.to_string(), y.to_string(), z));

    for (user_id, item_id, rating) in some_uir {
        let pred_rat = recommender.predict(&user_id, &item_id).expect("Should be possible to compute rating");
        println!("{} {}",pred_rat, rating);
//         assert!((pred_rat - rating).abs() < 0.1);
    }
}