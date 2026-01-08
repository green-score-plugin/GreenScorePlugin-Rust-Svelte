// +page.server.ts
export async function load() {
    // Récupérer les données depuis votre API backend
    return {
        title: "Mes données",
        description: "Consultez vos statistiques de consommation",
        noDatas: false,
        myAverageDailyCarbonFootprint: null,
        averageDailyCarbonFootprint: null,
        messageAverageFootprint: null,
        usersIdsCharts: [],
        letterGreenScore: null,
        envNomination: null,
        equivalent1: null,
        totalConsu: null,
        totalConsuUnit: null,
        equivalent2: null,
        advice: null,
        adviceDev: null
    };
}
