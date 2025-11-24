
#[cfg(any(feature = "ML_DSA_44", feature = "ML_DSA_65", feature = "ML_DSA_87"))]
#[cfg(feature = "HEDGED")]
#[cfg(test)]
mod hedged_signature_with_ctx_kat {
    #[cfg(feature = "ML_DSA_44")]
    #[test]
    fn test_mldsa_44() {
        let kat_list = crate::sign_with_ctx_kat::deterministic_signature_with_ctx_kat::read_mldsa_nist_acvp_kat("./kats/ml_dsa_44.kat").unwrap();
        assert!(!kat_list.is_empty());
        assert_eq!(kat_list.len(), 100);
        crate::sign_with_ctx_kat::deterministic_signature_with_ctx_kat::run_kat_with_ctx(kat_list);
    }

    #[cfg(feature = "ML_DSA_65")]
    #[test]
    fn test_mldsa_44() {
        let kat_list = crate::sign_with_ctx_kat::deterministic_signature_with_ctx_kat::read_mldsa_nist_acvp_kat("./kats/ml_dsa_65.kat").unwrap();
        assert!(!kat_list.is_empty());
        assert_eq!(kat_list.len(), 100);
        crate::sign_with_ctx_kat::deterministic_signature_with_ctx_kat::run_kat_with_ctx(kat_list);
    }

    #[cfg(feature = "ML_DSA_87")]
    #[test]
    fn test_mldsa_44() {
        let kat_list = crate::sign_with_ctx_kat::deterministic_signature_with_ctx_kat::read_mldsa_nist_acvp_kat("./kats/ml_dsa_87.kat").unwrap();
        assert!(!kat_list.is_empty());
        assert_eq!(kat_list.len(), 100);
        crate::sign_with_ctx_kat::deterministic_signature_with_ctx_kat::run_kat_with_ctx(kat_list);
    }
}
