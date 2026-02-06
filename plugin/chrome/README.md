# GreenScore Extension - Source Code

## Environnement de développement
- **Système d'exploitation** : Linux / macOS / Windows
- **Node.js** : v25.2.0
- **npm** : v11.6.2

## Structure du projet
- `source/` : Contient la logique métier (background.js, popup.js).
- `assets/` : Contient les icônes et ressources visuelles.
- `manifest.json` : Configuration de l'extension Firefox.
- `package.json` : Gestion des outils de développement.

## Instructions de compilation (Build)
L'extension est développée en **Vanilla JS**. Aucune étape de minification ou de transpilation n'est requise. Le code source est identique au code distribué.

1. Installez les dépendances de développement :
   ```bash
   npm install