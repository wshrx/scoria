//! The original Horizon Visuals pack: WESL-generated helpers plus three WGSL passes.
use horizon_shader_api::{Pack, check};
use std::path::Path;

#[test]
fn the_horizon_visuals_pack_compiles() {
    let pack = Pack::read(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/shader-packs/horizon-visuals"
    )))
    .expect("the Horizon Visuals pack reads");
    let report = check(&pack).expect("the Horizon Visuals pack compiles");
    let labels: Vec<_> = report.stages.iter().map(|s| s.label.as_str()).collect();
    assert_eq!(report.stages.len(), 6);
    assert!(labels.contains(&"Bloom extract"));
    assert!(labels.contains(&"Bloom blur"));
    assert!(labels.contains(&"Sun shafts"));
    assert!(labels.contains(&"Filmic composite"));
    assert!(report.stages.iter().all(|s| s.expressions > 0));
}
