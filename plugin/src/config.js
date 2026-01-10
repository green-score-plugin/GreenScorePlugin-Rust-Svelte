const CONFIG = {
    BACKEND: {
      PLUGIN_BACKEND_URL: "http://localhost:3000", // URL du backend du plugin

      DOMAIN: "localhost", // Utilisé pour récupérer l'ID utilisateur
      
      WEBSITE_URL: "http://localhost:5173", // URL du site Greenscore
      
      LOGIN_URL: "http://localhost:5173/login", // URL de la page de connexion
      DETAILS_URL: "http://localhost:5173/derniere-page" // URL de la page de détails
    }
  };
  
  if (typeof browser !== 'undefined') {
  } else if (typeof module !== 'undefined') {
    module.exports = CONFIG;
  }