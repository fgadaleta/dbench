use std::{
    // future::Ready, ops::Deref,
    usize};

use criterion::{
    // black_box,
    criterion_group, criterion_main,
    // AxisScale,
    BenchmarkId, Criterion,
    // PlotConfiguration,
};
use rocksdb::{DB, Options, IteratorMode};
use sled;
use redis;

use {
    // byteorder::{BigEndian, LittleEndian},
    zerocopy::{
        // byteorder::U64,
        AsBytes,
        // FromBytes, LayoutVerified, Unaligned, U16, U32,
    },
};

// use rand_isaac::Isaac64Rng;
// use random_string::{
//     Charset,
//     GenResult,
//     generate,
// };

use dbench::{generate_keys, generate_value};


#[derive(Default)]
struct Stats {
    total_cost: f64,
    runs: usize,
}

impl Stats {
    fn add(&mut self, cost: f64) {
        self.total_cost += cost;
        self.runs += 1;
    }
}

impl Drop for Stats {
    fn drop(&mut self) {
        if self.runs != 0 {
            println!("Average cost = {}", self.total_cost / self.runs as f64);
        }
    }
}

fn redis_bench(c: &mut Criterion) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
}

fn sled_bench(c: &mut Criterion) {
    // Number of keys to generate
    let n_keys = 10000 as usize;
    let n_search_keys: usize = 10;
    // Size of value in bytes
    let value_size = 2048 as usize;

    let mut benchmark = c.benchmark_group("sled");

    let db = sled::open("/tmp/db_sled").unwrap();
    let res = db.insert(&[1, 2, 3], vec![0]);

    // Get pre-generated keys and store to searchbox
    let sbox = generate_keys(n_keys);
    println!("sbox len {}", sbox.len());

    benchmark.bench_function(
        BenchmarkId::new("sled_params", format!("{}x{}", n_keys, value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Fill in database
                for i in 0..sbox.len() {
                    // Generate random data
                    let value = generate_value(1024);
                    // Add (key, value) pair
                    db.insert(sbox[i].0.as_bytes(), value.0.as_bytes()).unwrap();
                }

                // Search random keys
                for i in 0..n_search_keys {
                    let s = &sbox[i];

                    let _res = match db.get(s.0.as_bytes()) {
                        Ok(Some(_value)) => {
                            // println!("retrieved value {}", String::from_utf8(value).unwrap());
                            true
                        },

                        Ok(None) => false,
                        Err(_e) => {
                            // println!("operational problem encountered: {}", e);
                            false
                        },
                    };
                }
            }

            });
        },
    );

}


fn btree_bench(c: &mut Criterion) {

}

fn rocksdb_bench(c: &mut Criterion) {
    // let mut rng = Isaac64Rng::seed_from_u64(40);
    // let charset_str = Charset::new("1234567890").unwrap();
    let mut benchmark = c.benchmark_group("rocksdb");
    let mut stats = Stats::default();

    // Number of keys to generate
    let n_keys = 10000 as usize;
    let n_search_keys: usize = 10;
    // Size of value in bytes
    let value_size = 2048 as usize;

    let path = "/tmp/db_rocksdb";
    let db = DB::open_default(path).unwrap();

    // Get pre-generated keys and store to searchbox
    let sbox = generate_keys(n_keys);
    println!("sbox len {}", sbox.len());

    benchmark.bench_function(
        BenchmarkId::new("rocksdb_params", format!("{}x{}", n_keys, value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Fill in database
                for i in 0..sbox.len() {
                    // Generate random data
                    let value = generate_value(1024);
                    // Add (key, value) pair
                    db.put(sbox[i].0.as_bytes(), value.0.as_bytes()).unwrap();
                }

                // Search random keys
                for i in 0..n_search_keys {
                    let s = &sbox[i];

                    let _res = match db.get(s.0.as_bytes()) {
                        Ok(Some(_value)) => {
                            // println!("retrieved value {}", String::from_utf8(value).unwrap());
                            true
                        },

                        Ok(None) => false,
                        Err(_e) => {
                            // println!("operational problem encountered: {}", e);
                            false
                        },
                    };
                }
            }

            });
        },
    );

    // Delete all keys
    let iter = db.iterator(IteratorMode::Start);
    for (_i, item) in iter.enumerate() {
        let key = item.0;
        db.delete(key).unwrap();
        // if i % 10000 == 0 {
        //     println!("{} keys deleted", i + 10000);
        // }
    }

    let _ = DB::destroy(&Options::default(), path);
    stats.add(0 as f64);
    benchmark.finish();
}


criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = rocksdb_bench, sled_bench
}
criterion_main!(benches);