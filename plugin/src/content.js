// content.js: applique la langue envoyée par le popup au site actif
(function () {
  function applyLocale(locale) {
    try {
      if (!locale) return;
      document.documentElement.lang = locale;

      // Remplacer les éléments marqués avec data-i18n si présents sur le site
      // Les sites doivent fournir un objet window.GREENSCORE_I18N si souhaité
      if (window.GREENSCORE_I18N && typeof window.GREENSCORE_I18N === 'object') {
        const dict = window.GREENSCORE_I18N[locale] || window.GREENSCORE_I18N['en'] || {};
        document.querySelectorAll('[data-gs-i18n]').forEach((el) => {
          const key = el.dataset.gsI18n;
          if (key && dict[key]) {
            el.textContent = dict[key];
          }
        });
      }
    } catch (e) {
      console.warn('Erreur lors de l\'application de la locale:', e);
    }
  }

  // Écoute les messages depuis l'extension
  window.addEventListener('message', (event) => {
    if (!event.data || event.data.source !== 'greenscore-extension') return;
    if (event.data.type === 'setLocale') {
      applyLocale(event.data.locale);
    }
  });

  // Permet l'exécution si l'extension envoie un message via tabs.sendMessage
  try {
    const runtime = (typeof browser !== 'undefined' ? browser.runtime : (typeof chrome !== 'undefined' ? chrome.runtime : null));
    if (runtime && runtime.onMessage) {
      runtime.onMessage.addListener((message) => {
        if (message && message.type === 'setLocale') {
          applyLocale(message.locale);
        }
      });
    }
  } catch (e) {
    // ignore
  }
})();

