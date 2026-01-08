pub fn calculate_green_score(carbon_footprint: f64) -> (String, String) {
    let echelle: f64 = 0.25;

    let (letter_green_score, env_nomination) = if carbon_footprint < echelle {
        ("A", "Maître des forêts")
    } else if carbon_footprint < 2.0 * echelle {
        ("B", "Protecteur des Bois")
    } else if carbon_footprint < 3.0 * echelle {
        ("C", "Frère des Arbres")
    } else if carbon_footprint < 4.0 * echelle {
        ("D", "Initié de la Nature")
    } else if carbon_footprint < 5.0 * echelle {
        ("E", "Explorateur Imprudent")
    } else if carbon_footprint < 6.0 * echelle {
        ("F", "Tempête Numérique")
    } else {
        ("G", "Destructeur des Écosystèmes")
    };

    eprintln!("Carbon footprint: {}, Green Score: {}, Nomination: {}", carbon_footprint, letter_green_score, env_nomination);

    (letter_green_score.to_string(), env_nomination.to_string())
}
