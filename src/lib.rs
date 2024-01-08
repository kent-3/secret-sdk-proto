#[cfg(feature = "cosmos-sdk")]
pub use cosmos_sdk_proto::*;

#[cfg(feature = "secret")]
pub mod secret {
    pub mod compute {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/secret.compute.v1beta1.rs"));
        }
    }

    pub mod emergencybutton {
        pub mod v1beta1 {
            include!(concat!(
                env!("OUT_DIR"),
                "/secret.emergencybutton.v1beta1.rs"
            ));
        }
    }

    pub mod intertx {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/secret.intertx.v1beta1.rs"));
        }
    }

    pub mod registration {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/secret.registration.v1beta1.rs"));
        }
    }
}

// Include the types needed to support the secret types.
#[cfg(all(feature = "secret", not(feature = "cosmos-sdk")))]
pub mod tendermint {
    pub mod abci {
        include!(concat!(env!("OUT_DIR"), "/tendermint.abci.rs"));
    }
    pub mod crypto {
        include!(concat!(env!("OUT_DIR"), "/tendermint.crypto.rs"));
    }
    pub mod p2p {
        include!(concat!(env!("OUT_DIR"), "/tendermint.p2p.rs"));
    }
    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/tendermint.types.rs"));
    }
    pub mod version {
        include!(concat!(env!("OUT_DIR"), "/tendermint.version.rs"));
    }
}

// Include only the types needed to support the secret compute types.
#[cfg(all(feature = "secret", not(feature = "cosmos-sdk")))]
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/cosmos.auth.v1beta1.rs"));
        }
    }

    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/cosmos.bank.v1beta1.rs"));
        }
    }

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!(concat!(env!("OUT_DIR"), "/cosmos.base.abci.v1beta1.rs"));
            }
        }

        /// Key-value pairs.
        pub mod kv {
            pub mod v1beta1 {
                include!(concat!(env!("OUT_DIR"), "/cosmos.base.kv.v1beta1.rs"));
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!(concat!(env!("OUT_DIR"), "/cosmos.base.query.v1beta1.rs"));
            }
        }

        /// Coins.
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/cosmos.base.v1beta1.rs"));
        }

        /// Tendermint queries.
        pub mod tendermint {
            pub mod v1beta1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/cosmos.base.tendermint.v1beta1.rs"
                ));
            }
        }
    }

    /// Cryptographic primitives.
    pub mod crypto {
        /// Multi-signature support.
        pub mod multisig {
            include!(concat!(env!("OUT_DIR"), "/cosmos.crypto.multisig.rs"));
            pub mod v1beta1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/cosmos.crypto.multisig.v1beta1.rs"
                ));
            }
        }
        // pub mod ed25519 {
        //     include!(concat!(env!("OUT_DIR"), "/cosmos.crypto.ed25519.rs"));
        // }
        // pub mod secp256k1 {
        //     include!(concat!(env!("OUT_DIR"), "/cosmos.crypto.secp256k1.rs"));
        // }
        // pub mod secp256r1 {
        //     include!(concat!(env!("OUT_DIR"), "/cosmos.crypto.secp256r1.rs"));
        // }
    }

    // /// Messages and services handling token distribution
    // pub mod distribution {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.distribution.v1beta1.rs"));
    //     }
    // }
    //
    // /// Messages and services handling evidence
    // pub mod evidence {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.evidence.v1beta1.rs"));
    //     }
    // }
    //
    // /// Allows accounts to grant fee allowances and to use fees from their accounts.
    // pub mod feegrant {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.feegrant.v1beta1.rs"));
    //     }
    // }
    //
    // /// Messages and services handling gentx's
    // pub mod genutil {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.genutil.v1beta1.rs"));
    //     }
    // }

    /// Messages and services handling governance
    // pub mod gov {
    //     pub mod v1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.gov.v1.rs"));
    //     }
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.gov.v1beta1.rs"));
    //     }
    // }

    // /// Messages and services handling minting
    // pub mod mint {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.mint.v1beta1.rs"));
    //     }
    // }
    //
    // /// Messages and services handling chain parameters
    // pub mod params {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.params.v1beta1.rs"));
    //     }
    // }
    //
    // /// Handling slashing parameters and unjailing
    // pub mod slashing {
    //     pub mod v1beta1 {
    //         include!(concat!(env!("OUT_DIR"), "/cosmos.slashing.v1beta1.rs"));
    //     }
    // }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/cosmos.staking.v1beta1.rs"));
        }
    }

    /// Transactions.
    pub mod tx {
        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!(concat!(env!("OUT_DIR"), "/cosmos.tx.signing.v1beta1.rs"));
            }
        }

        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/cosmos.tx.v1beta1.rs"));
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod v1beta1 {
            include!(concat!(env!("OUT_DIR"), "/cosmos.upgrade.v1beta1.rs"));
        }
    }
}

// include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
