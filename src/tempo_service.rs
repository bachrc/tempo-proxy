use crate::cache::{get_from_cache, insert_multiple_into_cache, TempoCache};
use crate::edf_api;
use crate::server::TempoStatus;

pub async fn get_tempo_status_for_date(cache: &TempoCache, date: &str) -> Result<TempoStatus, String> {
    // Vérifier d'abord le cache
    if let Some(cached_status) = get_from_cache(cache, date).await {
        return Ok(cached_status);
    }

    tracing::info!("Appel API EDF nécessaire pour la date: {}", date);
    
    // Calculer la date de demain pour l'API EDF
    let date_parsed = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| "Format de date invalide".to_string())?;
    let tomorrow = date_parsed + chrono::Duration::days(1);
    let tomorrow_str = tomorrow.format("%Y-%m-%d").to_string();

    // Appeler l'API EDF pour récupérer toutes les dates
    match edf_api::fetch_tempo_calendar(date, &tomorrow_str).await {
        Ok(calendar_entries) => {
            // Insérer toutes les entrées dans le cache
            insert_multiple_into_cache(cache, calendar_entries.clone()).await;
            
            // Retourner le statut pour la date demandée
            calendar_entries
                .into_iter()
                .find(|(d, _)| d == date)
                .map(|(_, status)| status)
                .ok_or_else(|| "Date non trouvée dans la réponse API".to_string())
        }
        Err(error) => Err(error),
    }
}