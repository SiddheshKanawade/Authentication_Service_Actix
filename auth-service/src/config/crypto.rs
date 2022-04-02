use argonautica::Hasher;
use std::sync::Arc;
use color_eyre::Result;
use futures::compat::Future01CompatExt;
use eyre::eyre;
use tracing::instrument;



#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key: Arc<String>
}

// under IMPL we write the various functions to be implemented

impl CryptoService {
    // need to implement instrument macro
    #[instrument(self, password)]
    pub async fn hash_password(&self, password: String) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Hashing error: {:?}", err))
    }
}