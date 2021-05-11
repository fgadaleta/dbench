// use rand_isaac::Isaac64Rng;
use random_string::{
    Charset,
    GenResult,
    generate,
};


pub struct Record(pub Vec<u8>);

impl Record {
//     fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
//         Record { key, value}
//     }


}


/// Generate random keys and store to search box
//
pub fn generate_keys(num_records: usize) -> Vec<Record> {
    let charset_str = Charset::new("1234567890").unwrap();
    let mut searchbox = vec![];

    for _i in 0..num_records {
        // Generate key here and value directly from db (or waste too much memory)
        let key = generate(12, &charset_str).to_string();
        // let value = generate(value_size, &charset_str).to_string();
        // let record = Record::new(key.as_bytes().into(), value.as_bytes().into());
        let record = Record(key.as_bytes().into());
        searchbox.push(record);

        // if i % 10000 == 0 {
        //     println!("Added {} keys ", i + 10000);
        // }
    }

    searchbox
}

pub fn generate_value(size: usize) -> Record {
    let charset_str = Charset::new("1234567890").unwrap();
    let key = generate(size, &charset_str).to_string();
    Record(key.as_bytes().into())
}