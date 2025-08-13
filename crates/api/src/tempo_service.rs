use crate::cache::{get_from_cache, insert_multiple_into_cache, TempoCache};
use crate::edf_api;
use crate::server::TempoStatus;
use chrono::{Local, NaiveDate, Timelike};

/// Vérifie si on peut demander le statut de demain (après 11h heure française)
fn can_request_tomorrow_status() -> bool {
    let now = Local::now();
    now.hour() >= 11
}

/// Vérifie si la date demandée est demain
fn is_tomorrow_date(date: NaiveDate) -> bool {
    let today = Local::now().date_naive();
    let tomorrow = today + chrono::Duration::days(1);
    date == tomorrow
}

pub async fn get_tempo_status_for_date(cache: &TempoCache, date: NaiveDate) -> Result<TempoStatus, String> {
    // Vérifier d'abord le cache
    if let Some(cached_status) = get_from_cache(cache, date).await {
        return Ok(cached_status);
    }

    // Vérifier la règle des 11h pour le statut de demain
    if is_tomorrow_date(date) && !can_request_tomorrow_status() {
        tracing::info!("Demande du statut de demain avant 11h, retour NON_DEFINI pour la date: {}", date);
        return Ok(TempoStatus::NonDefini);
    }

    tracing::info!("Appel API EDF nécessaire pour la date: {}", date);

    // Appeler l'API EDF pour récupérer toutes les dates
    match edf_api::fetch_tempo_calendar(date, date).await {
        Ok(calendar_entries) => {
            // Insérer toutes les entrées dans le cache
            insert_multiple_into_cache(cache, calendar_entries.clone()).await;
            
            // Retourner le statut pour la date demandée
            let date_str = date.format("%Y-%m-%d").to_string();
            calendar_entries
                .into_iter()
                .find(|(d, _)| *d == date_str)
                .map(|(_, status)| status)
                .ok_or_else(|| "Date non trouvée dans la réponse API".to_string())
        }
        Err(error) => Err(error),
    }
}

async fn fetch_and_cache_date_range(cache: &TempoCache, start_date: NaiveDate, end_date: NaiveDate) -> Result<Vec<(String, TempoStatus)>, String> {
    match edf_api::fetch_tempo_calendar(start_date, end_date).await {
        Ok(calendar_entries) => {
            insert_multiple_into_cache(cache, calendar_entries.clone()).await;
            Ok(calendar_entries)
        }
        Err(error) => Err(error),
    }
}

pub async fn get_tempo_status_for_today_and_tomorrow(cache: &TempoCache) -> Result<(TempoStatus, TempoStatus), String> {
    let today_date = Local::now().date_naive();
    let tomorrow_date = today_date + chrono::Duration::days(1);

    // Vérifier le cache pour les deux dates
    let today_cached = get_from_cache(cache, today_date).await;
    let tomorrow_cached = get_from_cache(cache, tomorrow_date).await;
    let can_request_tomorrow = can_request_tomorrow_status();

    match (today_cached, tomorrow_cached, can_request_tomorrow) {
        // Tout en cache
        (Some(today_status), Some(tomorrow_status), _) => {
            Ok((today_status, tomorrow_status))
        }
        // Avant 11h, aujourd'hui en cache
        (Some(today_status), _, false) => {
            let now = Local::now();
            tracing::info!("Demande du statut de demain avant 11h (il est {}h{}), retour NON_DEFINI", now.hour(), now.minute());
            Ok((today_status, TempoStatus::NonDefini))
        }
        // Après 11h, une ou deux dates manquantes
        (today_opt, tomorrow_opt, true) => {
            // Un seul appel API pour aujourd'hui et demain
            tracing::info!("Appel API EDF pour aujourd'hui et demain: {} à {}", today_date, tomorrow_date);
            let calendar_entries = fetch_and_cache_date_range(cache, today_date, tomorrow_date).await?;
            
            let today_str = today_date.format("%Y-%m-%d").to_string();
            let tomorrow_str = tomorrow_date.format("%Y-%m-%d").to_string();
            
            let today_status = today_opt.or_else(|| {
                calendar_entries.iter().find(|(d, _)| d == &today_str).map(|(_, s)| *s)
            });
            
            let tomorrow_status = tomorrow_opt.or_else(|| {
                calendar_entries.iter().find(|(d, _)| d == &tomorrow_str).map(|(_, s)| *s)
            });
            
            match (today_status, tomorrow_status) {
                (Some(today_status), Some(tomorrow_status)) => Ok((today_status, tomorrow_status)),
                (Some(today_status), None) => Ok((today_status, TempoStatus::NonDefini)),
                _ => Err("Date d'aujourd'hui non trouvée dans la réponse API".to_string()),
            }
        }
        // Avant 11h, aujourd'hui manquant
        (None, _, false) => {
            let now = Local::now();
            tracing::info!("Avant 11h (il est {}h{}), appel API EDF pour aujourd'hui seulement: {}", now.hour(), now.minute(), today_date);
            let calendar_entries = fetch_and_cache_date_range(cache, today_date, today_date).await?;
            
            let today_str = today_date.format("%Y-%m-%d").to_string();
            let today_status = calendar_entries.iter().find(|(d, _)| d == &today_str).map(|(_, s)| *s);
            
            match today_status {
                Some(today_status) => Ok((today_status, TempoStatus::NonDefini)),
                None => Err("Date d'aujourd'hui non trouvée dans la réponse API".to_string()),
            }
        }
    }
}