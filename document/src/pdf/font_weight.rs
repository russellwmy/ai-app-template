#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FontWeight {
    Weight100,
    Weight200,
    Weight300,
    Weight400Normal,
    Weight500,
    Weight600,
    Weight700Bold,
    Weight800,
    Weight900,

    Custom(u32),
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Weight400Normal
    }
}
