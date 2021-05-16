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
use redis::Commands;

use {
    // byteorder::{BigEndian, LittleEndian},
    zerocopy::{
        // byteorder::U64,
        AsBytes,
        // FromBytes, LayoutVerified, Unaligned, U16, U32,
    },
};

use dbench::{generate_keys, generate_value, config};


/// This is the concatenation merge operator in Sled.
fn sled_cat(_key: &[u8], val: Option<&[u8]>, new: &[u8]) -> Option<Vec<u8>> {
    Some(val.into_iter().flatten().chain(new).cloned().collect())
}

/// This is the concatenation merge operator in RocksDB.
fn rocks_cat(_key: &[u8], val: Option<&[u8]>, new: &mut rocksdb::MergeOperands) -> Option<Vec<u8>> {
    Some(
        val.into_iter()
            .flatten()
            .chain(new.into_iter().flatten())
            .cloned()
            .collect(),
    )
}

/// Quick and dirty slice to u32.
fn from_bytes(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}



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
    // Number of keys to generate
    let n_keys = 10000 as usize;
    let n_search_keys: usize = 10;
    // Size of value in bytes
    let value_size = 2048 as usize;
    let mut benchmark = c.benchmark_group("redis");

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut db = client.get_connection().unwrap();

    // Get pre-generated keys and store to searchbox
    let sbox = generate_keys(n_keys);
    println!("sbox len {}", sbox.len());

    benchmark.bench_function(
        BenchmarkId::new("redis_set", format!("{}x{}", n_keys, value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Fill in database
                for i in 0..sbox.len() {
                    // Generate random data
                    let value = generate_value(1024);
                    // Add (key, value) pair
                    let _: () = db.set(sbox[i].0.as_bytes(), value.0.as_bytes()).unwrap();
                }
            }
            });
        },
    );

    benchmark.bench_function(
        BenchmarkId::new("redis_get", format!("{}x{}", n_keys, value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Search random keys
                for i in 0..n_search_keys {
                    let s = &sbox[i];
                    let _: String = db.get(s.0.as_bytes()).unwrap();
                }
            }
            });
        },
    );




}

fn sled_bench(c: &mut Criterion) {
    // Number of keys to generate, search and size of value
    let conf = config::Config::default();

    let mut benchmark = c.benchmark_group("sled");

    let config = sled::Config::default()
        .path("/tmp/db_sled".to_owned())
        .cache_capacity(10_000_000_000)
        .flush_every_ms(Some(1000));
    let db = config.open().unwrap();
    // let _res = db.insert(&[1, 2, 3], vec![0]);

    // Get pre-generated keys and store to searchbox
    let sbox = generate_keys(conf.n_keys);
    println!("sbox len {}", sbox.len());

    benchmark.bench_function(
        BenchmarkId::new("sled_set", format!("{}x{}", conf.n_keys, conf.value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Fill in database
                for i in 0..sbox.len() {
                    // Generate random data
                    let value = generate_value(conf.value_size);
                    // Add (key, value) pair
                    db.insert(sbox[i].0.as_bytes(), value.0.as_bytes()).unwrap();
                }
            }
            });
        },
    );

    benchmark.bench_function(
        BenchmarkId::new("sled_get", format!("{}x{}", conf.n_search_keys, conf.value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Search random keys
                for i in 0..conf.n_search_keys {
                    let s = &sbox[i];

                    let _res = match db.get(s.0.as_bytes()) {
                        Ok(Some(_value)) => {
                            true
                        },

                        Ok(None) => false,
                        Err(_e) => {
                            false
                        },
                    };
                }
            }
            });
        },
    );
}


fn btree_bench(_c: &mut Criterion) {

}

fn rocksdb_bench(c: &mut Criterion) {
    // Number of keys to generate, search and size of value
    let conf = config::Config::default();
    let mut benchmark = c.benchmark_group("rocksdb");
    // let mut stats = Stats::default();
    let rocks_path = "/tmp/db_rocksdb";
    let mut options = rocksdb::Options::default();
    options.create_if_missing(true);
    options.set_compression_type(rocksdb::DBCompressionType::Lz4);
    let db = rocksdb::DB::open(&options, rocks_path).unwrap();

    // Get pre-generated keys and store to searchbox
    let sbox = generate_keys(conf.n_keys);
    println!("sbox len {}", sbox.len());

    benchmark.bench_function(
        BenchmarkId::new("rocksdb_set", format!("{}x{}", conf.n_keys, conf.value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Fill in database
                for i in 0..sbox.len() {
                    // Generate random data
                    let value = generate_value(conf.value_size);
                    // Add (key, value) pair
                    db.put(sbox[i].0.as_bytes(), value.0.as_bytes()).unwrap();
                }
            }

            });
        },
    );

    benchmark.bench_function(
        BenchmarkId::new("rocksdb_get", format!("{}x{}", conf.n_search_keys, conf.value_size)),
        |bencher| {
            bencher.iter(|| {
            {
                // Search random keys
                for i in 0..conf.n_search_keys {
                    let s = &sbox[i];
                    let _res = match db.get(s.0.as_bytes()) {
                        Ok(Some(_value)) => {
                            true
                        },

                        Ok(None) => false,
                        Err(_e) => {
                            false
                        },
                    };
                }
            }
            });
        },
    );

    // // Delete all keys
    // let iter = db.iterator(IteratorMode::Start);
    // for (_i, item) in iter.enumerate() {
    //     let key = item.0;
    //     db.delete(key).unwrap();
    // }

    // let _ = DB::destroy(&Options::default(), rocks_path);
    // stats.add(0 as f64);
    benchmark.finish();
}


criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = rocksdb_bench, sled_bench
}
criterion_main!(benches);