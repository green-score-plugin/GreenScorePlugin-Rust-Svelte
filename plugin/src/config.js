const CONFIG = {
  BACKEND: {
    PLUGIN_BACKEND_URL: "https://greenscore.alwaysdata.net/plugin",
    DOMAIN: "greenscore.alwaysdata.net",
    WEBSITE_URL: "https://greenscore.alwaysdata.net",
    LOGIN_URL: "https://greenscore.alwaysdata.net/login",
    DETAILS_URL: "https://greenscore.alwaysdata.net/derniere-page-consultee"
  }
};

(function() {
  // If browser is already defined (Firefox), do nothing
  if (typeof browser !== "undefined") return;

  // We are in Chrome (or compatible)
  const chromeApi = (typeof chrome !== "undefined") ? chrome : null;

  // If no chrome API either, we can't do anything (not an extension context)
  if (!chromeApi) return;

  const polyfill = {
    runtime: {
      sendMessage: (message) => {
        return new Promise((resolve) => {
          chromeApi.runtime.sendMessage(message, (response) => {
            const error = chromeApi.runtime.lastError;
            resolve(error ? null : response);
          });
        });
      },
      onMessage: chromeApi.runtime.onMessage,
      getURL: (path) => chromeApi.runtime.getURL(path),
    },
    tabs: {
      query: (queryInfo) =>
        new Promise((resolve) => chromeApi.tabs.query(queryInfo, resolve)),
      create: (createProperties) =>
        new Promise((resolve) => chromeApi.tabs.create(createProperties, resolve)),
      sendMessage: (tabId, message) =>
        new Promise((resolve) => {
          chromeApi.tabs.sendMessage(tabId, message, (response) => {
            const error = chromeApi.runtime.lastError;
            resolve(error ? null : response);
          });
        }),
    },
    storage: {
      sync: {
        get: (keys) =>
          new Promise((resolve) => chromeApi.storage.sync.get(keys, resolve)),
        set: (items) =>
          new Promise((resolve) => chromeApi.storage.sync.set(items, resolve)),
      },
      local: {
        get: (keys) =>
          new Promise((resolve) => chromeApi.storage.local.get(keys, resolve)),
        set: (items) =>
          new Promise((resolve) => chromeApi.storage.local.set(items, resolve)),
      },
    },
    webNavigation: chromeApi.webNavigation,
    webRequest: chromeApi.webRequest,
    cookies: {
        getAll: (details) => new Promise(resolve => chromeApi.cookies.getAll(details, resolve))
    }
  };

  // Expose to global scope based on environment
  if (typeof globalThis !== 'undefined') {
      globalThis.browser = polyfill;
  } else if (typeof window !== 'undefined') {
      window.browser = polyfill;
  } else if (typeof self !== 'undefined') {
      self.browser = polyfill;
  }
})();

// Export pour l'environnement Node/Tests si nécessaire
if (typeof module !== 'undefined' && typeof module.exports !== 'undefined') {
  module.exports = CONFIG;
}
