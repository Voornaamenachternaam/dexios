use crate::global::states::{HashMode, ForceMode};

use super::states::{
    Compression, DirectoryMode, EraseMode, EraseSourceDir, HeaderLocation, Key, PrintMode,
};

pub struct CryptoParams {
    pub hash_mode: HashMode,
    pub force: ForceMode,
    pub erase: EraseMode,
    pub key: Key,
    pub header_location: HeaderLocation,
}

pub struct PackParams {
    pub dir_mode: DirectoryMode,
    pub print_mode: PrintMode,
    pub erase_source: EraseSourceDir,
    pub compression: Compression,
}
