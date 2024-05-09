use std::{fs, path::Path};

use serde::Serialize;

use crate::coder::{Coder, EncodingType};

#[derive(Debug, Serialize, Default)]
pub struct TreeNode {
    pub label: String,
    pub path: String,
    pub is_file: bool,
    pub children: Vec<TreeNode>,
    pub encoding: EncodingType,
}

pub struct Collector {
    extention: String,
}

impl Collector {
    pub fn new(extention: &str) -> Collector {
        Collector {
            extention: extention.to_owned(),
        }
    }
    pub fn collect(&self, root_dir: &Path) -> anyhow::Result<TreeNode> {
        let mut root = TreeNode {
            label: root_dir.file_name().unwrap().to_string_lossy().to_string(),
            path: root_dir.to_string_lossy().to_string(),
            is_file: false,
            children: vec![],
            ..Default::default()
        };
        if root_dir.is_file() {
            root.is_file = true;
            if root.path.ends_with(&format!(".{}", self.extention)) {
                let coder = Coder::new(root_dir)?;
                root.encoding = coder.encoding();
            }
            return Ok(root);
        }
        for entry in fs::read_dir(root_dir)?.into_iter() {
            let entry = entry?;
            let child_path = root_dir.join(entry.file_name());
            let node = self.collect(&child_path)?;
            if (!node.is_file && !node.children.is_empty())
                || (node.is_file && node.label.ends_with(&format!(".{}", self.extention)))
            {
                root.children.push(node);
            }
        }
        Ok(root)
    }
}

#[cfg(test)]
mod collector_test {
    use super::*;
    #[test]
    fn collect_test() {
        let root_dir = Path::new(r"D:\Studies\ak112\303\stats\CSR");
        let extention = "sas";
        let collector = Collector::new(extention);
        let result = collector.collect(root_dir).unwrap();
        println!("{:?}", result);
    }
}
