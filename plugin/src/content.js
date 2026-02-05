(function () {
  function applyLocale(locale) {
    try {
      if (!locale) return;
      document.documentElement.lang = locale;

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

  window.addEventListener('message', (event) => {
    if (!event.data || event.data.source !== 'greenscore-extension') return;
    if (event.data.type === 'setLocale') {
      applyLocale(event.data.locale);
    }
  });

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
  }
})();

