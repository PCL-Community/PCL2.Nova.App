use pcl2_nova_app_lib::core::minecraft;
use serde_json::json;

#[test]
fn asset_deserializer() {
    use crate::minecraft::minecraft::AssetObjects;
    let obj: AssetObjects = serde_json::from_value(
        json!(
            {
                "icons/icon_128x128.png": {
                    "hash": "b62ca8ec10d07e6bf5ac8dae0c8c1d2e6a1e3356",
                    "size": 9101
                },
                "icons/icon_16x16.png": {
                    "hash": "5ff04807c356f1beed0b86ccf659b44b9983e3fa",
                    "size": 781
                },
                "icons/icon_256x256.png": {
                    "hash": "8030dd9dc315c0381d52c4782ea36c6baf6e8135",
                    "size": 19642
                },
                "icons/icon_32x32.png": {
                    "hash": "af96f55a90eaf11b327f1b5f8834a051027dc506",
                    "size": 2063
                },
                "icons/icon_48x48.png": {
                    "hash": "b80b6e9ff01c78c624df5429e1d3dcd3d5130834",
                    "size": 3409
                }
            }
        )
    ).expect("");
    for v in obj.iter() {
        println!("Path: {}, Hash: {}, Size: {}", v.path, v.hash, v.size);
    }
}

#[test]
fn library_deserializer() {
    use crate::minecraft::minecraft::Library;
    let obj1: Library = serde_json::from_value(json!(
        {
            "downloads": {
                "artifact": {
                "path": "net/java/jutils/jutils/1.0.0/jutils-1.0.0.jar",
                "sha1": "e12fe1fda814bd348c1579329c86943d2cd3c6a6",
                "size": 7508,
                "url": "https://libraries.minecraft.net/net/java/jutils/jutils/1.0.0/jutils-1.0.0.jar"
                }
            },
            "name": "net.java.jutils:jutils:1.0.0"
        }
    )).expect("");
    let obj2: Library = serde_json::from_value(json!(
        {
            "downloads": {
                "classifiers": {
                    "natives-linux": {
                        "path": "org/lwjgl/lwjgl/lwjgl-platform/2.9.0/lwjgl-platform-2.9.0-natives-linux.jar",
                        "sha1": "2ba5dcb11048147f1a74eff2deb192c001321f77",
                        "size": 569061,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.0/lwjgl-platform-2.9.0-natives-linux.jar"
                    },
                    "natives-osx": {
                        "path": "org/lwjgl/lwjgl/lwjgl-platform/2.9.0/lwjgl-platform-2.9.0-natives-osx.jar",
                        "sha1": "6621b382cb14cc409b041d8d72829156a87c31aa",
                        "size": 518924,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.0/lwjgl-platform-2.9.0-natives-osx.jar"
                    },
                    "natives-windows": {
                        "path": "org/lwjgl/lwjgl/lwjgl-platform/2.9.0/lwjgl-platform-2.9.0-natives-windows.jar",
                        "sha1": "3f11873dc8e84c854ec7c5a8fd2e869f8aaef764",
                        "size": 609967,
                        "url": "https://libraries.minecraft.net/org/lwjgl/lwjgl/lwjgl-platform/2.9.0/lwjgl-platform-2.9.0-natives-windows.jar"
                    }
                }
            },
            "extract": {
                "exclude": [
                "META-INF/"
                ]
            },
            "name": "org.lwjgl.lwjgl:lwjgl-platform:2.9.0",
            "natives": {
                "linux": "natives-linux",
                "osx": "natives-osx",
                "windows": "natives-windows"
            },
            "rules": [
                {
                    "action": "allow"
                },
                {
                    "action": "disallow",
                    "os": {
                        "name": "osx",
                        "version": "^10\\.5\\.\\d$"
                    }
                }
            ]
        }
    )).expect("");


    println!("{} {}", obj1.native, obj2.native);
}