fn main() -> std::io::Result<()> {
    #[cfg(feature = "protoc")]
    {
        let (protoc_bin, _) = protoc_prebuilt::init("22.0").unwrap();
        std::env::set_var("PROTOC", protoc_bin);
    }

    #[cfg(feature = "secret")]
    {
        prost_build::Config::new()
            .enable_type_names()
            .compile_protos(
                &[
                    "proto/secret/compute/v1beta1/msg.proto",
                    "proto/secret/compute/v1beta1/query.proto",
                    "proto/secret/compute/v1beta1/types.proto",
                    "proto/secret/emergencybutton/v1beta1/params.proto",
                    "proto/secret/emergencybutton/v1beta1/query.proto",
                    "proto/secret/emergencybutton/v1beta1/tx.proto",
                    "proto/secret/intertx/v1beta1/query.proto",
                    "proto/secret/intertx/v1beta1/tx.proto",
                    "proto/secret/registration/v1beta1/msg.proto",
                    "proto/secret/registration/v1beta1/query.proto",
                    "proto/secret/registration/v1beta1/types.proto",
                ],
                &["proto/"],
            )
            .unwrap();
    }

    #[cfg(all(feature = "secret", not(feature = "cosmos-sdk")))]
    {
        prost_build::Config::new()
            .enable_type_names()
            .compile_protos(
                &[
                    "proto/tendermint/abci/types.proto",
                    "proto/tendermint/crypto/keys.proto",
                    "proto/tendermint/crypto/proof.proto",
                    "proto/tendermint/p2p/types.proto",
                    "proto/tendermint/types/events.proto",
                    "proto/tendermint/types/types.proto",
                    "proto/tendermint/version/types.proto",
                    "proto/cosmos/auth/v1beta1/auth.proto",
                    "proto/cosmos/auth/v1beta1/query.proto",
                    "proto/cosmos/authz/v1beta1/query.proto",
                    "proto/cosmos/authz/v1beta1/tx.proto",
                    "proto/cosmos/bank/v1beta1/query.proto",
                    "proto/cosmos/bank/v1beta1/tx.proto",
                    "proto/cosmos/base/abci/v1beta1/abci.proto",
                    "proto/cosmos/base/kv/v1beta1/kv.proto",
                    "proto/cosmos/base/node/v1beta1/query.proto",
                    "proto/cosmos/base/query/v1beta1/pagination.proto",
                    // "proto/cosmos/base/reflection/v1beta1/reflection.proto",
                    // "proto/cosmos/base/snapshots/v1beta1/snapshot.proto",
                    // "proto/cosmos/base/store/v1beta1/commit_info.proto",
                    // "proto/cosmos/base/store/v1beta1/listening.proto",
                    "proto/cosmos/base/tendermint/v1beta1/query.proto",
                    "proto/cosmos/base/v1beta1/coin.proto",
                    // "proto/cosmos/crypto/ed25519/keys.proto",
                    "proto/cosmos/crypto/multisig/v1beta1/multisig.proto",
                    "proto/cosmos/crypto/multisig/keys.proto",
                    // "proto/cosmos/crypto/secp256k1/keys.proto",
                    // "proto/cosmos/crypto/secp256r1/keys.proto",
                    "proto/cosmos/gov/v1beta1/gov.proto",
                    "proto/cosmos/gov/v1beta1/query.proto",
                    "proto/cosmos/gov/v1beta1/tx.proto",
                    "proto/cosmos/staking/v1beta1/query.proto",
                    "proto/cosmos/staking/v1beta1/staking.proto",
                    "proto/cosmos/staking/v1beta1/tx.proto",
                    "proto/cosmos/tx/signing/v1beta1/signing.proto",
                    "proto/cosmos/tx/v1beta1/service.proto",
                    "proto/cosmos/tx/v1beta1/tx.proto",
                    "proto/cosmos/upgrade/v1beta1/query.proto",
                    "proto/cosmos/upgrade/v1beta1/upgrade.proto",
                    "proto/secret/compute/v1beta1/msg.proto",
                    "proto/secret/compute/v1beta1/query.proto",
                    "proto/secret/compute/v1beta1/types.proto",
                    "proto/secret/emergencybutton/v1beta1/params.proto",
                    "proto/secret/emergencybutton/v1beta1/query.proto",
                    "proto/secret/emergencybutton/v1beta1/tx.proto",
                    "proto/secret/intertx/v1beta1/query.proto",
                    "proto/secret/intertx/v1beta1/tx.proto",
                    "proto/secret/registration/v1beta1/msg.proto",
                    "proto/secret/registration/v1beta1/query.proto",
                    "proto/secret/registration/v1beta1/types.proto",
                ],
                &["proto/"],
            )
            .unwrap();
    }

    Ok(())
}
