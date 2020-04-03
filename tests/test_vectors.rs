use cen::*;

// Run cargo test generate_test_vectors -- --nocapture
#[test]
fn generate_test_vectors() {
    use std::io::Cursor;

    let rak = ReportAuthorizationKey::new(rand::thread_rng());

    let mut buf = Vec::new();
    rak.write(Cursor::new(&mut buf))
        .expect("writing should succeed");
    println!("rak:\n{}", hex::encode(&buf));

    let mut cek = rak.initial_contact_event_key();

    for i in 0..10 {
        let mut buf = Vec::new();
        cek.write(Cursor::new(&mut buf))
            .expect("writing should succeed");
        println!("cek_{}:\n{}", i, hex::encode(&buf));

        let cen = cek.contact_event_number();
        println!("cen_{}:\n{}", i, hex::encode(&cen.0));

        cek = cek.ratchet().unwrap();
    }

    let signed_report = rak
        .create_report(
            MemoType::CoEpiV1,        // The memo type
            b"symptom data".to_vec(), // The memo data
            2,                        // Index of the first CEN to disclose
            10,                       // Index of the last CEN to check
        )
        .expect("Report creation can only fail if the memo data is too long");

    let mut buf = Vec::new();
    signed_report
        .write(Cursor::new(&mut buf))
        .expect("writing should succeed");
    println!("signed_report:\n{}", hex::encode(&buf));
}
