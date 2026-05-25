#[path = "../gen_rust"]
pub mod gen_rust {
    #[path = "."]
    pub mod authzpb {
        #[path = "authzpb.rbacpb.mod.rs"]
        pub mod rbacpb;

        #[path = "authzpb.v1.mod.rs"]
        pub mod v1;
    }

    #[path = "."]
    pub mod gatewaypb {
        #[path = "gatewaypb.v1.mod.rs"]
        pub mod v1;
    }

    #[path = "."]
    pub mod pkgpb {
        #[path = "pkgpb.v1.mod.rs"]
        pub mod v1;
    }

    #[path = "."]
    pub mod slotpb {
        #[path = "slotpb.extradatapb.mod.rs"]
        pub mod extradatapb;

        #[path = "slotpb.v1.mod.rs"]
        pub mod v1;
    }
}

#[path = "../gen_rust_connect"]
pub mod r#gen {
    #[path = "."]
    pub mod authzpb {
        #[path = "authzpb.rbacpb.mod.rs"]
        pub mod rbacpb;

        #[path = "authzpb.v1.mod.rs"]
        pub mod v1;
    }

    #[path = "."]
    pub mod gatewaypb {
        #[path = "gatewaypb.v1.mod.rs"]
        pub mod v1;
    }

    #[path = "."]
    pub mod slotpb {
        #[path = "slotpb.v1.mod.rs"]
        pub mod v1;
    }
}
