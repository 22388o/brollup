#[cfg(test)]
mod schnorr_tests {
    use brollup::signature::schnorr::{schnorr_sign, SignError, SignFlag};

    #[test]
    fn test_sign_even_key() -> Result<(), SignError> {
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();
        let private_key =
            hex::decode("09f5dde60c19101b671a5e3f4e6f0c0aaa92814170edf7f6bc19b5a21e358a51")
                .unwrap();
        // corresponding public key: 02dee61ab0f4cb3a993cb13c552e44f5abfbf1b377c08b0380da14de41234ea8bd

        let sig_expected = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72abbb8e6d1591f7a18165722d1aa035e1372532527fadf64ab71839728d8c2c468e").unwrap();

        let sig = schnorr_sign(
            private_key.try_into().unwrap(),
            message.try_into().unwrap(),
            SignFlag::EntrySign,
        )?;

        assert_eq!(sig.to_vec(), sig_expected);

        Ok(())
    }

    #[test]
    fn test_sign_odd_key() {
        // We expect ddd key use to return SignError::InvalidSecretKey.
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();
        let private_key =
            hex::decode("899434a6d726d79efeb552b541fb33cb9d98b043e7010b3cb8fa6da924737711")
                .unwrap();
        // corresponding public key: 032c66b988af840daca29ee924a9f0242f81e6552590f161fa9c10f40f910d08e9

        match schnorr_sign(
            private_key.try_into().unwrap(),
            message.try_into().unwrap(),
            SignFlag::EntrySign,
        ) {
            Result::Ok(_) => {
                panic!("Odd key use must have failed this test.")
            }
            Result::Err(err) => {
                match err {
                    SignError::InvalidSecretKey => {
                        // We expect SignError::InvalidSecretKey
                    }
                    _ => panic!(),
                }
            }
        }
    }
}
