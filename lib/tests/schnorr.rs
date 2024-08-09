#[cfg(test)]
mod schnorr_tests {
    use brollup::{serialization::conversion::IntoByteArray, signature::schnorr::{schnorr_sign, schnorr_verify, SecpError, SignFlag}};

    #[test]
    fn test_sign_schnorr() -> Result<(), SecpError> {
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();
        let private_key =
            hex::decode("09f5dde60c19101b671a5e3f4e6f0c0aaa92814170edf7f6bc19b5a21e358a51")
                .unwrap();
        // corresponding public key: 02dee61ab0f4cb3a993cb13c552e44f5abfbf1b377c08b0380da14de41234ea8bd

        let sig_expected = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72abbb8e6d1591f7a18165722d1aa035e1372532527fadf64ab71839728d8c2c468e").unwrap();

        let sig = schnorr_sign(
            private_key.into_byte_array_32().map_err(|_| SecpError::SignatureParseError)?,
            message.into_byte_array_32().map_err(|_| SecpError::SignatureParseError)?,
            SignFlag::EntrySign,
        )?;

        assert_eq!(sig.to_vec(), sig_expected);

        Ok(())
    }

    #[test]
    fn test_verify_schnorr() -> Result<(), SecpError> {
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();

        let public_key =
            hex::decode("dee61ab0f4cb3a993cb13c552e44f5abfbf1b377c08b0380da14de41234ea8bd")
                .unwrap();

        // corresponding secret key: 09f5dde60c19101b671a5e3f4e6f0c0aaa92814170edf7f6bc19b5a21e358a51

        let signature = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72abbb8e6d1591f7a18165722d1aa035e1372532527fadf64ab71839728d8c2c468e").unwrap();

        schnorr_verify(
            public_key.into_byte_array_32().map_err(|_| SecpError::SignatureParseError)?,
            message.into_byte_array_32().map_err(|_| SecpError::SignatureParseError)?,
            signature.into_byte_array_64().map_err(|_| SecpError::SignatureParseError)?,
            SignFlag::EntrySign,
        )
    }
}
