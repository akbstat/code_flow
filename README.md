# Code Flow

A tool for detecting encoding of sas files, and convert them into UTF-8 with BOM

# How to use

* list files with encoding
    ```rs
    #[test]
    fn list_files_test() {
        let path = Path::new(r"D:\macros");
        let result = list_files(path).unwrap();
        println!("{:?}", result);
    }
    ```

    Result:
    ```
    TreeNode { label: "macros", path: "D:\\macros", is_file: false, children: [TreeNode { label: "attrib_base_spec.sas", 
    path: "D:\\macros\\attrib_base_spec.sas", is_file: true, children: [], encoding: UTF8BOM }, TreeNode { label: "load_Trial_Design.sas", path: "D:\\macros\\load_Trial_Design.sas", is_file: true, children: [], encoding: UTF8 }, TreeNode { label: "VISIT.sas", path: "D:\\macros\\VISIT.sas", is_file: true, children: [], encoding: UTF8 }], encoding: Other }
    ```

* convert sas files into UTF-8 with BOM
    ```rs
    #[test]
    fn convert_files_test() {
        let filepath = Path::new(
            r"D:\macros\test.sas",
        );
        convert_to_utf8bom(&[filepath]).unwrap();
    }
    ```