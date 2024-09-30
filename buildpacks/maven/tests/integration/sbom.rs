use crate::default_build_config;
use libcnb::data::buildpack_id;
use libcnb::data::sbom::SbomFormat;
use libcnb_test::{SbomType, TestRunner};
use serde_cyclonedx::cyclonedx::v_1_4::{Component, CycloneDx, HashAlg};

#[test]
#[ignore = "integration test"]
pub(crate) fn sbom() {
    TestRunner::default().build(default_build_config("test-apps/simple-http-service"), |context| {
        context.download_sbom_files(|sbom_files| {
            let sbom_path = sbom_files.path_for(
                buildpack_id!("heroku/maven"),
                SbomType::Launch,
                SbomFormat::CycloneDxJson,
            );

            let sbom_simple_components = serde_json::from_str::<CycloneDx>(&std::fs::read_to_string(sbom_path).unwrap())
                .unwrap()
                .components
                .unwrap_or_default()
                .iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<SimpleSbomComponent>, _>>();

            assert_eq!(sbom_simple_components, Ok(vec![
                SimpleSbomComponent { purl: String::from("pkg:maven/io.undertow/undertow-core@2.3.12.Final?type=jar"), sha256_hash: String::from("3da2764c7a487e9bf196c9d28c95277648e0c510aa7449e17027b99a1052a53e"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.jboss.logging/jboss-logging@3.4.3.Final?type=jar"), sha256_hash: String::from("0b324cca4d550060e51e70cc0045a6cce62f264278ec1f5082aafeb670fcac49"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.jboss.xnio/xnio-api@3.8.8.Final?type=jar"), sha256_hash: String::from("701988bea1c7426d0cdbbd94c02141031cfe3001a470750e2d25b6ac166b7873"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.wildfly.common/wildfly-common@1.5.4.Final?type=jar"), sha256_hash: String::from("9fda3caf8bd528dec56ebc70daf78f5a9ff5d0bfcea8b3e41ab7ae838747e46a"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.wildfly.client/wildfly-client-config@1.0.1.Final?type=jar"), sha256_hash: String::from("80a4e963ce94ebb043ecb0f2c0e77d327f23dc87d81350b863752eedfa2c3bb3"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.jboss.xnio/xnio-nio@3.8.8.Final?type=jar"), sha256_hash: String::from("714c2d102c16aba245e5f50007bff49aba4d5e06c5303bd398df071c7614bc5f"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.jboss.threads/jboss-threads@3.5.0.Final?type=jar"), sha256_hash: String::from("e150b67a7f528525fe68dd60841520c22d59e0a831ea237c45a704de48b990b1"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/com.google.guava/guava@32.0.0-jre?type=jar"), sha256_hash: String::from("39f3550b0343d8d19dd4e83bd165b58ea3389d2ddb9f2148e63903f79ecdb114"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/com.google.guava/failureaccess@1.0.1?type=jar"), sha256_hash: String::from("a171ee4c734dd2da837e4b16be9df4661afab72a41adaf31eb84dfdaf936ca26"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/com.google.guava/listenablefuture@9999.0-empty-to-avoid-conflict-with-guava?type=jar"), sha256_hash: String::from("b372a037d4230aa57fbeffdef30fd6123f9c0c2db85d0aced00c91b974f33f99"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/com.google.code.findbugs/jsr305@3.0.2?type=jar"), sha256_hash: String::from("766ad2a0783f2687962c8ad74ceecc38a28b9f72a2d085ee438b7813e928d0c7"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/org.checkerframework/checker-qual@3.33.0?type=jar"), sha256_hash: String::from("e316255bbfcd9fe50d165314b85abb2b33cb2a66a93c491db648e498a82c2de1"), main_license_id: String::from("MIT") },
                SimpleSbomComponent { purl: String::from("pkg:maven/com.google.errorprone/error_prone_annotations@2.18.0?type=jar"), sha256_hash: String::from("9e6814cb71816988a4fd1b07a993a8f21bb7058d522c162b1de849e19bea54ae"), main_license_id: String::from("Apache-2.0") },
                SimpleSbomComponent { purl: String::from("pkg:maven/com.google.j2objc/j2objc-annotations@2.8?type=jar"), sha256_hash: String::from("f02a95fa1a5e95edb3ed859fd0fb7df709d121a35290eff8b74dce2ab7f4d6ed"), main_license_id: String::from("Apache-2.0") }
            ]));
        });
    });
}

/// A simple representation of an CycloneDX SBOM component for testing purposes.
#[derive(Debug, Eq, PartialEq)]
struct SimpleSbomComponent {
    purl: String,
    sha256_hash: String,
    main_license_id: String,
}

impl TryFrom<&Component> for SimpleSbomComponent {
    type Error = ();

    fn try_from(component: &Component) -> Result<Self, Self::Error> {
        Ok(SimpleSbomComponent {
            purl: component.purl.clone().ok_or(())?,
            sha256_hash: component
                .hashes
                .clone()
                .unwrap_or_default()
                .into_iter()
                .find(|hash| hash.alg == HashAlg::Sha256)
                .map(|hash| hash.content)
                .ok_or(())?,
            main_license_id: component
                .licenses
                .clone()
                .and_then(|license_choices| {
                    license_choices.first().and_then(|license_choice| {
                        license_choice
                            .license
                            .clone()
                            .and_then(|license| license.id)
                    })
                })
                .ok_or(())?,
        })
    }
}
