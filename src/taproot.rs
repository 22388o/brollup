#![allow(dead_code)]

use core::num;
use lazy_static::lazy_static;
use musig2::secp256k1::{Parity, PublicKey, Scalar, Secp256k1, XOnlyPublicKey};
use sha2::digest::consts::False;
use sha2::digest::consts::True;
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

    pub fn inner_key_lifted(&self) -> PublicKey {
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

            self.inner_key_lifted()
                .add_exp_tweak(&Secp256k1::new(), &scalar)
                .unwrap()
        } else {
            self.inner_key_lifted()
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

pub struct TapTree {
    leaves: Vec<TapLeaf>,
    uppermost_branch: Branch,
}

impl TapTree {
    pub fn new(leaves: Vec<TapLeaf>) -> TapTree {
        match leaves.len() {
            0 => panic!("TapTree must be initialized with at least one TapLeaf."),
            1 => TapTree {
                leaves: leaves.clone(),
                uppermost_branch: leaves[0].into_branch(),
            },
            _ => {
                // Number of TapTree levels is = log2(number of TapLeaves)
                let num_levels: u8 = (leaves.len() as f64).log2() as u8;

                let mut current_level: Vec<Branch> = Vec::new();
                let mut above_level: Vec<Branch> = Vec::new();

                // For each level of the TapTree
                for level in 0..(num_levels + 1) {
                    // If it is the level zero, initialize current_level  with individual TapLeaves
                    if level == 0 {
                        for i in 0..leaves.len() {
                            current_level.push(leaves[i].clone().into_branch());
                        }
                    }
                    // If it is the level one or above, move above_level items into current_level, and reset above_level
                    else {
                        current_level.clear();
                        current_level.extend(above_level.clone());
                        above_level.clear();
                    }

                    let mut iterator: usize = 0;
                    let iterator_bound: usize = current_level.len();

                    let operations: usize = match iterator_bound {
                        0 => panic!("This should not be the case."),
                        1 => 1,
                        _ => (iterator_bound / 2) + (iterator_bound % 2),
                    };

                    for _ in 0..operations {
                        match (iterator_bound - iterator) {
                            0 => panic!("This should not be the case."),
                            // last
                            1 => {
                                above_level.push(current_level[iterator].clone());
                                iterator += 1;
                            }
                            // two or more left in the current scope
                            _ => {
                                let new_branch: TapBranch = TapBranch::new(
                                    current_level[iterator].clone(),
                                    current_level[iterator + 1].clone(),
                                );
                                above_level.push(new_branch.into_branch());
                                iterator += 2;
                            }
                        }
                    }

                    // At the end of each level, the itertor must have covered all branches of that level
                    assert_eq!(iterator, iterator_bound);
                }

                // At the end, only the uppermost branch must have left
                assert_eq!(above_level.len(), 1);
                let uppermost_branch: Branch = above_level[0].clone();

                TapTree {
                    leaves: leaves.clone(),
                    uppermost_branch,
                }
            }
        }
    }
}

pub struct ControlBlock {
    inner_key: XOnlyPublicKey,
    parity: Parity,
    leaf_version: u8,
    path: Vec<u8>,
}

impl ControlBlock {
    pub fn new(inner_key: XOnlyPublicKey, parity: Parity, path: Vec<u8>) -> ControlBlock {
        ControlBlock {
            inner_key,
            parity,
            leaf_version: LEAF_VERSION,
            path,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::<u8>::new();

        match self.parity {
            Parity::Even => vec.push(self.leaf_version),
            Parity::Odd => vec.push(self.leaf_version + 1),
        };

        vec.extend(self.inner_key.serialize().to_vec());
        vec.extend(self.path.clone());
        vec
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
