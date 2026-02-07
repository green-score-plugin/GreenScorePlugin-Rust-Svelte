let gCO2eValue;
let currentLang = 'fr';
let translations = {};


function initLanguage(forcedLang = null) {
  // Détermination de la langue
  if (forcedLang) {
    currentLang = forcedLang;
    localStorage.setItem('gs_plugin_lang', forcedLang);
  } else {
    const storedLang = localStorage.getItem('gs_plugin_lang');
    if (storedLang) {
      currentLang = storedLang;
    } else {
      const browserLang = navigator.language.split('-')[0];
      currentLang = ['fr', 'en'].includes(browserLang) ? browserLang : 'en';
    }
  }

  translations = TRANSLATIONS[currentLang];

  updateFlagsUI();
  applyStaticTranslations();

  if (typeof refreshDynamicTexts === 'function') {
      refreshDynamicTexts();
  }
}

function t(key, params = {}) {
  let text = (translations && translations[key]) || key;
  Object.keys(params).forEach(k => {
    text = text.replace(`{${k}}`, params[k]);
  });
  return text;
}

function applyStaticTranslations() {
  document.querySelectorAll('[data-i18n]').forEach(el => {
    const key = el.getAttribute('data-i18n');
    if (translations[key]) {
        el.textContent = translations[key];
    }
  });
}

async function updateEquivalents() {
  try {
    console.log("Envoi de la requête à background.js...");
    const response = await browser.runtime.sendMessage({
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
          description.textContent = t(equivalent.name);
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
  await initLanguage();

  const frFlag = document.getElementById('lang-fr');
  const enFlag = document.getElementById('lang-en');

  if (frFlag) {
      frFlag.addEventListener('click', () => {
          if (currentLang !== 'fr') initLanguage('fr');
      });
  }
  if (enFlag) {
      enFlag.addEventListener('click', () => {
          if (currentLang !== 'en') initLanguage('en');
      });
  }

  let isLocalhost = false;

  // Écouteur pour le message localhost
  browser.runtime.onMessage.addListener((message) => {
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
        detailsButton.textContent = t("more_details");

        const rawUrl = `${CONFIG.BACKEND.WEBSITE_URL}/#`;
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
              browser.tabs.create({ url: detailsButton.href });
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

    function updateAverageConsumption(gCO2e) {
      const AVERAGE_CONSUMPTION = 0.74; // Valeur obtenue sur un calcul de moyenne sur plus de 100 sites

      if (gCO2e <= 0) {
        document.getElementById("average-consumption").textContent = t("negligible");
        return;
      }

      let multiplier = gCO2e / AVERAGE_CONSUMPTION;

      if (multiplier > 1) {
        document.getElementById(
          "average-consumption"
        ).textContent = t("higher", { mult: multiplier.toFixed(2) });
      } else {
        document.getElementById("average-consumption").textContent = t("lower", { mult: (1 / multiplier).toFixed(2) });
      }
    }

    try {
      const response = await browser.runtime.sendMessage({ type: "getgCO2e" });
      if (response && response.gCO2e !== undefined) {
        gCO2eValue = parseFloat(response.gCO2e).toFixed(2);
        updateColors(gCO2eValue);
        updateAverageConsumption(gCO2eValue);
      } else {
        console.warn("Pas de valeur gCO2e reçue");
        updateColors(0);
      }
    } catch (error) {
      console.error("Erreur récupération gCO2e:", error);
    }

    // Vérification du statut de connexion
    try {
      const userData = await browser.runtime.sendMessage({
        type: "checkLoginStatus",
      });
      const loginSection = document.querySelector(
        ".flex.font-outfit.text-sm.justify-center"
      );
      const detailsButton = document.getElementById("details-button");

      if (loginSection) {

        while (loginSection.firstChild) {
          loginSection.removeChild(loginSection.firstChild);
        }

        if (userData.isLoggedIn) {
          const connectedSpan = document.createElement("span");
          connectedSpan.id = "user-connected-msg";
          connectedSpan.className = "text-sm text-[#6D874B] font-bold";
          connectedSpan.textContent = t("logged_in");
          loginSection.appendChild(connectedSpan);
        } else {
          const promptSpan = document.createElement("span");
          promptSpan.id = "login-prompt-msg";
          promptSpan.className = "text-sm text-grey-950";
          promptSpan.textContent = t("save_prompt") + " ";

          const loginLink = document.createElement("a");
          loginLink.id = "login-link-action";
          loginLink.className = "text-[#6D874B] font-bold underline cursor-pointer";
          loginLink.textContent = t("sign_in");

          try {
            const parsed = new URL(CONFIG.BACKEND.LOGIN_URL);
            loginLink.href = parsed.toString();
            loginLink.addEventListener("click", (e) => {
              e.preventDefault();
              if (loginLink.href && loginLink.href !== "#") {
                browser.tabs.create({ url: loginLink.href });
              }
            });
          } catch (err) {
            loginLink.href = "#";
            loginLink.setAttribute("aria-disabled", "true");
            loginLink.addEventListener("click", (e) => e.preventDefault());
          }

          // Ajouter les éléments au DOM
          loginSection.appendChild(promptSpan);
          loginSection.appendChild(loginLink);
        }
      }

      // Gestion du bouton "Plus de détails"
      if (detailsButton) {
        detailsButton.addEventListener("click", async (e) => {
          e.preventDefault();

          if (userData.isLoggedIn) {
            await browser.runtime.sendMessage({
              type: "sendDataToDB"
            })
          }

          // Récupérer les détails actuels
          const response = await browser.runtime.sendMessage({
            type: "getFullDetails",
          });

          let url = CONFIG.BACKEND.DETAILS_URL;

          if (!userData.isLoggedIn) {
            // Construction des paramètres d'URL si non connecté
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

          // Ouvrir l'URL dans un nouvel onglet
          browser.tabs.create({ url: url });
        });
      }
    } catch (error) {
      console.error(
        "Erreur lors de la vérification de l'état de connexion:",
        error
      );
    }

    browser.runtime
      .sendMessage({ type: "getCountryAndUrl" })
      .then((response) => {
        if (response && response.country && response.url) {
          const urlElement = document.getElementById("site-url");
          const countryElement = document.getElementById("site-country");

          if (countryElement && urlElement) {
            countryElement.textContent = t("country_consumption_intro", { countryName: response.country });
            urlElement.textContent = response.url;
          }
        } else if (response.error) {
          console.error("Erreur :", response.error);
        }
      })
      .catch((error) => {
        console.error(
          "Erreur lors de la récupération du pays ou de l'URL :",
          error
        );
      });

    try {
      await updateEquivalents();
    } catch (error) {
      console.error("Erreur lors de la mise à jour des équivalents :", error);
    }
  }
});

function updateFlagsUI() {
    const frFlag = document.getElementById('lang-fr');
    const enFlag = document.getElementById('lang-en');

    if (frFlag && enFlag) {
        // Reset styles basics
        frFlag.style.opacity = '0.5';
        enFlag.style.opacity = '0.5';
        frFlag.style.borderBottom = 'none';
        enFlag.style.borderBottom = 'none';

        // Active style
        if (currentLang === 'fr') {
            frFlag.style.opacity = '1';
            frFlag.style.borderBottom = '2px solid #233430';
        } else {
            enFlag.style.opacity = '1';
            enFlag.style.borderBottom = '2px solid #233430';
        }
    }
}

function refreshDynamicTexts() {
    updateEquivalents();

    if (typeof gCO2eValue !== 'undefined') {
        const AVERAGE_CONSUMPTION = 0.74;
        const avgEl = document.getElementById("average-consumption");
        if (avgEl) {
             if (gCO2eValue <= 0) {
                 avgEl.textContent = t("negligible");
             } else {
                  let multiplier = gCO2eValue / AVERAGE_CONSUMPTION;
                  if (multiplier > 1) {
                     avgEl.textContent = t("higher", { mult: multiplier.toFixed(2) });
                  } else {
                     avgEl.textContent = t("lower", { mult: (1 / multiplier).toFixed(2) });
                  }
             }
        }
    }

    browser.runtime.sendMessage({ type: "getCountryAndUrl" }).then(response => {
         if (response && response.country) {
            const countryElement = document.getElementById("site-country");
            if (countryElement) countryElement.textContent = t("country_consumption_intro", { countryName: response.country });
         }
    });

    const promptSpan = document.getElementById("login-prompt-msg");
    if (promptSpan) {
         promptSpan.textContent = t("save_prompt") + " ";
    }

    const loginLink = document.getElementById("login-link-action");
    if(loginLink) {
        loginLink.textContent = t("sign_in");
    }

    const connectedMsg = document.getElementById("user-connected-msg");
    if (connectedMsg) {
        connectedMsg.textContent = t("logged_in");
    }

    const detailsBtn = document.getElementById("details-button");
    if (detailsBtn) {
        detailsBtn.textContent = t("more_details");
    }
}
