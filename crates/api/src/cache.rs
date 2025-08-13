use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::NaiveDate;

use crate::server::TempoStatus;

pub type TempoCache = Arc<RwLock<HashMap<String, TempoStatus>>>;

pub fn create_cache() -> TempoCache {
    Arc::new(RwLock::new(HashMap::new()))
}

pub async fn get_from_cache(cache: &TempoCache, date: NaiveDate) -> Option<TempoStatus> {
    let date_str = date.format("%Y-%m-%d").to_string();
    let cache_read = cache.read().await;
    let result = cache_read.get(&date_str).copied();

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
        match status {
            TempoStatus::NonDefini => {
                tracing::info!("Statut NON_DEFINI pour {}, pas de mise en cache", date);
            }
            _ => {
                tracing::info!("Ajout au cache: {} -> {:?}", date, status);
                cache_write.insert(date, status);
            }
        }
    }
}
