#![allow(unused)]
use alloc::vec::Vec;
use bitcoin::{
    consensus,
    hashes::{hex::FromHex, Hash},
    Transaction,
};

use crate::BlockId;

pub const RAW_TX_1: &str = "0200000000010116d6174da7183d70d0a7d4dc314d517a7d135db79ad63515028b293a76f4f9d10000000000feffffff023a21fc8350060000160014531c405e1881ef192294b8813631e258bf98ea7a1027000000000000225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c024730440220591b1a172a122da49ba79a3e79f98aaa03fd7a372f9760da18890b6a327e6010022013e82319231da6c99abf8123d7c07e13cf9bd8d76e113e18dc452e5024db156d012102318a2d558b2936c52e320decd6d92a88d7f530be91b6fe0af5caf41661e77da3ef2e0100";
pub const RAW_TX_2: &str = "02000000000101a688607020cfae91a61e7c516b5ef1264d5d77f17200c3866826c6c808ebf1620000000000feffffff021027000000000000225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c20fd48ff530600001600146886c525e41d4522042bd0b159dfbade2504a6bb024730440220740ff7e665cd20565d4296b549df8d26b941be3f1e3af89a0b60e50c0dbeb69a02206213ab7030cf6edc6c90d4ccf33010644261e029950a688dc0b1a9ebe6ddcc5a012102f2ac6b396a97853cb6cd62242c8ae4842024742074475023532a51e9c53194253e760100";
pub const RAW_TX_3: &str = "0200000000010135d67ee47b557e68b8c6223958f597381965ed719f1207ee2b9e20432a24a5dc0100000000feffffff021027000000000000225120a82f29944d65b86ae6b5e5cc75e294ead6c59391a1edc5e016e3498c67fc7bbb62215a5055060000160014070df7671dea67a50c4799a744b5c9be8f4bac690247304402207ebf8d29f71fd03e7e6977b3ea78ca5fcc5c49a42ae822348fc401862fdd766c02201d7e4ff0684ecb008b6142f36ead1b0b4d615524c4f58c261113d361f4427e25012103e6a75e2fab85e5ecad641afc4ffba7222f998649d9f18cac92f0fcc8618883b3ee760100";
pub const RAW_TX_4: &str = "02000000000101d00e8f76ed313e19b339ee293c0f52b0325c95e24c8f3966fa353fb2bedbcf580100000000feffffff021027000000000000225120882d74e5d0572d5a816cef0041a96b6c1de832f6f9676d9605c44d5e9a97d3dc9cda55fe53060000160014852b5864b8edd42fab4060c87f818e50780865ff0247304402201dccbb9bed7fba924b6d249c5837cc9b37470c0e3d8fbea77cb59baba3efe6fa0220700cc170916913b9bfc2bc0fefb6af776e8b542c561702f136cddc1c7aa43141012103acec3fc79dbbca745815c2a807dc4e81010c80e308e84913f59cb42a275dad97f3760100";

pub fn tx_from_hex(s: &str) -> Transaction {
    let raw = Vec::from_hex(s).expect("data must be in hex");
    consensus::deserialize(raw.as_slice()).expect("must deserialize")
}

pub fn new_hash<H: Hash>(s: &str) -> H {
    <H as bitcoin::hashes::Hash>::hash(s.as_bytes())
}

pub fn new_block_id(height: u32, hash: &str) -> BlockId {
    BlockId {
        height,
        hash: new_hash(hash),
    }
}