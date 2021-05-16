pub struct Config {
    // Number of keys to generate
    pub n_keys: usize,
    // Numer of keys to search
    pub n_search_keys: usize,
    // Size in bytes of value to get
    pub value_size: usize
}

impl Config {
    fn new(n_keys: usize, n_search_keys: usize, value_size: usize) -> Self {
        Config {n_keys, n_search_keys, value_size }
    }
}

impl Default for Config {
    fn default() -> Self {

        Self {
            n_keys: 100000,
            n_search_keys: 1000,
            value_size: 2048
        }
    }

}

