use std::path::Path;

use coder::Coder;
use collector::{Collector, TreeNode};

const SAS_EXTENTION: &str = "sas";

mod coder;
mod collector;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn list_files(path: &Path) -> anyhow::Result<TreeNode> {
    let collector = Collector::new(SAS_EXTENTION);
    collector.collect(path)
}

pub fn convert_to_utf8bom(paths: &[&Path]) -> anyhow::Result<()> {
    for p in paths {
        let coder = Coder::new(p)?;
        coder.convert_to_utf8bom()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_files_test() {
        let path = Path::new(r"D:\Studies\ak112\303\stats\CSR\product\program\macros");
        let result = list_files(path).unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn convert_files_test() {
        let filepath = Path::new(
            r"D:\Studies\ak112\303\stats\CSR\product\program\macros\attrib_base_spec.sas",
        );
        convert_to_utf8bom(&[filepath]).unwrap();
    }
}
