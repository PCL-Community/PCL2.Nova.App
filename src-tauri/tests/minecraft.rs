use pcl2_nova_app_lib::core::minecraft;

#[test]
fn version_list_get_test() {
    let res = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(minecraft::download::vanilla::VersionManifestOverall::new());
    match res {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn version_download_test() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let res = minecraft::download::vanilla::VersionManifestOverall::new().await;
        match res {
            Ok(v) => {
                let latest = v.versions.get(0).unwrap();
                println!("Start: {:?}", latest);
                latest
                    .download(dirs_next::desktop_dir().unwrap())
                    .await
                    .unwrap();
            }
            Err(e) => println!("Error: {}", e),
        }
    });
}
