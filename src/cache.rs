use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::server::TempoStatus;

pub type TempoCache = Arc<RwLock<HashMap<String, TempoStatus>>>;

pub fn create_cache() -> TempoCache {
    Arc::new(RwLock::new(HashMap::new()))
}

pub async fn get_from_cache(cache: &TempoCache, date: &str) -> Option<TempoStatus> {
    let cache_read = cache.read().await;
    let result = cache_read.get(date).copied();

    if result.is_some() {
        tracing::info!("Cache HIT pour la date: {}", date);
    } else {
        tracing::info!("Cache MISS pour la date: {}", date);
    }

    result
}

pub async fn insert_multiple_into_cache(cache: &TempoCache, entries: Vec<(String, TempoStatus)>) {
    let mut cache_write = cache.write().await;
    for (date, status) in entries {
        tracing::info!("Ajout au cache: {} -> {:?}", date, status);
        cache_write.insert(date, status);
    }
}
