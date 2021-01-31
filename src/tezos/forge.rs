use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

/// Get Script Expression
///
/// Used for big map indexing
/// this impl is transcribed from go-tezos
/// https://github.com/goat-systems/go-tezos/blob/master/forge/forge.go#L1236
pub fn get_script_expr(index: &str) -> String {
    bs58::encode(
        [
            // prefix "expr"
            [13u8, 44u8, 64u8, 27u8].as_ref(),
            &{
                let mut dig = VarBlake2b::new(32).unwrap();
                dig.update(
                    [
                        // type confirmation bytes?
                        [0x05, 0x01].as_ref(),
                        // length as bytes, minimum width (should never panic as long as format! does correct hex)
                        &hex::decode(format!("{:x}", index.len())).unwrap(),
                        // value bytes
                        index.as_bytes(),
                    ]
                    .concat(),
                );
                dig.finalize_boxed()
            },
        ]
        .concat(),
    )
    .with_check()
    .into_string()
}

#[test]
fn script_expr() {
    // taken from the tezos Go implementation tests
    assert_eq!(
        "expruGmscHLuUazE7d79EepWCnDuPJreo8R87wsDGUgKAuH4E5ayEj",
        get_script_expr("Tezos Tacos Nachos")
    )
}
