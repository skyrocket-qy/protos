// Library entrypoint for the protos crate.
// Exposes the buf-generated data types and connectrpc services.

pub mod gen_rust {
    pub mod authzpb {
        include!("../gen_rust/authzpb.mod.rs");

        pub mod rbacpb {
            include!("../gen_rust/authzpb.rbacpb.mod.rs");
        }
    }

    pub mod bastionpb {
        include!("../gen_rust/bastionpb.mod.rs");
    }

    pub mod gatewaypb {
        include!("../gen_rust/gatewaypb.mod.rs");
    }

    pub mod pkgpb {
        include!("../gen_rust/pkgpb.mod.rs");
    }

    pub mod slotpb {
        include!("../gen_rust/slotpb.mod.rs");
    }

    pub mod walletpb {
        include!("../gen_rust/walletpb.mod.rs");
    }

    pub mod fishpb {
        include!("../gen_rust/fishpb.mod.rs");
    }
}

pub mod gen_rust_connect {
    pub mod authzpb {
        include!("../gen_rust_connect/authzpb.mod.rs");

        pub mod rbacpb {
            include!("../gen_rust_connect/authzpb.rbacpb.mod.rs");
        }
    }

    pub mod gatewaypb {
        include!("../gen_rust_connect/gatewaypb.mod.rs");
    }

    pub mod slotpb {
        include!("../gen_rust_connect/slotpb.mod.rs");
    }

    pub mod fishpb {
        include!("../gen_rust_connect/fishpb.mod.rs");
    }
}
