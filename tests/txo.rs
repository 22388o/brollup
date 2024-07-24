#[cfg(test)]
mod txo_tests {
    use bitcoin_vm::txo::{
        connector::Connector,
        lift::Lift,
        projector::{Projector, ProjectorTag},
    };
    use musig2::{
        secp256k1::{Parity, PublicKey, XOnlyPublicKey},
        KeyAggContext,
    };

    #[test]
    fn test_lift() {
        let self_key: XOnlyPublicKey =
            "b2d9fb51db445564f1d4e754f644597b11ff191d12c2a582fb598e509cd72421"
                .parse()
                .unwrap();

        let lift_txo = Lift::new(self_key);

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

        let lift_txo = Connector::new_virtual(self_key);

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
}
