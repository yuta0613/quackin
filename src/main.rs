// use quackin::data::{Record, ReadOptions, read_custom_records, Field};
// quackin::data::Field::*;
extern crate csv;
extern crate sprs;

pub mod data;
pub mod recommender;
pub mod metrics;

use data::*;
// use recommender::*;
// use metrics::*;

// use Field::Other;
// use ReadError::Other;
// use std::io::ErrorKind::Other;


fn main (){
    let options = ReadOptions::custom(vec![data::Field::UserID, data::Field::ItemID, data::Field::Rating, data::Field::Other], true, ',');
    //                                                             ^^^^^   ^^^^  ^^^
    //                                                             |       |     |
    //                             we don't care about the timestamp.      |     |
    //                                                 this file has headers.    |
    //                                                    use comma as a delimiter.

    let _records = read_custom_records("../data/mock_headers.csv", options);

    for _record in _records {
        // println!("{} {} {}", _record.UserID, _record.ItemID, _record.Rating);
        println!("{:?}",_record[1])
    }
}