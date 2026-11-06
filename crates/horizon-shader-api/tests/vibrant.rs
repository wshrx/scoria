//! The `vibrant` pack in `tests/shader-packs` is the smoke fixture for a third-party
//! `.hshader`: it must read as a pack and every stage must compile.
use horizon_shader_api::{Pack, check};
use std::path::Path;

#[test]
fn the_vibrant_test_pack_compiles() {
    let pack = Pack::read(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/shader-packs/vibrant"
    )))
    .expect("the vibrant test pack reads");
    let report = check(&pack).expect("the vibrant test pack compiles");
    assert_eq!(report.stages.len(), 1);
    assert_eq!(report.stages[0].label, "Vibrant grade");
    assert!(report.stages[0].expressions > 0);
}
