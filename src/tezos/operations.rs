pub enum MichelineValue {
    Primitive {
        prim: String,
        args: Option<Vec<MichelineTypes>>,
        annots: Option<Vec<String>>,
    },
    Bytes(Vec<u8>),
    Int(Vec<u8>),
    String(String),
    Address(String),
    Contract(String),
    Key(String),
    KeyHash(String),
    Signature(String),
    Array(Vec<MichelineTypes>),
}

pub enum MichelineExpression {
    Call {
        entrypoint: String,
        value: MichelineValue,
    },
    Value(MichelineValue),
}
