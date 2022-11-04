

pub enum ModelVersion {
    V15,
    V21
}

pub enum  ModelKind {
    VAE,
    UNet,
    Clip {
        version: ModelVersion,
    }
}