# (k,v) store benchmarking

## Install

```
sudo apt install gnuplot
cargo build
cargo install cargo-criterion
cargo-criterion
```



## Databases considered
* RocksDB
* Sled
* Redis
* levelDB



## Benchmark description

### RocksDB - Sled

* Set/Get 100 keys of 512 bytes each
* Set/Get 1000 keys of 512 bytes each
* Set/Get 100 keys of 1024 bytes each
* Set/Get 1000 keys of 1024 bytes each

```
rocksdb/rocksdb_set/1000x512    time:   [4.0774 ms 4.1015 ms 4.1169 ms]
sled/sled_set/1000x512          time:   [3.1818 ms 3.3511 ms 3.5089 ms]

rocksdb_get/100x512     time:   [40.022 us 40.275 us 40.818 us]
sled/sled_get/100x512   time:   [18.596 us 18.632 us 18.658 us]

rocksdb_set/1000x1024   time:   [6.0301 ms 6.0946 ms 6.1493 ms]
sled_set/1000x1024      time:   [4.7047 ms 4.9552 ms 5.2539 ms]

rocksdb_get/100x1024    time:   [48.303 us 48.940 us 49.210 us]
sled_get/100x1024       time:   [18.527 us 18.623 us 18.743 us]
```

### Sled 1M keys (value = 10 bytes ) vs 1k keys (value = 10000 bytes)
```
sled_set_1M_small_keys/1000000x10  time:   [2.8777 s 2.9240 s 2.9564 s]
sled_set_1000_large_keys/1000x1000 time:   [33.428 ms 33.727 ms 33.995 ms]

sled_get_1M_small_keys/1000000x10  time:   [1.0038 s 1.0121 s 1.0292 s]
sled_get_1000_large_keys/1000x1000  time:   [607.27 us 608.40 us 609.47 us]
```