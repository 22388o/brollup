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

        let agg_pubkey: PublicKey = (&key_agg_ctx).aggregated_pubkey();

        let agg_pubkey_without_tweak: PublicKey = (&key_agg_ctx).aggregated_pubkey_untweaked();

        println!(
            "aggregate pubkey without tweak: {} ",
            agg_pubkey_without_tweak.to_string()
        );
        println!("aggregate pubkey with tweak: {} ", agg_pubkey.to_string());
    }

    #[test]
    fn test_tap_branch() {
        let tap_leaf_1: TapLeaf = TapLeaf::new(vec![0xde, 0xad]);
        let tap_leaf_2: TapLeaf = TapLeaf::new(vec![0xbe, 0xef]);

        let branch_1: Branch = Branch::Leaf(tap_leaf_1.clone());
        let branch_2: Branch = Branch::Leaf(tap_leaf_2.clone());

        let tap_branch: TapBranch = TapBranch::new(branch_1.clone(), branch_2.clone());
        let tap_branch_reversed: TapBranch = TapBranch::new(branch_2, branch_1);

        let expected: Vec<u8> =
            hex::decode("b220872a5f6915e7779e659c2925b4b6cef6c1792f2e7bed0ba6331631fa7c63")
                .unwrap();

        assert_eq!(tap_branch.hash_as_vec(), expected);
        assert_eq!(tap_branch_reversed.hash_as_vec(), expected);
    }
}

fn main() {}
