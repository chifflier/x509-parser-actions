#![cfg(feature = "verify")]

use x509_parser::parse_x509_der;

static CA_DER: &[u8] = include_bytes!("../assets/IGC_A.der");
static CA_LETSENCRYPT_X3: &[u8] = include_bytes!("../assets/lets-encrypt-x3-cross-signed.der");
static CERT_DER: &[u8] = include_bytes!("../assets/certificate.der");

#[test]
fn test_signature_verification() {
    // for a root CA, verify self-signature
    let (_, x509_ca) = parse_x509_der(CA_DER).expect("could not parse certificate");
    let res = x509_ca.verify(None);
    eprintln!("Verification: {:?}", res);
    assert!(res.is_ok());

    // for a standard certificate, first load the authority, then the certificate, and verify it
    let (_, x509_ca) = parse_x509_der(CA_LETSENCRYPT_X3).expect("could not parse certificate");
    let (_, x509_cert) = parse_x509_der(CERT_DER).expect("could not parse certificate");
    let res = x509_cert.verify(Some(&x509_ca.tbs_certificate.subject_pki));
    eprintln!("Verification: {:?}", res);
    assert!(res.is_ok());
}
