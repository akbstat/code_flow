use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use chardet::detect;
use encoding_rs::GB18030;
use serde::Serialize;
use thiserror::Error;

pub const UTF8: &str = "UTF-8";
pub const ASCII: &str = "ASCII";
const BOM: &[u8] = &[239, 187, 191];
const PROBABILITY_AT_LEAST: f32 = 0.7;

#[derive(Error, Debug)]
pub struct ConvertError {
    file: String,
    #[source]
    source: anyhow::Error,
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.file, self.source)
    }
}

#[derive(Debug, PartialEq, Serialize, Default)]
pub enum EncodingType {
    UTF8,
    UTF8BOM,
    #[default]
    Other,
}

pub struct Coder {
    bytes: Vec<u8>,
    filepath: PathBuf,
}

impl Coder {
    pub fn new(filepath: &Path) -> anyhow::Result<Coder> {
        let bytes = fs::read(filepath)?;
        Ok(Coder {
            bytes,
            filepath: filepath.into(),
        })
    }
    pub fn encoding(&self) -> EncodingType {
        let (encode_result, probability, _) = detect(&self.bytes);
        let encode_result = encode_result.to_uppercase();
        if (encode_result.eq(UTF8) || encode_result.eq(ASCII))
            && probability.gt(&PROBABILITY_AT_LEAST)
        {
            if let Some(head) = self.bytes.get(0..3) {
                if BOM.eq(head) {
                    return EncodingType::UTF8BOM;
                }
            }
            return EncodingType::UTF8;
        }
        EncodingType::Other
    }
    pub fn convert_to_utf8bom(&self) -> anyhow::Result<(), ConvertError> {
        let bytes = match self.encoding() {
            EncodingType::UTF8 => {
                let mut bytes = self.bytes.clone();
                BOM.iter().rev().for_each(|c| {
                    bytes.insert(0, c.clone());
                });
                bytes
            }
            EncodingType::UTF8BOM => self.bytes.clone(),
            EncodingType::Other => {
                // if other encoding, then try to read as GB18030 and save as UTF-8 with BOM
                let (contents, _, _) = GB18030.decode(&self.bytes);
                let mut bytes = contents.to_string().as_bytes().to_owned();
                BOM.iter().rev().for_each(|c| {
                    bytes.insert(0, c.clone());
                });
                bytes
            }
        };
        if let Err(err) = fs::write(&self.filepath, bytes) {
            return Err(ConvertError {
                source: err.into(),
                file: self.filepath.to_string_lossy().to_string(),
            });
        };
        Ok(())
    }
}

#[cfg(test)]
mod code_test {
    use super::*;
    #[test]
    fn detect_encode_test() {
        let filepath =
            Path::new(r"D:\projects\rusty\mobius_kit\.mocks\code\generated\sdtm\dev\ae.sas");
        let code_file = Coder::new(filepath).unwrap();
        assert_eq!(code_file.encoding(), EncodingType::Other);
    }
    #[test]
    fn convert_to_utf8bom_test() {
        let filepath = Path::new(
            r"D:\Studies\ak112\303\stats\CSR\validation\dummy\program\macros\aecount5-v3-1.sas",
        );
        let coder = Coder::new(filepath).unwrap();
        assert_eq!(coder.encoding(), EncodingType::UTF8);
        coder.convert_to_utf8bom().unwrap();
        let coder = Coder::new(filepath).unwrap();
        assert_eq!(coder.encoding(), EncodingType::UTF8BOM);
    }
}
