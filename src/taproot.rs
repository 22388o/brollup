#![allow(dead_code)]

use lazy_static::lazy_static;
use musig2::secp256k1::{Parity, PublicKey, Scalar, Secp256k1, XOnlyPublicKey};
use sha2::Digest as _;
use sha2::Sha256;
use std::cmp::Ordering;
use std::vec;

const LEAF_VERSION: u8 = 0xc0;

lazy_static! {
    static ref POINT_WITH_UNKNOWN_DISCRETE_LOGARITHM: Vec<u8> = vec![
        0x50, 0x92, 0x9b, 0x74, 0xc1, 0xa0, 0x49, 0x54, 0xb7, 0x8b, 0x4b, 0x60, 0x35, 0xe9, 0x7a,
        0x5e, 0x07, 0x8a, 0x5a, 0x0f, 0x28, 0xec, 0x96, 0xd5, 0x47, 0xbf, 0xee, 0x9a, 0xce, 0x80,
        0x3a, 0xc0
    ];
}

pub enum HashTag {
    TapLeafTag,
    TapBranchTag,
    TapTweakTag,
    CustomTag(String),
}

#[derive(Clone)]
pub enum Branch {
    Leaf(TapLeaf),
    Branch(Box<TapBranch>),
}

#[derive(Clone)]
pub struct TapLeaf {
    leaf_version: u8,
    tap_script: Vec<u8>,
}

impl TapLeaf {
    pub fn new(tap_script: Vec<u8>) -> TapLeaf {
        TapLeaf {
            leaf_version: LEAF_VERSION,
            tap_script,
        }
    }

    pub fn new_version(tap_script: Vec<u8>, leaf_version: u8) -> TapLeaf {
        TapLeaf {
            leaf_version,
            tap_script,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        hash_tap_leaf(&self.tap_script, self.leaf_version)
    }

    pub fn hash_as_vec(&self) -> Vec<u8> {
        self.hash().to_vec()
    }

    pub fn into_branch(&self) -> Branch {
        Branch::Leaf(self.clone())
    }
}

#[derive(Clone)]
pub struct TapBranch {
    left_branch: Branch,
    right_branch: Branch,
}

impl TapBranch {
    pub fn new(first: Branch, second: Branch) -> TapBranch {
        let first_branch_vec: Vec<u8> = match &first {
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
            Branch::Branch(branch) => branch.hash_as_vec(),
        };

        let second_branch_vec: Vec<u8> = match &second {
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
            Branch::Branch(branch) => branch.hash_as_vec(),
        };

        match &first_branch_vec.cmp(&second_branch_vec) {
            Ordering::Less => TapBranch {
                left_branch: first,
                right_branch: second,
            },
            _ => TapBranch {
                left_branch: second,
                right_branch: first,
            },
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let left_branch_vec: Vec<u8> = match &self.left_branch {
            Branch::Branch(branch) => branch.hash_as_vec(),
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
        };

        let right_branch_vec: Vec<u8> = match &self.right_branch {
            Branch::Branch(branch) => branch.hash_as_vec(),
            Branch::Leaf(leaf) => leaf.hash_as_vec(),
        };

        hash_tap_branch(&left_branch_vec, &right_branch_vec)
    }

    pub fn hash_as_vec(&self) -> Vec<u8> {
        self.hash().to_vec()
    }

    pub fn into_branch(&self) -> Branch {
        Branch::Branch(Box::new(self.clone()))
    }
}

pub struct TapRoot {
    inner_key: XOnlyPublicKey,
    uppermost_branch: Option<Branch>,
}

impl TapRoot {
    pub fn key_and_script_path(key: PublicKey, branch: Branch) -> TapRoot {
        TapRoot {
            inner_key: key.x_only_public_key().0,
            uppermost_branch: Some(branch),
        }
    }

    pub fn key_path_only(key: PublicKey) -> TapRoot {
        TapRoot {
            inner_key: key.x_only_public_key().0,
            uppermost_branch: None,
        }
    }

    pub fn script_path_only(branch: Branch) -> TapRoot {
        TapRoot {
            inner_key: XOnlyPublicKey::from_slice(&POINT_WITH_UNKNOWN_DISCRETE_LOGARITHM).unwrap(),
            uppermost_branch: Some(branch),
        }
    }

    pub fn lift_x(&self) -> PublicKey {
        self.inner_key.public_key(Parity::Even)
    }

    pub fn tap_tweak(&self) -> [u8; 32] {
        let inner_vec: Vec<u8> = self.inner_key.serialize().to_vec();
        let mut tweak_vec: Vec<u8> = vec![];

        if let Some(branch) = &self.uppermost_branch {
            match branch {
                Branch::Leaf(leaf) => tweak_vec.extend(leaf.hash_as_vec()),
                Branch::Branch(branch) => tweak_vec.extend(branch.hash_as_vec()),
            };
        }

        hash_tap_tweak(&inner_vec, &tweak_vec)
    }

    pub fn tweaked_key(&self) -> PublicKey {
        if let Some(_) = &self.uppermost_branch {
            let scalar: Scalar = Scalar::from_be_bytes(self.tap_tweak()).unwrap();

            self.lift_x()
                .add_exp_tweak(&Secp256k1::new(), &scalar)
                .unwrap()
        } else {
            self.lift_x()
        }
    }

    pub fn tweaked_key_parity(&self) -> Parity {
        let (_, parity) = self.tweaked_key().x_only_public_key();
        parity
    }
    pub fn tweaked_key_x_only(&self) -> XOnlyPublicKey {
        let (x_only, _) = self.tweaked_key().x_only_public_key();
        x_only
    }

    pub fn spk(&self) -> Vec<u8> {
        let mut spk: Vec<u8> = vec![0x51, 0x20];
        spk.extend(
            self.tweaked_key()
                .x_only_public_key()
                .0
                .serialize()
                .to_vec(),
        );
        spk
    }
}

pub fn tagged_hash(data: impl AsRef<[u8]>, tag: HashTag) -> [u8; 32] {
    let tag_digest = match tag {
        HashTag::TapLeafTag => Sha256::digest("TapLeaf"),
        HashTag::TapBranchTag => Sha256::digest("TapBranch"),
        HashTag::TapTweakTag => Sha256::digest("TapTweak"),
        HashTag::CustomTag(tag) => Sha256::digest(tag),
    };

    let hash: [u8; 32] = {
        Sha256::new()
            .chain_update(&tag_digest)
            .chain_update(&tag_digest)
            .chain_update(&data)
            .finalize()
            .into()
    };

    hash
}

pub fn hash_tap_leaf(raw_script_vec: &Vec<u8>, version: u8) -> [u8; 32] {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(&[version]);
    data.extend_from_slice(&[(&raw_script_vec).len() as u8]);
    data.extend_from_slice(raw_script_vec);

    tagged_hash(&data, HashTag::TapLeafTag)
}

pub fn hash_tap_branch(left_branch_vec: &Vec<u8>, right_branch_vec: &Vec<u8>) -> [u8; 32] {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(left_branch_vec);
    data.extend_from_slice(right_branch_vec);

    tagged_hash(&data, HashTag::TapBranchTag)
}

pub fn hash_tap_tweak(inner_key_vec: &Vec<u8>, tweak_vec: &Vec<u8>) -> [u8; 32] {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(inner_key_vec);
    data.extend_from_slice(tweak_vec);

    tagged_hash(&data, HashTag::TapTweakTag)
}
