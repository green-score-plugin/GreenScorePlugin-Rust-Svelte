let gCO2eValue;

// === Système i18n dynamique ===
let currentMessages = {};

async function loadMessages(locale) {
  let url = chrome.runtime.getURL(`_locales/${locale}/messages.json`);
  try {
    const response = await fetch(url);
    if (!response.ok) throw new Error('Locale not found');
    return await response.json();
  } catch (e) {
    const defaultLocale = 'en';
    url = chrome.runtime.getURL(`_locales/${defaultLocale}/messages.json`);
    const response = await fetch(url);
    return await response.json();
  }
}

function getI18n(key, fallback = '') {
  return currentMessages[key]?.message || fallback;
}

function updateLoginSection(isLoggedIn) {
  const loginSection = document.querySelector(
    ".flex.font-outfit.text-sm.justify-center"
  );
  if (!loginSection) return;
  while (loginSection.firstChild) {
    loginSection.removeChild(loginSection.firstChild);
  }
  if (isLoggedIn) {
    const connectedSpan = document.createElement("span");
    connectedSpan.className = "text-sm text-[#6D874B] font-bold";
    connectedSpan.textContent = getI18n('connected', 'Vous êtes connecté !');
    loginSection.appendChild(connectedSpan);
  } else {
    const promptSpan = document.createElement("span");
    promptSpan.className = "text-sm text-grey-950";
    promptSpan.textContent = getI18n('save_result', 'Vous souhaitez enregistrer ce résultat ?') + "\u00A0";
    const loginLink = document.createElement("a");
    loginLink.className = "text-[#6D874B] font-bold underline";
    loginLink.textContent = getI18n('login', 'Se connecter');
    try {
      const parsed = new URL(CONFIG.BACKEND.LOGIN_URL);
      loginLink.href = parsed.toString();
      loginLink.addEventListener("click", (e) => {
        e.preventDefault();
        if (loginLink.href && loginLink.href !== "#") {
          chrome.tabs.create({ url: loginLink.href });
        }
      });
    } catch (err) {
      loginLink.href = "#";
      loginLink.setAttribute("aria-disabled", "true");
      loginLink.addEventListener("click", (e) => e.preventDefault());
    }
    loginSection.appendChild(promptSpan);
    loginSection.appendChild(loginLink);
  }
}

function updateCountrySentence(country) {
  const countryElement = document.getElementById("site-country");
  if (countryElement) {
    const template = getI18n('country_sentence', 'Dans votre pays (*Pays*), cette page consomme');
    countryElement.textContent = template.replace('*Pays*', country).replace('*Country*', country);
  }
}

function updateAverageConsumption(gCO2e) {
  const AVERAGE_CONSUMPTION = 0.74;
  const avgElem = document.getElementById("average-consumption");
  if (!avgElem) return;
  if (gCO2e <= 0) {
    avgElem.textContent = getI18n('negligible', 'négligeable comparé');
    return;
  }
  let multiplier = gCO2e / AVERAGE_CONSUMPTION;
  if (multiplier > 1) {
    avgElem.textContent = `${multiplier.toFixed(2)}x ${getI18n('superior', 'supérieur')}`;
  } else {
    avgElem.textContent = `${(1 / multiplier).toFixed(2)}x ${getI18n('inferior', 'inférieur')}`;
  }
}

async function updateEquivalents() {
  try {
    console.log("Envoi de la requête à background.js...");
    const response = await chrome.runtime.sendMessage({
      type: "getEquivalent",
      count: 3, // Nombre d'équivalents à récupérer
    });
    console.log("Réponse reçue :", response);

    if (response && response.success && response.equivalents) {
      const cards = document.querySelectorAll(".comparison-card");
      response.equivalents.forEach((equivalent, index) => {
        if (cards[index]) {
          const card = cards[index];
          const img = card.querySelector("img");
          const valueElement = card.querySelector("p.text-xl");
          const description = card.querySelector("p.text-xs");

          img.src = equivalent.image || "../assets/images/default.svg";
          valueElement.textContent = equivalent.value;

          const translationKey = equivalent.name.replace(/\./g, '_');
          description.textContent = getI18n(translationKey, equivalent.name);
        }
      });
    } else {
      console.error(
        "Erreur dans la réponse reçue :",
        response ? response.error : "Réponse indéfinie"
      );
    }
  } catch (error) {
    console.error("Erreur dans updateEquivalents :", error);
  }
}

document.addEventListener("DOMContentLoaded", async () => {
  // Utilise la langue du navigateur uniquement
  const browserLocale = chrome.i18n.getUILanguage().split('-')[0] || 'en';
  const messages = await loadMessages(browserLocale);
  currentMessages = messages;
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    if (messages[key] && messages[key].message) {
      el.textContent = messages[key].message;
    }
  });

  let isLocalhost = false;

  // Écouteur pour le message localhost
  chrome.runtime.onMessage.addListener((message) => {
    if (message.type === "localhostDetected") {
      isLocalhost = true;
      const mainContainer = document.getElementById("main-container");
      if (mainContainer) {
        while (mainContainer.firstChild) {
          mainContainer.removeChild(mainContainer.firstChild);
        }

        const wrapper = document.createElement("div");
        wrapper.className = "flex flex-col items-center justify-center h-full text-center gap-6 py-4 px-4";

        const title = document.createElement("p");
        title.className = "text-3xl font-bold font-outfit";
        title.textContent = message.message || ""; // usage de textContent pour éviter l'injection

        const detailsButton = document.createElement("a");
        detailsButton.id = "details-button";
        detailsButton.className = "flex justify-center items-center py-2 px-4 text-white font-outfit font-medium bg-gs-green-950 rounded-lg";
        detailsButton.textContent = "Plus d'informations";

        const rawUrl = `${CONFIG.BACKEND.BASE_URL}/#`;
        try {
          const parsed = new URL(rawUrl);
          detailsButton.href = parsed.toString();
        } catch (e) {
          detailsButton.href = "#";
          detailsButton.setAttribute("aria-disabled", "true");
        }

        wrapper.appendChild(title);
        wrapper.appendChild(detailsButton);
        mainContainer.appendChild(wrapper);

        if (detailsButton) {
          detailsButton.addEventListener("click", (event) => {
            event.preventDefault();
            if (detailsButton.href && detailsButton.href !== "#") {
              chrome.tabs.create({ url: detailsButton.href });
            }
          });
        }
      }
    }
  });

  // Si ce n'est pas localhost, continuer avec le reste des fonctionnalités
  if (!isLocalhost) {
    function getColorClass(gCO2e) {
      const value = Number(gCO2e);

      if (value <= 0.3) {
        return {
          text: "text-[#617D3B]",
          bg: "bg-[#ECFDF2]",
          border: "border-[#6D874B]",
        };
      } else if (value <= 0.7) {
        return {
          text: "text-[#EAC13A]",
          bg: "bg-[#FFF1C5]",
          border: "border-[#EAC13A]",
        };
      } else if (value <= 1) {
        return {
          text: "text-[#E98035]",
          bg: "bg-[#F9D2B6]",
          border: "border-[#E98035]",
        };
      } else {
        return {
          text: "text-[#BD2525]",
          bg: "bg-[#FFB7B7]",
          border: "border-[#BD2525]",
        };
      }
    }

    function updateColors(gCO2e) {
      const colorClasses = getColorClass(gCO2e);

      const gCO2eContainer = document.getElementById("gCO2e-container");
      const gCO2eValue = document.getElementById("gCO2e-value");

      if (gCO2eContainer && gCO2eValue) {
        gCO2eContainer.className = `flex items-baseline font-outfit ${colorClasses.text}`;
        gCO2eValue.textContent = `${gCO2e}\u00A0`;
      }

      const comparisonCards = document.querySelectorAll(".comparison-card");
      comparisonCards.forEach((card) => {
        card.className = `comparison-card flex flex-col p-2 w-[120px] h-[120px] ${colorClasses.bg} ${colorClasses.text} gap-2 border ${colorClasses.border} rounded-[4px]`;
      });
    }

    // Lance toutes les requêtes en parallèle
    const [gCO2eResponse, userData, countryResponse] = await Promise.all([
      chrome.runtime.sendMessage({ type: "getgCO2e" }),
      chrome.runtime.sendMessage({ type: "checkLoginStatus" }),
      chrome.runtime.sendMessage({ type: "getCountryAndUrl" })
    ]);

    // 1. Gestion de gCO2e
    try {
      if (gCO2eResponse && gCO2eResponse.gCO2e !== undefined) {
        gCO2eValue = parseFloat(gCO2eResponse.gCO2e).toFixed(2);
        updateColors(gCO2eValue);
        updateAverageConsumption(gCO2eValue);
      } else {
        console.warn("Pas de valeur gCO2e reçue");
        updateColors(0);
      }
    } catch (error) {
      console.error("Erreur gCO2e processing:", error);
    }

    // 2. Gestion du Login
    try {
      window._isLoggedIn = userData.isLoggedIn; 
      updateLoginSection(userData.isLoggedIn);
      const detailsButton = document.getElementById("details-button");

      if (detailsButton) {
        detailsButton.addEventListener("click", async (e) => {
          e.preventDefault();

          if (userData.isLoggedIn) {
            await chrome.runtime.sendMessage({ type: "sendDataToDB" });
          }


          const response = await chrome.runtime.sendMessage({ type: "getFullDetails" });

          let url = CONFIG.BACKEND.DETAILS_URL;

          if (!userData.isLoggedIn) {
            const params = new URLSearchParams({
              country: response.country || "",
              url_full: response.urlFull || "",
              totalConsu: gCO2eValue || 0,
              pageSize: response.totalResourceSize || 0,
              loadingTime: response.loadTime || 0,
              queriesQuantity: response.totalRequests || 0,
            });
            url += "?" + params.toString();
          }
          chrome.tabs.create({ url: url });
        });
      }
    } catch (error) {
       console.error("Erreur login processing:", error);
    }

    // 3. Gestion Pays/URL
    if (countryResponse && countryResponse.country && countryResponse.url) {
      const urlElement = document.getElementById("site-url");
      const countryElement = document.getElementById("site-country");

      if (countryElement && urlElement) {
        const template = getI18n('country_sentence', 'Dans votre pays (*Pays*), cette page consomme');
        countryElement.textContent = template.replace('*Pays*', countryResponse.country).replace('*Country*', countryResponse.country);
        urlElement.textContent = countryResponse.url;
      }
    } else if (countryResponse && countryResponse.error) {
       console.error("Erreur pays :", countryResponse.error);
    }

    // 4. Equivalents (indépendant, peut être lancé en parallèle aussi)
    try {
      await updateEquivalents();
    } catch (error) {
      console.error("Erreur lors de la mise à jour des équivalents :", error);
    }
  }
});
