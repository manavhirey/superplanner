use sp_schema::{FilesystemIdentity, RegistryIdentity};

const DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn registry_identity_has_canonical_scalar_form() {
    let identity = RegistryIdentity {
        path: "/registry/record".to_string(),
        device: 1,
        inode: 2,
        link_count: 1,
        sha256: DIGEST.to_string(),
    };
    let scalar = identity.to_canonical_scalar().unwrap();
    assert_eq!(
        scalar,
        format!(
            "{{\"device\":1,\"inode\":2,\"link_count\":1,\"path\":\"/registry/record\",\"sha256\":\"{DIGEST}\"}}"
        )
    );
    assert_eq!(
        RegistryIdentity::from_canonical_scalar(&scalar).unwrap(),
        identity
    );
}

#[test]
fn identity_scalars_reject_noncanonical_or_unsafe_values() {
    let pretty = format!(
        "{{\n  \"device\": 1,\n  \"inode\": 2,\n  \"link_count\": 1,\n  \"path\": \"/registry/record\",\n  \"sha256\": \"{DIGEST}\"\n}}"
    );
    assert!(RegistryIdentity::from_canonical_scalar(&pretty).is_err());

    let mut bad_link = RegistryIdentity {
        path: "/registry/record".to_string(),
        device: 1,
        inode: 2,
        link_count: 2,
        sha256: DIGEST.to_string(),
    };
    assert!(bad_link.validate().is_err());
    bad_link.link_count = 1;
    bad_link.path = "/registry/../record".to_string();
    assert!(bad_link.validate().is_err());

    let bad_inode = FilesystemIdentity {
        path: "/custody/root".to_string(),
        device: 1,
        inode: 0,
    };
    assert!(bad_inode.validate().is_err());
}
