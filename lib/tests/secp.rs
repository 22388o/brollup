#[cfg(test)]
mod secp_tests {
    use brollup::{
        serialization::conversion::IntoByteArray,
        signature::{
            into::{IntoPoint, IntoScalar},
            schnorr::{schnorr_sign, schnorr_verify_compressed, SecpError, SignFlag},
            sum::{sum_points, sum_scalars},
        },
    };

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

        schnorr_verify_compressed(
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
                .map_err(|_| SecpError::InvalidScalar)?;

        let scalar_1 = scalar_1_bytes.into_scalar()?;

        let scalar_2_bytes =
            hex::decode("fa22dfe1da9013b3c1145040acae9089e0c08bc1c1a0719614f4b73add6f6ef5")
                .map_err(|_| SecpError::InvalidScalar)?;

        let scalar_2 = scalar_2_bytes.into_scalar()?;

        let scalars = vec![scalar_1, scalar_2];

        let sum = sum_scalars(scalars)?;

        let expected_sum_bytes =
            hex::decode("d1bbb1dca14d5f658233a071d40ec59b394948e5c9487a1b04aca919a3eb800e")
                .map_err(|_| SecpError::InvalidScalar)?;
        let expected_sum = expected_sum_bytes.into_scalar()?;

        assert_eq!(sum, expected_sum);

        Ok(())
    }

    #[test]
    fn test_sum_points() -> Result<(), SecpError> {
        let point_1_bytes =
            hex::decode("7759eb7a3182a6e5ab4818ab2bbbb79d1aa93b16e0ef1f2b1141614a9c8402a5")
                .map_err(|_| SecpError::InvalidPoint)?;

        let point_1 = point_1_bytes.into_point()?;

        let point_2_bytes =
            hex::decode("2be00f329e405edacf4beaf1f235e1c38df8dc3a280b92573216cb8e98cc5f3c")
                .map_err(|_| SecpError::InvalidPoint)?;

        let point_2 = point_2_bytes.into_point()?;

        let points = vec![point_1, point_2];

        let sum = sum_points(points)?;

        let expected_sum_vec =
            hex::decode("60dadabf8a850d6f4d6ffa8ec4777bdb085e3dbb49fe6122bed3d2c3c7e0e1e3")
                .map_err(|_| SecpError::InvalidPoint)?;
        let expected_sum = expected_sum_vec.into_point()?;

        assert_eq!(sum, expected_sum);

        Ok(())
    }
}
