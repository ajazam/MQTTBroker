#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Byte(pub u8);

impl AsRef<u8> for Byte {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FourByteInteger(pub u32);

impl AsRef<u32> for FourByteInteger {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BinaryData(pub Vec<u8>);

impl AsRef<Vec<u8>> for BinaryData {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TwoByteInteger(pub u16);

impl AsRef<u16> for TwoByteInteger {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct VariableByteInteger(pub u32);

// max is 268_435_455
impl AsRef<u32> for VariableByteInteger {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Utf8EncodedString(pub String);

impl AsRef<String> for Utf8EncodedString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Utf8StringPair(pub String, pub String);

pub const MAX_VARIABLE_BYTE_INTEGER: u32 = 268_435_455;

impl Byte {
    pub fn new(value: u8) -> Byte {
        Byte(value)
    }
}

impl FourByteInteger {
    pub fn new(value: u32) -> FourByteInteger {
        FourByteInteger(value)
    }
}

impl BinaryData {
    pub fn new(value: Vec<u8>) -> BinaryData {
        BinaryData(value)
    }
}

impl TwoByteInteger {
    pub fn new(value: u16) -> TwoByteInteger {
        TwoByteInteger(value)
    }
}

impl VariableByteInteger {
    pub fn new(value: u32) -> VariableByteInteger {
        VariableByteInteger(value)
    }
}

impl Utf8EncodedString {
    pub fn new(value: String) -> Utf8EncodedString {
        Utf8EncodedString(value)
    }
}
