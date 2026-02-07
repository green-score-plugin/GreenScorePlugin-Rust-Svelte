# GreenScore Extension - Chrome

## Environnement de développement
- **Système d'exploitation** : Linux / macOS / Windows
- **Node.js** : v25.2.0 ou supérieur
- **npm** : v11.6.2 ou supérieur

## Structure du projet
- `assets/` : Icônes et ressources visuelles
- `_locales/` : Fichiers de traduction (fr, en, etc.)
- `background.js` : Script principal d'arrière-plan
- `config.js` : Configuration de l'extension
- `manifest.json` : Manifest Chrome (déclare permissions, scripts, default_locale...)
- `popup.html` / `popup.js` / `popup.css` : Interface utilisateur de la popup
- `package.json` : Outils de développement (non inclus dans le zip de déploiement)

## Internationalisation
- La langue de l'extension s'adapte automatiquement à la langue du navigateur Chrome.
- Si la langue du navigateur n'est pas supportée, l'anglais est utilisé par défaut (voir `default_locale` dans le manifest).
- Les textes dynamiques et les équivalents sont traduits automatiquement via les fichiers `_locales/en/messages.json` et `_locales/fr/messages.json`.

## Déploiement d'une nouvelle version (Chrome Web Store ou local)
1. **Préparer le dossier de build** :
   - Inclure uniquement les fichiers/dossiers suivants dans le zip :
     - `_locales/`
     - `assets/`
     - `background.js`
     - `config.js`
     - `manifest.json`
     - `popup.html`
     - `popup.js`
     - `popup.css`
2. **Créer l'archive** :
   - Sélectionner tous ces fichiers/dossiers puis créer un fichier `.zip` (ne pas zipper le dossier parent, mais bien le contenu).
3. **Charger ou publier** :
   - Pour tester en local :
     - Aller dans `chrome://extensions/`, activer le mode développeur, puis "Charger l'extension non empaquetée" et sélectionner le dossier dézippé.
   - Pour publier :
     - Uploader le `.zip` sur le Chrome Web Store.

## Notes
- Le code source est en **JavaScript Vanilla** (pas de transpilation/minification nécessaire).
- Les dépendances de développement (npm, package.json) ne sont pas nécessaires pour le fonctionnement de l'extension, uniquement pour le développement.
- Pour toute modification de traduction, éditer les fichiers dans `_locales/en/messages.json` et `_locales/fr/messages.json`.
