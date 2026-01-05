const CONFIG = {
    BACKEND: {
      PLUGIN_BACKEND_URL: "http://127.0.0.1:3000", // URL du backend du plugin

      DOMAIN: "127.0.0.1", // Utilisé pour récupérer l'ID utilisateur
      
      WEBSITE_URL: "http://127.0.0.1:5173", // URL du site Greenscore
      
      LOGIN_URL: "http://127.0.0.1:5173/login", // URL de la page de connexion
      DETAILS_URL: "http://127.0.0.1:5173/derniere-page-web-consultee" // URL de la page de détails
    }
  };
  
  if (typeof browser !== 'undefined') {
  } else if (typeof module !== 'undefined') {
    module.exports = CONFIG;
  }