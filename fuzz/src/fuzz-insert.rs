use arbitrary::Arbitrary;
use honggfuzz::fuzz;

#[derive(Arbitrary)]
pub struct Testcase {
    hasher_seed: usize,
    length: u16,
    key: String,
    value: String,
}

fn main() {
    loop {
        fuzz!(|testcase: Testcase| {
            let hasher = ahash::RandomState::with_seed(testcase.hasher_seed);
            let length = std::cmp::max(1, testcase.length) as usize;
            let mut lru = schnellru::LruMap::with_hasher(schnellru::ByLength::new(length as u32), hasher.clone());
            lru.insert(testcase.key, testcase.value);
        });
    }
}
