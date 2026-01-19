# GreenScore Web

## Prérequis
- **Rust**
- **Node**
- **Firefox**
---

## Installation et mise en route

### 1. Cloner le projet
Clonez le projet depuis le dépôt GitHub :
```bash
git clone https://github.com/green-score-plugin/GreenScorePlugin-Rust-Svelte
```

### 2. Configurer le site web
Accédez à la racine du projet puis au dossier **GreenScorePlugin-Rust-Svelte** :
```bash
cd frontend
cd backend
```

#### Ajouter les .env
Pour que le projet fonctionne en local, il vous faut les identifiants de la base de données dans deux fichier .env

- Front-end du site : [.env](BACKEND_URL=http://127.0.0.1:3000), que vous devrez mettre à la racine du dossier 'GreenScorePlugin-Rust-Svelte/frontend'
- Backend du site : [.env]("FRONTEND_URL=http://localhost:5173 BACKEND_URL=127.0.0.1:3000"), que vous devrez mettre à la racine du dossier 'GreenScorePlugin-Rust-Svelte/backend'

Pensez a correctement les renommer '.env' et à changer les valeurs dans les .env pour qu'ils fonctionnent correctement avec votre base de données.
Pour que la récupération de l'empreinte carbone fonctionne, pensez à créer une clé API ElectricityMap et l'ajouter correctement dans le .env du front-end.

#### Modifier les adresses du plugin
Pour que le plugin puisse fonctionner correctement, il vous faut paramétrer le fichier [config.js](./Plugin/src/config.js) pour permettre au plugin une connexion vers son backend, ainsi que l'accès au site et l'API.

#### Base de données
Pour que la base de données puisse fonctionner, il vous faut utiliser le .sql suivant :

[greenscore.sql](./db/greenscoreweb_database.sql)

#### Installer les dépendances du front-end
Dans le dossier front-end :
```bash
npm install
```

#### Installer les dépendances back-end
Installez les dépendances front-end nécessaires avec npm :
```bash
cargo install sqlx
cargo sqlx prepare
```

#### Démarrer le serveur Rust
Lancez le serveur local Rust :
```bash
cargo run
```

#### Démarrer le serveur SvelteKit
```bash
npm run dev
```

Laissez ces terminalaux ouvert.


### 4. Configurer le plugin dans Firefox
1. Ouvrez **Firefox**.
2. Cliquez sur l’icône en forme de puzzle en haut à droite.
3. Cliquez sur **Gérer les extensions.**
4. Cliquez sur la roue dentée pour accéder à **Gestion de vos extensions**.
5. Sélectionnez **Déboguer les modules**.
6. Cliquez sur **Charger un module complémentaire temporaire...**.
7. Accédez au dossier `Plugin` à la racine du projet.
8. Sélectionnez le fichier `manifest.json` et cliquez sur **Ouvrir**.

### 6. Tester le plugin
Le plugin est maintenant actif !

1. Rendez-vous sur [amazon.fr](https://www.amazon.fr) avec Firefox.
2. Testez les fonctionnalités du plugin.
