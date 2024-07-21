mod serialize;
mod taproot;
mod txo;
mod well_known;

#[cfg(test)]
mod tests {
    use crate::txo::connector::Connector;
    use crate::txo::lift::Lift;
    use crate::txo::projector::{Projector, ProjectorTag};

    use crate::serialize::{self, *};
    use crate::taproot::{ControlBlock, TapBranch, TapLeaf, TapRoot, TapTree};
    use musig2::secp256k1::{Parity, PublicKey, XOnlyPublicKey};
    use musig2::KeyAggContext;
    use std::error::Error;

    #[test]
    fn test_key_aggregation() -> Result<(), Box<dyn Error>> {
        let public_key_1: PublicKey =
            "022b1a426e4e68cf052240da0ef9256e6cb2c713adf5f9d1a6b51349d90fb00ca3".parse()?;
        let public_key_2: PublicKey =
            "020982a79762a6e6c4fee6e166708147627bb7a90cd071ffd9dfc951feac871642".parse()?;
        let public_key_3: PublicKey =
            "02963f498fc5e07c1c9c972608540933f56864c2aaf9166e9de2cbb50df0e153dd".parse()?;

        let mut pubkeys: [PublicKey; 3] = [public_key_1, public_key_2, public_key_3];
        pubkeys.sort();

        let tap_script: Vec<u8> = vec![0x93, 0x93];
        let tap_leaf: TapLeaf = TapLeaf::new(tap_script);

        let key_agg_ctx: KeyAggContext =
            KeyAggContext::new(pubkeys)?.with_taproot_tweak(&tap_leaf.hash())?;

        let agg_pubkey: PublicKey = key_agg_ctx.aggregated_pubkey();
        let agg_pubkey_without_tweak: PublicKey = key_agg_ctx.aggregated_pubkey_untweaked();

        println!(
            "aggregate pubkey without tweak: {} ",
            agg_pubkey_without_tweak.to_string()
        );
        println!("aggregate pubkey with tweak: {} ", agg_pubkey.to_string());

        Ok(())
    }

    #[test]
    fn test_tap_branch() -> Result<(), Box<dyn Error>> {
        // Test - Branch two TapLeaves

        let tap_leaf_1: TapLeaf = TapLeaf::new(vec![0xde, 0xad]);
        let tap_leaf_2: TapLeaf = TapLeaf::new(vec![0xbe, 0xef]);

        let tap_branch: TapBranch =
            TapBranch::new(tap_leaf_1.into_branch(), tap_leaf_2.into_branch());

        let expected: Vec<u8> =
            hex::decode("b220872a5f6915e7779e659c2925b4b6cef6c1792f2e7bed0ba6331631fa7c63")?;

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
            hex::decode("a590e5a5cc3576cacb587676397bb8c7fa8645279ce740e5bf48bc7c25b1d813")?;

        assert_eq!(upper_tap_branch.hash_as_vec(), expected_upper);

        Ok(())
    }

    #[test]
    fn test_taproot_key_and_script_path() -> Result<(), Box<dyn Error>> {
        let tap_leaf: TapLeaf = TapLeaf::new(vec![0xaa, 0xbb, 0xcc]);

        // Test - with even inner key

        let inner_key_even: PublicKey =
            "028c17db0c798574086299e5041ffbcfa06bd501eb0e50914731bfbd2f3c9f980e".parse()?;

        let taproot = TapRoot::key_and_script_path_single(inner_key_even, tap_leaf.clone());

        let expected_with_odd: Vec<u8> =
            hex::decode("51202e1a63521f2d72ff54da28cf8e114c6e3ce3ef497e9a6ac71b3e28e06446a218")?;

        assert_eq!(taproot.spk()?, expected_with_odd);

        // Test - with odd inner key

        let inner_key_odd: PublicKey =
            "037b55a1c853b28c398141c8fdf4eb69469430f019983af4be4b5aa7512936f295".parse()?;

        let taproot = TapRoot::key_and_script_path_single(inner_key_odd, tap_leaf.clone());

        let expected_with_even: Vec<u8> =
            hex::decode("51208cda55510b8f99ec248ed9772e6a71537eb26142d6624d38426a7a1311b488e6")?;

        assert_eq!(taproot.spk()?, expected_with_even);

        Ok(())
    }

    #[test]
    fn test_taproot_key_path_only() -> Result<(), Box<dyn Error>> {
        // Test with even inner key

        let inner_key_even: PublicKey =
            "02d14c281713f15b608cc75d94717bbb1c2a4ff11e169c757f87a149daf61d54f0".parse()?;

        let taproot_with_even_inner = TapRoot::key_path_only(inner_key_even);

        let expected_spk_with_inner =
            hex::decode("5120d14c281713f15b608cc75d94717bbb1c2a4ff11e169c757f87a149daf61d54f0")?;

        assert_eq!(taproot_with_even_inner.spk()?, expected_spk_with_inner);

        // Test with odd inner key

        let inner_key_odd: PublicKey =
            "03a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed299".parse()?;

        let taproot_with_odd_inner = TapRoot::key_path_only(inner_key_odd);

        let expected_spk_with_inner =
            hex::decode("5120a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed299")?;

        assert_eq!(taproot_with_odd_inner.spk()?, expected_spk_with_inner);

        Ok(())
    }

    #[test]
    fn test_taproot_script_path_only() -> Result<(), Box<dyn Error>> {
        // Test with odd tweaked key

        let tap_leaf_with_odd = TapLeaf::new(vec![0x01, 0x23, 0xab, 0xcd]);
        let tap_root_with_odd = TapRoot::script_path_only_single(tap_leaf_with_odd.clone());

        let expected_spk =
            hex::decode("512085dbf94f892274c41acb75d48daf338c739d1157c70963912db526c4cad30d1a")?;
        assert_eq!(tap_root_with_odd.spk()?, expected_spk);
        assert_eq!(tap_root_with_odd.tweaked_key_parity()?, Parity::Odd);

        // Test with even tweaked key

        let tap_leaf_with_even = TapLeaf::new(vec![0x01, 0x23, 0xab, 0xcd, 0xef, 0xff]);
        let tap_root_with_even = TapRoot::script_path_only_single(tap_leaf_with_even.clone());

        let expected_spk =
            hex::decode("51201fbb64a309f43ee6a442cd293a9df3ce3bbb0864a2215a1091c06521021f9de4")?;
        assert_eq!(tap_root_with_even.spk()?, expected_spk);
        assert_eq!(tap_root_with_even.tweaked_key_parity()?, Parity::Even);

        Ok(())
    }

    #[test]
    fn test_control_block() -> Result<(), Box<dyn Error>> {
        let tap_leaf_single: TapLeaf = TapLeaf::new(vec![0xaa, 0xbb, 0xcc]);
        let tap_root_single_leaf: TapRoot = TapRoot::script_path_only_multi(vec![tap_leaf_single]);

        let expected_cb: Vec<u8> =
            hex::decode("c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0")?;

        assert_eq!(tap_root_single_leaf.control_block(0)?.to_vec(), expected_cb);

        let tap_leaf_1: TapLeaf = TapLeaf::new(vec![0xaa]);
        let tap_leaf_2: TapLeaf = TapLeaf::new(vec![0xbb]);
        let tap_leaf_3: TapLeaf = TapLeaf::new(vec![0xcc]);

        let leaves: Vec<TapLeaf> = vec![tap_leaf_1, tap_leaf_2, tap_leaf_3];

        let tap_root: TapRoot = TapRoot::script_path_only_multi(leaves);

        let expected_cb_1 =
            hex::decode("c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac091af7e676faf0d787dd6628f8d068756dd2de2473b94e5aa63915f168764e821fe06075904b2d09b06d544283b5ed7948355e691785c7b3e1a952a1a705151fe")?;

        let expected_cb_2 =
            hex::decode("c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0083b809ebc8a6e8077a1521d2621ef988887817d95691059b63db4efa6b354c8fe06075904b2d09b06d544283b5ed7948355e691785c7b3e1a952a1a705151fe")?;

        let expected_cb_3 =
            hex::decode("c050929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8c")?;

        assert_eq!(tap_root.control_block(0)?.to_vec(), expected_cb_1);
        assert_eq!(tap_root.control_block(1)?.to_vec(), expected_cb_2);
        assert_eq!(tap_root.control_block(2)?.to_vec(), expected_cb_3);

        Ok(())
    }

    #[test]
    fn test_control_block_create() -> Result<(), Box<dyn Error>> {
        let inner_key: PublicKey =
            "03a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed299".parse()?;

        let path: Vec<u8> =
            hex::decode("0576e0a5d1c8fd852ab17ffac14e336b3143298fad1d3d9a302212ec9b1f8202")?;

        let control_block = ControlBlock::new(inner_key.x_only_public_key().0, Parity::Odd, path);

        let expected_cb = hex::decode("c1a2314467943d47cf102477b985d21c5ffa6512961b08906724f13e779cfed2990576e0a5d1c8fd852ab17ffac14e336b3143298fad1d3d9a302212ec9b1f8202")?;

        assert_eq!(control_block.to_vec(), expected_cb);

        Ok(())
    }

    #[test]
    fn test_tap_tree() -> Result<(), Box<dyn Error>> {
        let tap_leaf_1 = TapLeaf::new(vec![0xaa]);
        let tap_leaf_2 = TapLeaf::new(vec![0xbb]);
        let tap_leaf_3 = TapLeaf::new(vec![0xcc]);
        let tap_leaf_4 = TapLeaf::new(vec![0xdd]);
        let tap_leaf_5 = TapLeaf::new(vec![0xee]);
        let tap_leaf_6 = TapLeaf::new(vec![0xff]);
        let tap_leaf_7 = TapLeaf::new(vec![0x00]);
        let tap_leaf_8 = TapLeaf::new(vec![0x11]);
        let tap_leaf_9 = TapLeaf::new(vec![0x22]);
        let tap_leaf_10 = TapLeaf::new(vec![0x33]);
        let tap_leaf_11 = TapLeaf::new(vec![0x44]);
        let tap_leaf_12 = TapLeaf::new(vec![0x55]);

        let mut leaves = vec![];

        // Test single-leaf - aa
        leaves.push(tap_leaf_1.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("083b809ebc8a6e8077a1521d2621ef988887817d95691059b63db4efa6b354c8")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 2 leaves - aa bb
        leaves.push(tap_leaf_2.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8c")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 3 leaves - aa bb cc
        leaves.push(tap_leaf_3.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("fbacf98dc7eed29334d7f70ad70b78d8d0fd3362537f1f23d27fdbe7df302636")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 4 leaves - aa bb cc dd
        leaves.push(tap_leaf_4.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("4ab024178a74f8e2435cc88b8fd5c03cbb75d0e14b4e72e8388062b67be8e842")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 5 leaves - aa bb cc dd ee
        leaves.push(tap_leaf_5.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("fda09a939d87da777a274e0ad4232769445f15acd6b6e9d72053e4268354782d")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 6 leaves - aa bb cc dd ee ff
        leaves.push(tap_leaf_6.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("4ad803081bbcd04f49c4682d999ee748bf8400629a424f0c3dbad2638af45cc9")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 7 leaves - aa bb cc dd ee ff 00
        leaves.push(tap_leaf_7.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("73e54d9b7301cd6d8b528c16b801edba35347fcbf99da51abcc9727d43401ea7")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 8 leaves - aa bb cc dd ee ff 00 11
        leaves.push(tap_leaf_8.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("7648d42aead620a6ed02d82cc44a8e18a08da8ca1467928220ecf43ab308f195")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 9 leaves - aa bb cc dd ee ff 00 11 22
        leaves.push(tap_leaf_9.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("01efb5b091f906f27aa04dcb5a7a74938f736538a75df778acd66f3a968a310a")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 10 leaves - aa bb cc dd ee ff 00 11 22 33
        leaves.push(tap_leaf_10.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("3dad9105423be9dce1422e4f4f3ea6e49196104df08db7bcd8fd6d39591e79d4")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 11 leaves - aa bb cc dd ee ff 00 11 22 33 44
        leaves.push(tap_leaf_11.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("fe90a52c636872a7c7f1bc8faf59da361ad7d51d5bf88c883cc2dd268fa26b47")?;
        assert_eq!(tap_tree.root(), expected);

        // Test 12 leaves - aa bb cc dd ee ff 00 11 22 33 44 55
        leaves.push(tap_leaf_12.clone());
        let tap_tree = TapTree::new(leaves.clone());
        let expected =
            hex::decode("44446fb50fce9c698734e1bfd10ed894baaed244dc7ce67e4bf12b1d38760c30")?;
        assert_eq!(tap_tree.root(), expected);

        Ok(())
    }

    #[test]
    fn test_tap_tree_path() -> Result<(), Box<dyn Error>> {
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
            hex::decode("91af7e676faf0d787dd6628f8d068756dd2de2473b94e5aa63915f168764e8217f7b1fecf4af01c485881138c8484c4c7e6f537e896686a5e46d90e9b0c83692f6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")?;

        let expected_path_2 =
            hex::decode("083b809ebc8a6e8077a1521d2621ef988887817d95691059b63db4efa6b354c87f7b1fecf4af01c485881138c8484c4c7e6f537e896686a5e46d90e9b0c83692f6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")?;

        let expected_path_3 =
            hex::decode("b6537362191d9a5e0aa3a730b93b6f98a99ef63ed893bef4b9dfa7e3451eaf36823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8cf6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")?;

        let expected_path_4 =
            hex::decode("fe06075904b2d09b06d544283b5ed7948355e691785c7b3e1a952a1a705151fe823a89de31a35a726355b97b88e0f8fa0692fbf38630ebed328478f17c054a8cf6f920dc9dcb98ba04cff112f583969b4fa240bedf781759d6b6e0f2e74eb7fa")?;

        let expected_path_5: Vec<u8> =
            hex::decode("4ab024178a74f8e2435cc88b8fd5c03cbb75d0e14b4e72e8388062b67be8e842")?;

        assert_eq!(tap_tree.path(0), expected_path_1);
        assert_eq!(tap_tree.path(1), expected_path_2);
        assert_eq!(tap_tree.path(2), expected_path_3);
        assert_eq!(tap_tree.path(3), expected_path_4);
        assert_eq!(tap_tree.path(4), expected_path_5);

        Ok(())
    }

    #[test]
    fn test_tap_tree_64() -> Result<(), Box<dyn Error>> {
        let mut leaves = Vec::<TapLeaf>::new();

        for i in 0..64 {
            leaves.push(TapLeaf::new(vec![i as u8]));
        }
        let tap_root = TapRoot::script_path_only_multi(leaves);

        let expected_spk =
            hex::decode("5120b88bb9de3afa63f0cd5b533f70a58f60004b65b6a1b6683a1ba766e37b11455b")?;
        assert_eq!(tap_root.spk()?, expected_spk);

        let expected_cb_leaf_0 = hex::decode("c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac01aac25408a4d28233cd325faefade9ef0fae76fcb1e35d08140045bbaa381b30c01491a3c808832bdf3bab5ea3208726c6eae12c3db3f8098919e145caa981de8fd6e8cdf1c53b7b1509958f4288d46fcc6c172dc9d32a52c0f8af4d5f86efc369632feaaca2e76395ae30e30fa5211fc0c099997a7de3a80d6ac566bdef300b7a41ea55777781977241267979150a1654dd92eecd7eb820b4aae57967a28952a2489c6a8c3011b12b89148d2abafa042d7982533826d3b911851abb34e7e741")?;
        assert_eq!(tap_root.control_block(0)?.to_vec(), expected_cb_leaf_0);

        let expected_cb_leaf_10 = hex::decode("c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac033f0e118ce1dc6ee36199de7766e39d5a37da363e63cdb5342fac9e437c98261e2100cc38af83fde32fdb302ce68109844fa99f71ba58721f7cdbf7c3083ccae9602fd5610cc4ce5ff81afb18acd5140c2c2525e61e0ae7bfc335d1457df2352c33b3b99fa0737f5da94cfb3fe918e3b8467ed9d546588a117531672f48928657a41ea55777781977241267979150a1654dd92eecd7eb820b4aae57967a28952a2489c6a8c3011b12b89148d2abafa042d7982533826d3b911851abb34e7e741")?;
        assert_eq!(tap_root.control_block(10)?.to_vec(), expected_cb_leaf_10);

        let expected_cb_leaf_45 = hex::decode("c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac03fe216e39e56269ca739d5e48e09bc93de208b9ebdbe524f665d28e103c86fe6c4154feadf35d5527875e82839e0878e9b6f18c4096c652830beee93cff38219923ebb8ebff4c5a8907da345ac47ce386249f745e8f2e942de33050358d20b289430f4b106bf5617e6d11333464d368b33b0433bf1f3d32ce840ecb65ac92d84c350786781aec83736c548e62ac04427a1747036cb212292bc4011aecb275e6326f6c6d5df019644b4aa8fa2116fe6c09bcc83bdedb621e2443a69218954063b")?;
        assert_eq!(tap_root.control_block(45)?.to_vec(), expected_cb_leaf_45);

        let expected_cb_leaf_61 = hex::decode("c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac06925381e2d092124c53f87297a6e68f07ed3132a9761684bcaa475ea4fcf248dc870c97467df9cb37e9b481d0296b2660b23ef76ed7f84dee951c0d90b54aef97403758af8698bc5cdf75ca317b1036d1c0a33d9834962095693fc6b72ed68b2082edeb867fd98827cca5c1a0c7b517910712bb20e7c97d7ea50b273c5b19ddcaffb3ddcecd12cc515b9487bdd4b9497a9efa05e22b3c00bad374b7dce8c5f9c26f6c6d5df019644b4aa8fa2116fe6c09bcc83bdedb621e2443a69218954063b")?;
        assert_eq!(tap_root.control_block(61)?.to_vec(), expected_cb_leaf_61);

        let expected_cb_leaf_63 = hex::decode("c150929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0a0a62d83f5b0ca6f2623bf7e2c347a9c8c4f950918cafad4ab742ab5c2ae04bebbc8d5c38114a3884f07c076229288ba618fe866ed324ab2e67b482f3c1965607403758af8698bc5cdf75ca317b1036d1c0a33d9834962095693fc6b72ed68b2082edeb867fd98827cca5c1a0c7b517910712bb20e7c97d7ea50b273c5b19ddcaffb3ddcecd12cc515b9487bdd4b9497a9efa05e22b3c00bad374b7dce8c5f9c26f6c6d5df019644b4aa8fa2116fe6c09bcc83bdedb621e2443a69218954063b")?;
        assert_eq!(tap_root.control_block(63)?.to_vec(), expected_cb_leaf_63);

        Ok(())
    }

    #[test]
    fn test_prefix_pushdata() {
        let data_1 = hex::decode("aa").unwrap();
        let expected_1 = hex::decode("01aa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_1), expected_1);

        let data_2 = hex::decode("aaaa").unwrap();
        let expected_2 = hex::decode("02aaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_2), expected_2);

        let data_3 = hex::decode("aaaaaaaaaa").unwrap();
        let expected_3 = hex::decode("05aaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_3), expected_3);

        let data_4 = hex::decode(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap();
        let expected_4 = hex::decode("2aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_4), expected_4);

        let data_5 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_5 = hex::decode("4baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_5), expected_5);

        let data_6 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_6 = hex::decode("4c4daaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_6), expected_6);

        let data_7 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_7 = hex::decode("4d0a01aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(with_prefix_pushdata(&data_7), expected_7);

        // Minimal pushes

        let data_8 = hex::decode("").unwrap();
        let expected_8 = hex::decode("00").unwrap();

        assert_eq!(with_prefix_pushdata(&data_8), expected_8);

        let data_9 = hex::decode("01").unwrap();
        let expected_9 = hex::decode("51").unwrap();

        assert_eq!(with_prefix_pushdata(&data_9), expected_9);

        let data_10 = hex::decode("09").unwrap();
        let expected_10 = hex::decode("59").unwrap();

        assert_eq!(with_prefix_pushdata(&data_10), expected_10);

        let data_11 = hex::decode("0a").unwrap();
        let expected_11 = hex::decode("5a").unwrap();

        assert_eq!(with_prefix_pushdata(&data_11), expected_11);

        let data_12 = hex::decode("0f").unwrap();
        let expected_12 = hex::decode("5f").unwrap();

        assert_eq!(with_prefix_pushdata(&data_12), expected_12);

        let data_13 = hex::decode("10").unwrap();
        let expected_13 = hex::decode("60").unwrap();

        assert_eq!(with_prefix_pushdata(&data_13), expected_13);

        let data_14 = hex::decode("11").unwrap();
        let not_expected_14 = hex::decode("61").unwrap();
        let expected_14 = hex::decode("0111").unwrap();

        assert_eq!(with_prefix_pushdata(&data_14), expected_14);
        assert_ne!(with_prefix_pushdata(&data_14), not_expected_14);
    }

    #[test]
    fn test_chunkify() {
        // Test empty

        let data = hex::decode("").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::ScriptPush);

        assert_eq!(chunks.len(), 1);

        let first: Vec<u8> = chunks[0].clone();
        let expected = hex::decode("").unwrap();

        assert_eq!(first, expected);

        // Test single-byte

        let data = hex::decode("aa").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::ScriptPush);

        assert_eq!(chunks.len(), 1);

        let first: Vec<u8> = chunks[0].clone();
        let expected = hex::decode("aa").unwrap();

        assert_eq!(first, expected);

        // Test multi-bytes

        let data = hex::decode("deadbeef").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::ScriptPush);

        assert_eq!(chunks.len(), 1);

        let first: Vec<u8> = chunks[0].clone();
        let expected = hex::decode("deadbeef").unwrap();

        assert_eq!(first, expected);

        // Test script_push full minus one

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::ScriptPush);

        assert_eq!(chunks.len(), 1);

        let first: Vec<u8> = chunks[0].clone();
        let expected = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(first, expected);

        // Test script_push full

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::ScriptPush);

        assert_eq!(chunks.len(), 1);

        let first: Vec<u8> = chunks[0].clone();
        let expected = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(first, expected);

        // Test script_push full plus one

        let data = hex::decode("001122334455aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa66778899aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa101112131415").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::ScriptPush);

        assert_eq!(chunks.len(), 2);

        // 261 bytes
        let first: Vec<u8> = chunks[0].clone();
        let expected = hex::decode("001122334455aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa6677").unwrap();

        assert_eq!(first, expected);

        // 260 bytes
        let second: Vec<u8> = chunks[1].clone();
        let expected = hex::decode("8899aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa101112131415").unwrap();

        assert_eq!(second, expected);

        // Test non-standard witness push

        let data = hex::decode("001122334455aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa66778899aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa101112131415").unwrap();
        let chunks = serialize::chunkify(&data, serialize::PushFlag::StandardWitnessPush);

        let expected_1 = hex::decode("001122334455aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_2 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_3 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_4 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa66778899aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_5 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_6 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let expected_7 = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa101112131415").unwrap();

        assert_eq!(chunks[0].clone(), expected_1);
        assert_eq!(chunks[1].clone(), expected_2);
        assert_eq!(chunks[2].clone(), expected_3);
        assert_eq!(chunks[3].clone(), expected_4);
        assert_eq!(chunks[4].clone(), expected_5);
        assert_eq!(chunks[5].clone(), expected_6);
        assert_eq!(chunks[6].clone(), expected_7);
    }

    #[test]
    fn test_encode_multi_push() {
        // Test empty - Script Push

        let data = hex::decode("").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("00").unwrap();

        assert_eq!(encoded, expected);

        // Test empty - Standard Witness Push

        let data = hex::decode("").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("00").unwrap();

        assert_eq!(encoded, expected);

        // Test empty - Non-standard Witness Push

        let data = hex::decode("").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("00").unwrap();

        assert_eq!(encoded, expected);

        // Test single-byte - Script Push

        let data = hex::decode("aa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("01aa").unwrap();

        assert_eq!(encoded, expected);

        // Test single-byte - Standard Witness Push

        let data = hex::decode("aa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("01aa").unwrap();

        assert_eq!(encoded, expected);

        // Test single-byte - Non-standard Witness Push

        let data = hex::decode("aa").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("01aa").unwrap();

        assert_eq!(encoded, expected);

        // Test multi-byte - Script Push

        let data = hex::decode("deadbeef").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("04deadbeef").unwrap();

        assert_eq!(encoded, expected);

        // Test multi-byte - Standard Witness Push

        let data = hex::decode("deadbeef").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("04deadbeef").unwrap();

        assert_eq!(encoded, expected);

        // Test multi-byte - Non-standard Witness Push

        let data = hex::decode("deadbeef").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("04deadbeef").unwrap();

        assert_eq!(encoded, expected);

        // Test 75-bytes - Script Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("4baaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 76-bytes - Script Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("4c4caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 255-bytes - Script Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("4cffaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 256-bytes - Script Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("4d0001aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 520-bytes - Script Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("4d0802aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 521-bytes - Script Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::ScriptPush);
        let expected = hex::decode("4d0501aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4d0401aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 252-bytes - No-standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("fcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        //assert_eq!(encoded, expected);

        println!("letsee {}", hex::encode(encoded));

        // Test 253-bytes - No-standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("fdfd00aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        println!("letsee {}", hex::encode(encoded));

        // Test 520-bytes - No-standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("fd0802aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 521-bytes - No-standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded =
            serialize::encode_multi_push(&data, serialize::PushFlag::NonStandardWitnessPush);
        let expected = hex::decode("fd0501aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaafd0401aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 80-bytes - Standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("50aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 81-bytes - Standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("29aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa28aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 159-bytes - Standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("50aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa4faaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 160-bytes - Standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("50aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa50aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);

        // Test 161-bytes - Standard Witness Push

        let data = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let encoded = serialize::encode_multi_push(&data, serialize::PushFlag::StandardWitnessPush);
        let expected = hex::decode("36aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa36aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa35aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_lift() {
        let self_key: XOnlyPublicKey =
            "b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421"
                .parse()
                .unwrap();

        let lift_txo = Lift::new(self_key);

        println!("spkis {}", hex::encode(lift_txo.spk().unwrap()));

        let tree = lift_txo.taproot().tree();

        if let Some(tree) = tree {
            let collab_path = tree.leaves()[0].tap_script();
            let exit_path = tree.leaves()[1].tap_script();

            let collab_path_expected = hex::decode("20b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421ad20fe44f87e8dcf65392e213f304bee1e3a31e562bc1061830d6f2e9539496c46f2ac").unwrap();
            let exit_path_expected = hex::decode(
                "02e010b27520b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421ac",
            )
            .unwrap();

            assert_eq!(collab_path, collab_path_expected);
            assert_eq!(exit_path, exit_path_expected);
        }
    }

    #[test]
    fn test_connector() {
        let self_key: XOnlyPublicKey =
            "b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421"
                .parse()
                .unwrap();

        let lift_txo = Connector::new(self_key);

        println!("spkis {}", hex::encode(lift_txo.spk().unwrap()));

        let tree = lift_txo.taproot().tree();

        if let Some(tree) = tree {
            let connector = tree.leaves()[0].tap_script();

            let connector_expected = hex::decode("20b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421ad20fe44f87e8dcf65392e213f304bee1e3a31e562bc1061830d6f2e9539496c46f2ac").unwrap();

            assert_eq!(connector, connector_expected);
        }
    }

    #[test]
    fn test_projector() {
        let public_key_1: XOnlyPublicKey =
            "9dde15a45d76d940f90188537d52136ba5e86c8fb2f521f53be794410352798f"
                .parse()
                .unwrap();
        let public_key_2: XOnlyPublicKey =
            "cf77e4bb66c0a1ce2cd04cd2838ea5d4210e1474fabe717c47237a1da77b81bc"
                .parse()
                .unwrap();
        let public_key_3: XOnlyPublicKey =
            "ea3d7da21468ef105ad5f3fef1710dd2c759f0014563fd9df922ec7456a9f811"
                .parse()
                .unwrap();

        let mut pubkeys: [PublicKey; 3] = [
            public_key_1.public_key(Parity::Even),
            public_key_2.public_key(Parity::Even),
            public_key_3.public_key(Parity::Even),
        ];
        pubkeys.sort();

        let key_agg_ctx: KeyAggContext = KeyAggContext::new(pubkeys).unwrap();
        let agg_key_expected: XOnlyPublicKey = key_agg_ctx.aggregated_pubkey();
        // 8c12ef9e2507f9c7898ccf47f9059c70c4005f8b9c738597fd015cefe23ed701

        let pubkeys = vec![public_key_1, public_key_2, public_key_3];

        let projector = Projector::new(pubkeys, ProjectorTag::VTXOProjector);

        assert_eq!(projector.msg_senders_aggregate_key(), agg_key_expected);

        let reveal_path = projector.taproot().tree().unwrap().leaves()[0].tap_script();
        let expected_reveal_path = hex::decode("208c12ef9e2507f9c7898ccf47f9059c70c4005f8b9c738597fd015cefe23ed701ad20fe44f87e8dcf65392e213f304bee1e3a31e562bc1061830d6f2e9539496c46f2ac").unwrap();

        assert_eq!(reveal_path, expected_reveal_path);

        let expected_reclaim_path = hex::decode(
            "02a032b27520fe44f87e8dcf65392e213f304bee1e3a31e562bc1061830d6f2e9539496c46f2ac",
        )
        .unwrap();
        let reclaim_path = projector.taproot().tree().unwrap().leaves()[1].tap_script();

        assert_eq!(reclaim_path, expected_reclaim_path);

        let expected_spk =
            hex::decode("5120e83ee0684831fdf523c7ae8e6448ad32e4bb2b986881414a8494341159ac0e1f")
                .unwrap();
        let spk = projector.spk().unwrap();

        assert_eq!(expected_spk, spk);
    }

    #[test]
    fn test_csv_days() {
        // 1
        let days = 1;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("90000000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("029000b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 2
        let days = 2;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("20010000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("022001b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 3
        let days = 3;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("b0010000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02b001b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);
 
        // 4
        let days = 4;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("40020000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("024002b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 5
        let days = 5;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("d0020000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02d002b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 10
        let days = 10;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("a0050000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02a005b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 55
        let days = 55;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("f01e0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02f01eb275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 100
        let days = 100;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("40380000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("024038b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 200
        let days = 200;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("80700000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("028070b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 220
        let days = 220;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("c07b0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("02c07bb275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 230
        let days = 230;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("60810000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03608100b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 240
        let days = 240;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("00870000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03008700b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 254
        let days = 254;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("e08e0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03e08e00b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);

        // 254
        let days = 255;
        let n_sequence = to_n_sequence_encode(CSVFlag::Days(days));
        let n_sequence_expected = hex::decode("708f0000").unwrap();

        let csv_script = to_csv_script_encode(CSVFlag::Days(days));
        let csv_script_expected = hex::decode("03708f00b275").unwrap();

        assert_eq!(n_sequence, n_sequence_expected);
        assert_eq!(csv_script, csv_script_expected);
    }
}

fn main() {}
