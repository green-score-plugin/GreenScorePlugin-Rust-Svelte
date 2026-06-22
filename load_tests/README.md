# Load Tests

Ce dossier contient des tests de charge pour le backend utilisant Locust.

## Installation

Il est recommandé d'utiliser un environnement virtuel Python.

```bash
# Windows (Bash)
python -m venv venv
source venv/Scripts/activate
pip install -r requirements.txt

# Linux / Mac
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

## Lancer les tests

Assurez-vous que le backend tourne (par défaut sur `http://localhost:3000` ou `http://localhost:8080`, vérifiez votre configuration).

```bash
locust
```

Ensuite, ouvrez votre navigateur sur `http://localhost:8089`.

Vous pouvez aussi lancer sans interface graphique :

```bash
locust --headless --users 10 --spawn-rate 1 -H http://localhost:3000
```

