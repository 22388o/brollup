mod taproot;

use hex;
use musig2::secp256k1::PublicKey;
use musig2::KeyAggContext;
use taproot::*;

#[cfg(test)]
mod tests {
    use musig2::secp256k1::Parity;

    use super::*;

    #[test]
    fn test_key_aggregation() {
        let public_key_1: PublicKey =
            "022b1a426e4e68cf052240da0ef9256e6cb2c713adf5f9d1a6b51349d90fb00ca3"
                .parse()
                .unwrap();
        let public_key_2: PublicKey =
            "020982a79762a6e6c4fee6e166708147627bb7a90cd071ffd9dfc951feac871642"
                .parse()
                .unwrap();
        let public_key_3: PublicKey =
            "02963f498fc5e07c1c9c972608540933f56864c2aaf9166e9de2cbb50df0e153dd"
                .parse()
                .unwrap();

        let mut pubkeys: [PublicKey; 3] = [public_key_1, public_key_2, public_key_3];

        pubkeys.sort();

        let tap_script: Vec<u8> = vec![0x93, 0x93];

        let tap_leaf: TapLeaf = TapLeaf::new(tap_script);

        let key_agg_ctx: KeyAggContext = KeyAggContext::new(pubkeys)
            .unwrap()
            .with_taproot_tweak(&tap_leaf.hash())
            .unwrap();

        let agg_pubkey: PublicKey = key_agg_ctx.aggregated_pubkey();

        let agg_pubkey_without_tweak: PublicKey = key_agg_ctx.aggregated_pubkey_untweaked();

        println!(
            "aggregate pubkey without tweak: {} ",
            agg_pubkey_without_tweak.to_string()
        );
        println!("aggregate pubkey with tweak: {} ", agg_pubkey.to_string());
    }

    #[test]
    fn test_tap_branch() {
        // Test - Branch two TapLeaves

        let tap_leaf_1: TapLeaf = TapLeaf::new(vec![0xde, 0xad]);
        let tap_leaf_2: TapLeaf = TapLeaf::new(vec![0xbe, 0xef]);

        let tap_branch: TapBranch =
            TapBranch::new(tap_leaf_1.into_branch(), tap_leaf_2.into_branch());

        let expected: Vec<u8> =
            hex::decode("b220872a5f6915e7779e659c2925b4b6cef6c1792f2e7bed0ba6331631fa7c63")
                .unwrap();

        assert_eq!(tap_branch.hash_as_vec(), expected);

        // Test - Reversed order does not affect the branch

        let tap_branch_reversed: TapBranch =
            TapBranch::new(tap_leaf_2.into_branch(), tap_leaf_1.into_branch());

        assert_eq!(tap_branch_reversed.hash_as_vec(), expected);

        // Test - Branch two TapBranches

        let tap_leaf_3: TapLeaf = TapLeaf::new(vec![0xaa, 0xbb]);
        let tap_leaf_4: TapLeaf = TapLeaf::new(vec![0xcc, 0xdd]);

        let tap_branch_2: TapBranch =
            TapBranch::new(tap_leaf_3.into_branch(), tap_leaf_4.into_branch());

        let upper_tap_branch: TapBranch =
            TapBranch::new(tap_branch.into_branch(), tap_branch_2.into_branch());

        let expected_upper: Vec<u8> =
            hex::decode("a590e5a5cc3576cacb587676397bb8c7fa8645279ce740e5bf48bc7c25b1d813")
                .unwrap();

        assert_eq!(upper_tap_branch.hash_as_vec(), expected_upper);
    }

    #[test]
    fn test_taproot_key_and_script_path() {
        let tap_leaf: TapLeaf = TapLeaf::new(vec![0xaa, 0xbb, 0xcc]);

        // Test - with even inner key

        let inner_key_even: PublicKey =
            "028c17db0c798574086299e5041ffbcfa06bd501eb0e50914731bfbd2f3c9f980e"
                .parse()
                .unwrap();

        let taproot = TapRoot::key_and_script_path_single(inner_key_even, tap_leaf.clone());

        let expected_with_odd: Vec<u8> =
            hex::decode("51202e1a63521f2d72ff54da28cf8e114c6e3ce3ef497e9a6ac71b3e28e06446a218")
                .unwrap();

        assert_eq!(taproot.spk(), expected_with_odd);

        // Test - with odd inner key

        let inner_key_odd: PublicKey =
            "037b55a1c853b28c398141c8fdf4eb69469430f019983af4be4b5aa7512936f295"
                .parse()
                .unwrap();

        let taproot = TapRoot::key_and_script_path_single(inner_key_odd, tap_leaf.clone());

        let expected_with_even: Vec<u8> =
            hex::decode("51208cda55510b8f99ec248ed9772e6a71537eb26142d6624d38426a7a1311b488e6")
                .unwrap();

        assert_eq!(taproot.spk(), expected_with_even);
    }

    #[test]
    fn test_taproot_key_path_only() {
        // Test with even inner key

        let inner_key_even: PublicKey =
            "02d14c281713f15b608cc75d94717bbb1c2a4ff11e169c757f87a149daf61d54f0"
                .parse()
                .unwrap();

        let taproot_with_even_inner = TapRoot::key_path_only(inner_key_even);

        let expected_spk_with_inner =
            hex::decode("5120d14c281713f15b608cc75d94717bbb1c2a4ff11e169c757f87a149daf61d54f0")
                .unwrap();

        assert_eq!(taproot_with_even_inner.spk(), expected_spk_with_inner);

        // Test with odd inner key

        let inner_key_odd: PublicKey =
            "03a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed299"
                .parse()
                .unwrap();

        let taproot_with_odd_inner = TapRoot::key_path_only(inner_key_odd);

        let expected_spk_with_inner =
            hex::decode("5120a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed299")
                .unwrap();

        assert_eq!(taproot_with_odd_inner.spk(), expected_spk_with_inner);
    }

    #[test]
    fn test_taproot_script_path_only() {
        // Test with odd tweaked key

        let tap_leaf_with_odd: TapLeaf = TapLeaf::new(vec![0x01, 0x23, 0xab, 0xcd]);
        let tap_root_with_odd: TapRoot = TapRoot::script_path_only_single(tap_leaf_with_odd.clone());

        let expected_spk =
            hex::decode("512085dbf94f892274c41acb75d48daf338c739d1157c70963912db526c4cad30d1a")
                .unwrap();
        assert_eq!(tap_root_with_odd.spk(), expected_spk);
        assert_eq!(tap_root_with_odd.tweaked_key_parity(), Parity::Odd);

        // Test with even tweaked key

        let tap_leaf_with_odd: TapLeaf = TapLeaf::new(vec![0x01, 0x23, 0xab, 0xcd, 0xef, 0xff]);
        let tap_root_with_odd: TapRoot = TapRoot::script_path_only_single(tap_leaf_with_odd.clone());

        let expected_spk =
            hex::decode("51201fbb64a309f43ee6a442cd293a9df3ce3bbb0864a2215a1091c06521021f9de4")
                .unwrap();
        assert_eq!(tap_root_with_odd.spk(), expected_spk);
        assert_eq!(tap_root_with_odd.tweaked_key_parity(), Parity::Even);
    }

    #[test]
    fn test_control_block_create() {
        let inner_key: PublicKey =
            "03a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed299"
                .parse()
                .unwrap();

        let path: Vec<u8> =
            hex::decode("0576e0a5d1c8fd852ab17ffac14e336b3143298fad1d3d9a302212ec9b1f8202")
                .unwrap();

        let control_block = ControlBlock::new(inner_key.x_only_public_key().0, Parity::Odd, path);

        let expected_cb =
        hex::decode("c1a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed2990576e0a5d1c8fd852ab17ffac14e336b3143298fad1d3d9a302212ec9b1f8202")
            .unwrap();

        assert_eq!(control_block.to_vec(), expected_cb);
    }

    #[test]
    fn test_tap_tree() {
        let tap_leaf_1: TapLeaf = TapLeaf::new(vec![0xaa]);
        let tap_leaf_2: TapLeaf = TapLeaf::new(vec![0xbb]);
        let tap_leaf_3: TapLeaf = TapLeaf::new(vec![0xcc]);
        let tap_leaf_4: TapLeaf = TapLeaf::new(vec![0xdd]);
        let tap_leaf_5: TapLeaf = TapLeaf::new(vec![0xee]);
        let tap_leaf_6: TapLeaf = TapLeaf::new(vec![0xff]);
        let tap_leaf_7: TapLeaf = TapLeaf::new(vec![0x00]);
        let tap_leaf_8: TapLeaf = TapLeaf::new(vec![0x11]);
        let tap_leaf_9: TapLeaf = TapLeaf::new(vec![0x22]);
        let tap_leaf_10: TapLeaf = TapLeaf::new(vec![0x33]);
        let tap_leaf_11: TapLeaf = TapLeaf::new(vec![0x44]);
        let tap_leaf_12: TapLeaf = TapLeaf::new(vec![0x55]);

        let mut leaves: Vec<TapLeaf> = vec![];

        // Test single-leaf - aa
        leaves.push(tap_leaf_1);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("083b809ebc8a6e8077a1521d2621ef988887817d95691059b63db4efa6b354c8")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 2 leaves - aa bb
        leaves.push(tap_leaf_2);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8c")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 3 leaves - aa bb cc
        leaves.push(tap_leaf_3);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("fbacf98dc7eed29334d7f70ad70b78d8d0fd3362537f1f23d27fdbe7df302636")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 4 leaves - aa bb cc dd
        leaves.push(tap_leaf_4);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("4ab024178a74f8e2435cc88b8fd5c03cbb75d0e14b4e72e8388062b67be8e842")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 5 leaves - aa bb cc dd ee
        leaves.push(tap_leaf_5);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("fda09a939d87da777a274e0ad4232769445f15acd6b6e9d72053e4268354782d")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 6 leaves - aa bb cc dd ee ff
        leaves.push(tap_leaf_6);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("4ad803081bbcd04f49c4682d999ee748bf8400629a424f0c3dbad2638af45cc9")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 7 leaves - aa bb cc dd ee ff 00
        leaves.push(tap_leaf_7);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("73e54d9b7301cd6d8b528c16b801edba35347fcbf99da51abcc9727d43401ea7")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 8 leaves - aa bb cc dd ee ff 00 11
        leaves.push(tap_leaf_8);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("7648d42aead620a6ed02d82cc44a8e18a08da8ca1467928220ecf43ab308f195")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 9 leaves - aa bb cc dd ee ff 00 11 22
        leaves.push(tap_leaf_9);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("01efb5b091f906f27aa04dcb5a7a74938f736538a75df778acd66f3a968a310a")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 10 leaves - aa bb cc dd ee ff 00 11 22 33
        leaves.push(tap_leaf_10);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("3dad9105423be9dce1422e4f4f3ea6e49196104df08db7bcd8fd6d39591e79d4")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 11 leaves - aa bb cc dd ee ff 00 11 22 33 44
        leaves.push(tap_leaf_11);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("fe90a52c636872a7c7f1bc8faf59da361ad7d51d5bf88c883cc2dd268fa26b47")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);

        // Test 12 leaves - aa bb cc dd ee ff 00 11 22 33 44 55
        leaves.push(tap_leaf_12);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected =
            hex::decode("44446fb50fce9c698734e1bfd10ed894baaed244dc7ce67e4bf12b1d38760c30")
                .unwrap();

        assert_eq!(tap_tree.root(), expected);
    }

    #[test]
    fn test_tap_tree_path() {
        let tap_leaf_1: TapLeaf = TapLeaf::new(vec![0xaa]);
        let tap_leaf_2: TapLeaf = TapLeaf::new(vec![0xbb]);
        let tap_leaf_3: TapLeaf = TapLeaf::new(vec![0xcc]);
        let tap_leaf_4: TapLeaf = TapLeaf::new(vec![0xdd]);
        let tap_leaf_5: TapLeaf = TapLeaf::new(vec![0xee]);

        let mut leaves: Vec<TapLeaf> = vec![];

        // Test single-leaf - aa
        leaves.push(tap_leaf_1);
        leaves.push(tap_leaf_2);
        leaves.push(tap_leaf_3);
        leaves.push(tap_leaf_4);
        leaves.push(tap_leaf_5);

        let tap_tree: TapTree = TapTree::new(leaves.clone());

        let expected_path_1 =
            hex::decode("91af7e676faf0d787dd6628f8d068756dd2de2473b94e5aa63915f168764e8217f7b1fecf4af01c485881138c8484c4c7e6f537e896686a5e46d90e9b0c83692f6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")
                .unwrap();

        let expected_path_2 =
            hex::decode("083b809ebc8a6e8077a1521d2621ef988887817d95691059b63db4efa6b354c87f7b1fecf4af01c485881138c8484c4c7e6f537e896686a5e46d90e9b0c83692f6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")
                .unwrap();

        let expected_path_3 =
             hex::decode("b6537362191d9a5e0aa3a730b93b6f98a99ef63ed893bef4b9dfa7e3451eaf36823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8cf6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")
                 .unwrap();

        let expected_path_4 =
             hex::decode("fe06075904b2d09b06d544283b5ed7948355e691785c7b3e1a952a1a705151fe823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8cf6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")
                 .unwrap();

        let expected_path_5: Vec<u8> =
            hex::decode("4ab024178a74f8e2435cc88b8fd5c03cbb75d0e14b4e72e8388062b67be8e842")
                .unwrap();

        assert_eq!(tap_tree.path(0), expected_path_1);
        assert_eq!(tap_tree.path(1), expected_path_2);
        assert_eq!(tap_tree.path(2), expected_path_3);
        assert_eq!(tap_tree.path(3), expected_path_4);
        assert_eq!(tap_tree.path(4), expected_path_5);
    }
}

fn main() {}
