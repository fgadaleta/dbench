use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub mod config;

// use rand_isaac::Isaac64Rng;
// use random_string::generate;

pub struct Record(pub Vec<u8>);

impl Record {
//     fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
//         Record { key, value}
//     }
}
const KEY_SPACE: usize = 20;

/// Generate random keys and store to search box
//
pub fn generate_keys(num_records: usize) -> Vec<Record> {
    let mut searchbox = vec![];

    for _i in 0..num_records {
        let key: String = thread_rng()
                                    .sample_iter(&Alphanumeric)
                                    .take(KEY_SPACE)
                                    .map(char::from)
                                    .collect();

        // Generate key here and value directly from db (or waste too much memory)
        // let key = generate(KEY_SPACE, CHARSET).to_string();
        let record = Record(key.as_bytes().into());
        searchbox.push(record);
    }

    searchbox
}

pub fn generate_value(size: usize) -> Record {
    // let key = generate(KEY_SPACE, CHARSET).to_string();
    let key: String = thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(size)
                        .map(char::from)
                        .collect();
    Record(key.as_bytes().into())
}