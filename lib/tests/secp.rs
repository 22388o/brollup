#[cfg(test)]
mod secp_tests {
    use brollup::{
        serialization::conversion::IntoByteArray,
        signature::{
            schnorr::{schnorr_sign, schnorr_verify, SecpError, SignFlag},
            sum::sum_scalars,
        },
    };
    use secp::Scalar;

    #[test]
    fn test_sign_schnorr() -> Result<(), SecpError> {
        let message =
            hex::decode("e97f06fabc231539119048bd3c55d0aa6015ed157532e6a5e6fb15aae331791d")
                .unwrap();
        let private_key =
            hex::decode("09f5dde60c19101b671a5e3f4e6f0c0aaa92814170edf7f6bc19b5a21e358a51")
                .unwrap();
        // corresponding public key: 02dee61ab0f4cb3a993cb13c552e44f5abfbf1b377c08b0380da14de41234ea8bd

        let sig_expected = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72ab5facfd42b58fb4852a09228690349fac690b3cb261ff57f208e38c6c2a387e14").unwrap();

        let sig: [u8; 64] = schnorr_sign(
            private_key
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            message
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
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

        let signature = hex::decode("3cdbcc837e40a3b360f09387fd376e62b3f0c509b45a770adfd71f4006de72ab5facfd42b58fb4852a09228690349fac690b3cb261ff57f208e38c6c2a387e14").unwrap();

        schnorr_verify(
            public_key
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            message
                .into_byte_array_32()
                .map_err(|_| SecpError::SignatureParseError)?,
            signature
                .into_byte_array_64()
                .map_err(|_| SecpError::SignatureParseError)?,
            SignFlag::EntrySign,
        )
    }

    #[test]
    fn test_sum_scalars() -> Result<(), SecpError> {
        let scalar_1_bytes =
            hex::decode("d798d1fac6bd4bb1c11f50312760351013379a0ab6f0a8c0af8a506b96b2525a")
                .unwrap();

        let scalar_1 = Scalar::from_slice(&scalar_1_bytes).unwrap();

        let scalar_2_bytes =
            hex::decode("fa22dfe1da9013b3c1145040acae9089e0c08bc1c1a0719614f4b73add6f6ef5")
                .unwrap();

        let scalar_2 = Scalar::from_slice(&scalar_2_bytes).unwrap();

        let scalars = vec![scalar_1, scalar_2];

        let sum = sum_scalars(scalars)?;

        let expected_sum_vec =
            hex::decode("d1bbb1dca14d5f658233a071d40ec59b394948e5c9487a1b04aca919a3eb800e")
                .unwrap();
        let expected_sum = Scalar::from_slice(&expected_sum_vec).unwrap();

        assert_eq!(sum, expected_sum);

        Ok(())
    }

    #[test]
    fn test_sum_scalars_2() -> Result<(), SecpError> {
        let scalar_1_bytes =
            hex::decode("d798d1fac6bd4bb1c11f50312760351013379a0ab6f0a8c0af8a506b96b2525a")
                .unwrap();

        let scalar_1 = Scalar::from_slice(&scalar_1_bytes).unwrap();

        let scalar_2_bytes =
            hex::decode("fa22dfe1da9013b3c1145040acae9089e0c08bc1c1a0719614f4b73add6f6ef5")
                .unwrap();

        let scalar_2 = Scalar::from_slice(&scalar_2_bytes).unwrap();

        let scalar_3_bytes =
            hex::decode("94455e3ed9f716bea425ef99b51fae47128769a1a0cd04244221e4e14631ab83")
                .unwrap();

        let scalar_3 = Scalar::from_slice(&scalar_3_bytes).unwrap();

        let scalars = vec![scalar_1, scalar_2, scalar_3];

        let sum = sum_scalars(scalars)?;

        let expected_sum_vec =
            hex::decode("6601101b7b4476242659900b892e73e39121d5a0baccde0386fc2f6e19e6ea50")
                .unwrap();
        let expected_sum = Scalar::from_slice(&expected_sum_vec).unwrap();

        assert_eq!(sum, expected_sum);

        Ok(())
    }
}