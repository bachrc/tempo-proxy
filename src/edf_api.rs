use serde::Deserialize;

use crate::server::TempoStatus;

#[derive(Debug, Deserialize)]
struct EdfApiResponse {
    content: EdfApiContent,
}

#[derive(Debug, Deserialize)]
struct EdfApiContent {
    options: Vec<EdfApiOption>,
}

#[derive(Debug, Deserialize)]
struct EdfApiOption {
    option: String,
    calendrier: Vec<CalendarDay>,
}

#[derive(Debug, Deserialize)]
struct CalendarDay {
    #[serde(rename = "dateApplication")]
    date_application: String,
    statut: String,
}

pub async fn fetch_tempo_calendar(today: &str, tomorrow: &str) -> Result<Vec<(String, TempoStatus)>, String> {
    let url = format!(
        "https://api-commerce.edf.fr/commerce/activet/v1/calendrier-jours-effacement?option=TEMPO&dateApplicationBorneInf={}&dateApplicationBorneSup={}&identifiantConsommateur=src",
        today, tomorrow
    );

    let client = reqwest::Client::new();

    match client.get(&url).send().await {
        Ok(response) => {
            let response_text = response.text().await.unwrap_or_default();
            tracing::info!("Réponse API EDF: {}", response_text);

            match serde_json::from_str::<EdfApiResponse>(&response_text) {
                Ok(api_response) => {
                    let calendar_entries: Vec<(String, TempoStatus)> = api_response
                        .content
                        .options
                        .iter()
                        .find(|opt| opt.option == "TEMPO")
                        .map(|opt| {
                            opt.calendrier
                                .iter()
                                .map(|day| {
                                    let status = match day.statut.as_str() {
                                        "TEMPO_BLEU" => TempoStatus::TempoBleu,
                                        "TEMPO_ROUGE" => TempoStatus::TempoRouge,
                                        "TEMPO_BLANC" => TempoStatus::TempoBlanc,
                                        _ => TempoStatus::TempoBleu,
                                    };
                                    (day.date_application.clone(), status)
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    Ok(calendar_entries)
                }
                Err(_) => Err("Erreur lors du parsing de la réponse EDF".to_string()),
            }
        }
        Err(_) => Err("Erreur lors de l'appel à l'API EDF".to_string()),
    }
}
