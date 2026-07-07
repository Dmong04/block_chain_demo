use block_chain_demo::Blockchain;

#[test]
fn full_workflow_stays_valid() {
    let mut chain = Blockchain::new(2);

    for i in 0..5 {
        chain.add_block(format!("transacción #{i}")).unwrap();
    }

    assert_eq!(chain.len(), 6); // 5 + génesis
    assert!(chain.is_valid());
}

#[test]
fn detects_tampering_via_public_api() {
    let mut chain = Blockchain::new(2);
    chain.add_block("dato original").unwrap();

    chain.blocks_mut()[1].data = "dato alterado".to_string();

    assert!(!chain.is_valid());
}

#[test]
fn persists_and_reloads_correctly() {
    let mut chain = Blockchain::new(2);
    chain.add_block("tx1").unwrap();
    chain.add_block("tx2").unwrap();

    let path = std::env::temp_dir().join("mini_blockchain_integration_test.json");
    chain.save_to_file(&path).unwrap();

    let reloaded = Blockchain::load_from_file(&path, 2).unwrap();
    assert_eq!(reloaded.len(), chain.len());
    assert!(reloaded.is_valid());

    std::fs::remove_file(path).unwrap();
}
