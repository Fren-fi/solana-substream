// @generated
// @@protoc_insertion_point(attribute:pumpswap)
pub mod pumpswap {
    include!("pumpswap.rs");
    // @@protoc_insertion_point(pumpswap)
}
pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
                // @@protoc_insertion_point(sf.solana.type.v1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod solana {
            // @@protoc_insertion_point(attribute:sf.substreams.solana.v1)
            pub mod v1 {
                include!("sf.substreams.solana.v1.rs");
                // @@protoc_insertion_point(sf.substreams.solana.v1)
            }
        }
    }
}
