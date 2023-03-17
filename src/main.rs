fn main() {
    // The raw data.
    // let data: Vec<u8> = vec![1, 2, 3, 4, 10, 11, 12, 13];
    // println!("Data: {:?}", data);
    
    // Convert first 4 bytes into a u32.
    // let first = u32::from_ne_bytes(data[0..4].try_into().unwrap());
    // println!("First: {:#x}", first);
    
    // Convert back into u32.
    // let original = u32::to_ne_bytes(first);
    // println!("Original: {:?}", original);

    let bytes = std::fs::read("./blk00000.dat").unwrap();

    for byte in &bytes[..10] {
        println!("{}", byte);
    }
}
