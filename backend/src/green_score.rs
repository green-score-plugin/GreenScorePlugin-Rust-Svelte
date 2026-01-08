pub fn calculate_green_score(carbon_footprint: f64) -> (String, String) {
    let echelle: f64 = 0.25;
    let mut letter_green_score: String = String::new();
    let mut env_nomination: String = String::new();

    if carbon_footprint < echelle {
        letter_green_score = "A".to_string();
        env_nomination = "Maître des forêts".to_string();
    }
    else if carbon_footprint >= echelle && carbon_footprint < 2.0 * echelle {
        letter_green_score = "B".to_string();
        env_nomination = "Protecteur des Bois".to_string();
    }
    else if carbon_footprint >= 2.0 * echelle && carbon_footprint < 3.0 * echelle {
        letter_green_score = "C".to_string();
        env_nomination = "Frère des Arbres".to_string();
    }
    else if carbon_footprint >= 3.0 * echelle && carbon_footprint < 4.0 * echelle {
        letter_green_score = "D".to_string();
        env_nomination = "Initié de la Nature".to_string();
    }
    else if carbon_footprint >= 4.0 * echelle && carbon_footprint < 5.0 * echelle {
        letter_green_score = "E".to_string();
        env_nomination = "Explorateur Imprudent".to_string();
    }
    else if carbon_footprint >= 5.0 * echelle && carbon_footprint < 6.0 * echelle {
        letter_green_score = "F".to_string();
        env_nomination = "Tempête Numérique".to_string();
    }
    else if carbon_footprint >= 6.0 {
        letter_green_score = "G".to_string();
        env_nomination = "Desctructeur des Écosystèmes".to_string();
    }

    (letter_green_score, env_nomination)
}