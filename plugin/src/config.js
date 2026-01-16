const CONFIG = {
    BACKEND: {
      PLUGIN_BACKEND_URL: "https://greenscore.alwaysdata.net/plugin", // URL du backend du plugin

      DOMAIN: "greenscore.alwaysdata.net", // Utilisé pour récupérer l'ID utilisateur
      
      WEBSITE_URL: "https://greenscore.alwaysdata.net", // URL du site Greenscore
      
      LOGIN_URL: "https://greenscore.alwaysdata.net/login", // URL de la page de connexion
      DETAILS_URL: "https://greenscore.alwaysdata.net/derniere-page" // URL de la page de détails
    }
  };
  
  if (typeof browser !== 'undefined') {
  } else if (typeof module !== 'undefined') {
    module.exports = CONFIG;
  }