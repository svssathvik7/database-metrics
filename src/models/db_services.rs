use crate::databases::mongodb::MongoDB;

pub struct DBServices {
    mongodb: MongoDB,
}

impl DBServices {
    pub async fn init() -> Self {
        Self {
            mongodb: MongoDB::init().await,
        }
    }
}
