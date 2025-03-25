use std::fmt;
struct ByteArray<'a>(&'a [u8; 8]);

impl<'a> fmt::LowerHex for ByteArray<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

fn bytes_to_hex(bytes: &[u8; 8]) -> String {
    format!("{:x}", ByteArray(bytes))
}
fn main() -> () {
    let mut namespace = "global".to_string();
    let mut name = None;
    for arg in std::env::args().skip(1) {
        if arg == "-e" {
            namespace = "event".to_string(); //std::env::args().nth(2).expect("no namespace given");
        } else if arg == "-n" {
            namespace = std::env::args().nth(2).expect("no namespace given");
        } else {
            name = Some(arg);
        }
    }
    let name = name.expect("no name given");
    let hash = get_hash(&namespace, &name);
    let hex = bytes_to_hex(&hash);

    // print result
    println!("namespace: {}", namespace);
    println!("name: {}", name);
    println!("hash:{:?} 0x{}", hash, hex);
    println!("b64:{}", base64::encode(hash));

    ()
}

pub fn get_hash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}
