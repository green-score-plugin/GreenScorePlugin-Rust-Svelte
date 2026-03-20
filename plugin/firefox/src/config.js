const BASE_DOMAIN = "greenscore.alwaysdata.net";
const BASE_URL = `https://${BASE_DOMAIN}`;

const CONFIG = {
    BACKEND: {
      PLUGIN_BACKEND_URL: `${BASE_URL}/plugin`, // URL du backend du plugin

      DOMAIN: BASE_DOMAIN, // Utilisé pour récupérer l'ID utilisateur
      
      BASE_URL, // URL du site Greenscore
      
      LOGIN_URL: `${BASE_URL}/login`, // URL de la page de connexion
      DETAILS_URL: `${BASE_URL}/derniere-page-consultee` // URL de la page de détails
    }
  };
  
  if (typeof browser !== 'undefined') {
  } else if (typeof module !== 'undefined') {
    module.exports = CONFIG;
  }