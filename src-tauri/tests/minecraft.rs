use pcl2_nova_app_lib::core::minecraft;



#[test]
fn version_list_get_test(){
    let res = tokio::runtime::Runtime::new().unwrap().block_on(minecraft::download::vanilla::get_manifest_version_list());
    match res {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("Error: {}", e)
    }
}