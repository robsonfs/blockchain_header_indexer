use std::collections::HashMap;
use std::io;
use std::path::Path;
use web3::types::U256;

type Int = u64;

#[derive(Debug)]
pub struct BlockHeader {
    pub version: String,
    pub prev_hash: String,
    pub merkle_hash: String,
    pub time: String,
    pub n_bits: String,
    pub nonce: String,
}

impl BlockHeader {
    pub fn new<T: AsRef<str>>(version: T, prev_hash: T, merkle_hash: T, time: T, n_bits: T, nonce: T) -> Self {
        Self {
            version: version.as_ref().to_owned(),
            prev_hash: prev_hash.as_ref().to_owned(),
            merkle_hash: merkle_hash.as_ref().to_owned(),
            time: time.as_ref().to_owned(),
            n_bits: n_bits.as_ref().to_owned(),
            nonce: nonce.as_ref().to_owned(),
        }
    }

    pub fn json(&self) -> HashMap<String, String> {
        HashMap::from(
            [
                ("version".to_owned(), self.version.clone()),
                ("previous_block_header_hash".to_owned(), self.prev_hash.clone()),
                ("merkle_root_hash".to_owned(), self.merkle_hash.clone()),
                ("time".to_owned(), self.time.clone()),
                ("n_bits".to_owned(), self.n_bits.clone()),
                ("nonce".to_owned(), self.nonce.clone()),
            ]
        )
    }
}

#[derive(Default)]
pub struct Storage {
    pub block_map: HashMap<Int, BlockHeader>
}

impl Storage {
    pub fn add_block(&mut self, block_height: Int, block_header: BlockHeader) {
        self.block_map.insert(block_height, block_header);
    }
}

pub fn parse_file<T: AsRef<Path>>(path: &T, storage: &mut Storage) -> Result<(), io::Error> {
    let mut offset = 0;
    let mut block_height = 0;

    let mut file_as_bytes = std::fs::read(path)?;
    let size = file_as_bytes.len();
    
    let mut pos = 0;

    while pos < size {
        let iter = file_as_bytes.iter();
        let mut chunk = iter.skip(pos + 4);

        // Find block size
        let block_size = u32::from_ne_bytes([*chunk.next().unwrap(); 4]);

        // Parse Header and Strore
        let version = format!("{:#x}", u32::from_ne_bytes([*chunk.next().unwrap(); 4]));
        let prev_hash = format!("{:#x}", U256::from_little_endian(&[*chunk.next().unwrap(); 32]));
        let merkle_hash = format!("{:#x}", U256::from_little_endian(&[*chunk.next().unwrap(); 32]));
        let time = format!("{:#x}", u32::from_ne_bytes([*chunk.next().unwrap(); 4]));
        let n_bits = format!("{:#x}", u32::from_ne_bytes([*chunk.next().unwrap(); 4]));
        let nonce = format!("{:#x}", u32::from_ne_bytes([*chunk.next().unwrap(); 4]));
        storage.add_block(
            block_height,
            BlockHeader::new(version, prev_hash, merkle_hash, time, n_bits, nonce)
        );

        file_as_bytes = chunk.map(|k| *k).collect::<Vec<u8>>();

        // Set next offset
        offset = offset + 8 + block_size;
        pos = offset as usize;


        block_height += 1;

        println!("Size: {}\nPos: {}\nOffset: {}\nBlock Height: {}\n", size, pos, offset, block_height);

    }


    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
