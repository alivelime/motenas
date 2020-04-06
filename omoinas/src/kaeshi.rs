pub const CHAR: Kaeshi = Kaeshi {
    aisatsu: Aisatsu {
        hibi: Hibi {
            ohayo: "おはよう!"
        },
    },
};

pub struct Kaeshi {
    pub aisatsu: Aisatsu,
}
pub struct Aisatsu {
    pub hibi: Hibi,
}
pub struct Hibi {
    pub ohayo: &'static str,
}
