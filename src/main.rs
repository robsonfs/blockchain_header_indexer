use blockchain_header_indexer::{Storage, parse_file};

fn main() {

    let mut storage = Storage::default();

    parse_file(&"./blk00000.dat", &mut storage).unwrap();

    println!("{:#?}", storage.block_map.get(&0).unwrap());
}
