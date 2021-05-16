pub struct Config {
    // Number of keys to generate
    pub n_keys: usize,
    // Numer of keys to search
    pub n_search_keys: usize,
    // Size in bytes of value to get
    pub value_size: Vec<usize>
}

impl Config {
    fn new(n_keys: usize, n_search_keys: usize, value_size: Vec<usize>) -> Self {
        Config {n_keys, n_search_keys, value_size }
    }
}

impl Default for Config {
    fn default() -> Self {

        Self {
            n_keys: 1000,
            n_search_keys: 100,
            value_size: vec![512, 1024, 2048]
        }
    }

}

