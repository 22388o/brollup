mod taproot;

use hex;
use musig2::secp256k1::PublicKey;
use musig2::KeyAggContext;
use taproot::*;

#[cfg(test)]
mod tests {
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
    fn test_tap_root() {
        let tap_leaf: TapLeaf = TapLeaf::new(vec![0xaa, 0xbb, 0xcc]);

        // Test - with even inner key

        let inner_key_even: PublicKey =
            "028c17db0c798574086299e5041ffbcfa06bd501eb0e50914731bfbd2f3c9f980e"
                .parse()
                .unwrap();

        let taproot = TapRoot::new(inner_key_even, tap_leaf.into_branch());

        let expected_with_odd: Vec<u8> =
            hex::decode("51202e1a63521f2d72ff54da28cf8e114c6e3ce3ef497e9a6ac71b3e28e06446a218")
                .unwrap();

        assert_eq!(taproot.spk(), expected_with_odd);

        // Test - with odd inner key

        let inner_key_odd: PublicKey =
            "037b55a1c853b28c398141c8fdf4eb69469430f019983af4be4b5aa7512936f295"
                .parse()
                .unwrap();

        let taproot = TapRoot::new(inner_key_odd, tap_leaf.into_branch());

        let expected_with_even: Vec<u8> =
            hex::decode("51208cda55510b8f99ec248ed9772e6a71537eb26142d6624d38426a7a1311b488e6")
                .unwrap();

        assert_eq!(taproot.spk(), expected_with_even);
    }
}

fn main() {}
