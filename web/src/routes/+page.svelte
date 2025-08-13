<script lang="ts">
    interface CalendarResponse {
        date: string;
        statut: "TEMPO_BLEU" | "TEMPO_BLANC" | "TEMPO_ROUGE" | "NON_DEFINI";
    }

    interface CalendarData {
        today: CalendarResponse;
        tomorrow: CalendarResponse;
    }

    async function fetchCalendarData(): Promise<CalendarData> {
        const res = await fetch("/calendar");
        return await res.json();
    }

    // Promise pour les données du calendrier
    let calendarPromise: Promise<CalendarData> = $state(fetchCalendarData());

    // Fonction pour formater une date ISO en français
    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleDateString("fr-FR", {
            weekday: "long",
            year: "numeric",
            month: "long",
            day: "numeric",
        });
    }

    // Fonction pour obtenir la classe CSS selon le statut Tempo
    function getTempoClass(status: CalendarResponse["statut"]): string {
        switch (status) {
            case "TEMPO_BLEU":
                return "tempo-bleu";
            case "TEMPO_BLANC":
                return "tempo-blanc";
            case "TEMPO_ROUGE":
                return "tempo-rouge";
            case "NON_DEFINI":
                return "tempo-non-defini";
        }
    }

    // Fonction pour obtenir le nom français du statut
    function getTempoName(status: CalendarResponse["statut"]): string {
        switch (status) {
            case "TEMPO_BLEU":
                return "Tempo Bleu";
            case "TEMPO_BLANC":
                return "Tempo Blanc";
            case "TEMPO_ROUGE":
                return "Tempo Rouge";
            case "NON_DEFINI":
                return "Non Défini";
        }
    }
</script>

<svelte:head>
    <title>Tempo Proxy - EDF</title>
    <meta
        name="description"
        content="Interface de consultation des données Tempo d'EDF"
    />
</svelte:head>

<main>
    <header class="hero">
        <div class="hero-content">
            <div class="logo-section">
                <div class="edf-logo">EDF</div>
                <h1>Tempo Proxy</h1>
            </div>
            <p class="tagline">
                Votre interface de consultation des données Tempo
            </p>
        </div>
    </header>

    <section class="date-section">
        <div class="dates-container">
            {#await calendarPromise}
                <div class="loading-card">
                    <p>Chargement des données...</p>
                </div>
            {:then calendarData}
                <div
                    class="date-card {getTempoClass(calendarData.today.statut)}"
                >
                    <div class="date-header">
                        <h2>Aujourd'hui</h2>
                    </div>
                    <div class="date-content">
                        <div class="current-date">
                            {formatDate(calendarData.today.date)}
                        </div>
                        <div class="tempo-badge">
                            {getTempoName(calendarData.today.statut)}
                        </div>
                    </div>
                </div>

                <div
                    class="date-card {getTempoClass(
                        calendarData.tomorrow.statut,
                    )}"
                >
                    <div class="date-header">
                        <h2>Demain</h2>
                    </div>
                    <div class="date-content">
                        <div class="current-date">
                            {formatDate(calendarData.tomorrow.date)}
                        </div>
                        <div class="tempo-badge">
                            {getTempoName(calendarData.tomorrow.statut)}
                        </div>
                        {#if calendarData.tomorrow.statut === "NON_DEFINI"}
                            <div class="tempo-info">
                                Disponible à partir de 11h
                            </div>
                        {/if}
                    </div>
                </div>
            {:catch error}
                <div class="error-card">
                    <p>Erreur lors du chargement: {error.message}</p>
                </div>
            {/await}
        </div>
    </section>

    <section class="info-section">
        <div class="info-grid">
            <div class="info-card tempo-bleu">
                <h3>Tempo Bleu</h3>
                <p>Tarif normal, jours de consommation standard</p>
            </div>
            <div class="info-card tempo-blanc">
                <h3>Tempo Blanc</h3>
                <p>Tarif intermédiaire, périodes de forte demande</p>
            </div>
            <div class="info-card tempo-rouge">
                <h3>Tempo Rouge</h3>
                <p>Tarif élevé, pic de consommation nationale</p>
            </div>
        </div>
    </section>
</main>

<style>
    :global(body) {
        margin: 0;
        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Inter",
            "Roboto", sans-serif;
        background: #fafbfc;
        color: #1c2832;
        min-height: 100vh;
        line-height: 1.6;
    }

    main {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }

    .hero {
        background: linear-gradient(
            135deg,
            #1a365d 0%,
            #2c5282 50%,
            #3182ce 100%
        );
        color: white;
        padding: 5rem 2rem;
        text-align: center;
        position: relative;
        overflow: hidden;
    }

    .hero::before {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: linear-gradient(
            45deg,
            rgba(255, 255, 255, 0.05) 0%,
            transparent 50%
        );
    }

    .hero-content {
        max-width: 1000px;
        margin: 0 auto;
        position: relative;
        z-index: 1;
    }

    .logo-section {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 2rem;
        margin-bottom: 2.5rem;
    }

    .edf-logo {
        background: linear-gradient(135deg, #e53e3e 0%, #dd6b20 100%);
        color: white;
        padding: 1rem 2rem;
        border-radius: 12px;
        font-weight: 700;
        font-size: 1.5rem;
        letter-spacing: 3px;
        box-shadow: 0 8px 32px rgba(229, 62, 62, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.1);
    }

    h1 {
        font-size: 4rem;
        font-weight: 700;
        margin: 0;
        letter-spacing: -2px;
        background: linear-gradient(135deg, #ffffff 0%, #e2e8f0 100%);
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }

    .tagline {
        font-size: 1.25rem;
        opacity: 0.9;
        margin: 1rem 0 0 0;
        font-weight: 400;
        color: #cbd5e0;
    }

    .date-section {
        flex: 1;
        background: linear-gradient(180deg, #fafbfc 0%, #f7fafc 100%);
        padding: 4rem 2rem;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .dates-container {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
        gap: 2.5rem;
        max-width: 1200px;
        width: 100%;
    }

    .date-card {
        background: white;
        border-radius: 20px;
        box-shadow:
            0 4px 20px rgba(0, 0, 0, 0.05),
            0 1px 3px rgba(0, 0, 0, 0.1);
        padding: 3rem 2rem;
        text-align: center;
        border: 1px solid #e2e8f0;
        position: relative;
        overflow: hidden;
    }

    .date-card::before {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 4px;
        background: linear-gradient(90deg, #3182ce 0%, #63b3ed 100%);
    }

    .date-header h2 {
        color: #2d3748;
        font-size: 1.25rem;
        margin: 0 0 0.5rem 0;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 1px;
        opacity: 0.8;
    }

    .current-date {
        font-size: 1.3rem;
        font-weight: 600;
        color: #2d3748;
        text-transform: capitalize;
        line-height: 1.3;
        margin-bottom: 1rem;
    }

    .tempo-info {
        font-size: 0.9rem;
        color: #718096;
        margin-top: 0.5rem;
        font-style: italic;
    }

    .tempo-badge {
        background: rgba(255, 255, 255, 0.95);
        color: #1a202c;
        padding: 0.8rem 1.4rem;
        border-radius: 14px;
        font-size: 1.15rem;
        font-weight: 650;
        display: inline-block;
        border: 1px solid rgba(0, 0, 0, 0.2);
        box-shadow: 0 2px 6px rgba(0, 0, 0, 0.12);
        text-transform: capitalize;
        letter-spacing: 0.3px;
        transition: transform 0.2s ease, box-shadow 0.2s ease;
    }

    .tempo-badge:hover {
        transform: translateY(-1px);
        box-shadow: 0 3px 10px rgba(0, 0, 0, 0.15);
    }

    .loading-card,
    .error-card {
        grid-column: 1 / -1;
        background: white;
        border-radius: 20px;
        box-shadow:
            0 4px 20px rgba(0, 0, 0, 0.05),
            0 1px 3px rgba(0, 0, 0, 0.1);
        padding: 3rem 2rem;
        text-align: center;
        border: 1px solid #e2e8f0;
    }

    .error-card {
        border-left: 4px solid #e53e3e;
        color: #c53030;
    }

    .info-section {
        background: white;
        padding: 5rem 2rem;
    }

    .info-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        gap: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .info-card {
        padding: 2.5rem;
        border-radius: 16px;
        text-align: center;
        border: 1px solid #e2e8f0;
        position: relative;
        overflow: hidden;
    }

    .info-card::before {
        content: "";
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 3px;
    }

    .tempo-bleu {
        background: #f7fafc;
    }

    .tempo-bleu::before {
        background: linear-gradient(135deg, #3182ce 0%, #63b3ed 100%);
    }

    .tempo-blanc {
        background: #fafafa;
    }

    .tempo-blanc::before {
        background: linear-gradient(135deg, #718096 0%, #a0aec0 100%);
    }

    .tempo-rouge {
        background: #fff9f9;
    }

    .tempo-rouge::before {
        background: linear-gradient(135deg, #e53e3e 0%, #fc8181 100%);
    }

    .tempo-non-defini {
        background: #f5f5f5;
    }

    .tempo-non-defini::before {
        background: linear-gradient(135deg, #9e9e9e 0%, #bdbdbd 100%);
    }

    .info-card h3 {
        color: #1a202c;
        font-size: 1.5rem;
        margin: 0 0 1rem 0;
        font-weight: 700;
        position: relative;
        z-index: 1;
    }

    .info-card p {
        color: #4a5568;
        margin: 0;
        line-height: 1.6;
        position: relative;
        z-index: 1;
    }

    @media (max-width: 768px) {
        .hero {
            padding: 3rem 1rem;
        }

        .logo-section {
            flex-direction: column;
            gap: 1.5rem;
        }

        .edf-logo {
            font-size: 1.25rem;
            padding: 0.8rem 1.5rem;
        }

        h1 {
            font-size: 2.75rem;
        }

        .dates-container {
            grid-template-columns: 1fr;
            gap: 2rem;
        }

        .date-card {
            padding: 2.5rem 1.5rem;
        }

        .current-date {
            font-size: 1.5rem;
        }

        .info-grid {
            grid-template-columns: 1fr;
            gap: 1.5rem;
        }

        .info-card {
            padding: 2rem;
        }
    }
</style>
