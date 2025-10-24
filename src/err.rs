#[allow(dead_code)]
#[repr(u16)]
#[derive(Clone, Debug)]
pub enum MlDsaError {
    KegGenRandomSeedError,
    NTTPolySampleError,
    BoundedPolySampleError,
}