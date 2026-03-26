let gCO2eValue;
let currentLang = 'fr';
let translations = {};

async function loadTranslations(lang) {
  try {
    const response = await fetch(`./i18n/${lang}.json`);
    if (!response.ok) {
      throw new Error(`Failed to load translations for ${lang}`);
    }
    return await response.json();
  } catch (error) {
    if (lang !== 'fr') {
      return await loadTranslations('fr');
    }
    return {};
  }
}

async function initLanguage(forcedLang = null) {
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

  translations = await loadTranslations(currentLang);
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
    const response = await browser.runtime.sendMessage({
      type: "getEquivalent",
      count: 3,
    });

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
    }
  } catch {
  }
}

document.addEventListener("DOMContentLoaded", async () => {
  await initLanguage();

  const frFlag = document.getElementById('lang-fr');
  const enFlag = document.getElementById('lang-en');

  if (currentLang === 'fr') {
    frFlag?.classList.add('flag-active');
    enFlag?.classList.remove('flag-active');
  } else {
    enFlag?.classList.add('flag-active');
    frFlag?.classList.remove('flag-active');
  }

  if (frFlag) {
    frFlag.addEventListener('click', async () => {
      if (currentLang !== 'fr') {
        await initLanguage('fr');
        await updateEquivalents();
        frFlag.classList.add('flag-active');
        enFlag.classList.remove('flag-active');
      }
    });
  }

  if (enFlag) {
    enFlag.addEventListener('click', async () => {
      if (currentLang !== 'en') {
        await initLanguage('en');
        await updateEquivalents();
        enFlag.classList.add('flag-active');
        frFlag.classList.remove('flag-active');
      }
    });
  }

  let isLocalhost = false;

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
        title.textContent = message.message || "";

        const detailsButton = document.createElement("a");
        detailsButton.id = "details-button";
        detailsButton.className = "flex justify-center items-center py-2 px-4 text-white font-outfit font-medium bg-gs-green-950 rounded-lg";
        detailsButton.textContent = t("more_details");

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
              browser.tabs.create({ url: detailsButton.href });
            }
          });
        }
      }
    }
  });

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
      const AVERAGE_CONSUMPTION = 0.74;

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

    // Lance toutes les requêtes en parallèle
    const [gCO2eResponse, userData, countryResponse] = await Promise.all([
      browser.runtime.sendMessage({ type: "getgCO2e" }),
      browser.runtime.sendMessage({ type: "checkLoginStatus" }),
      browser.runtime.sendMessage({ type: "getCountryAndUrl" })
    ]);

    // 1. Gestion de gCO2e
    try {
      if (gCO2eResponse && gCO2eResponse.gCO2e !== undefined) {
        gCO2eValue = parseFloat(gCO2eResponse.gCO2e).toFixed(2);
        updateColors(gCO2eValue);
        updateAverageConsumption(gCO2eValue);
      } else {
        updateColors(0);
      }
    } catch (error) {
       console.error("Erreur récupération gCO2e:", error);
    }

    // 2. Vérification du statut de connexion
    try {
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

          loginSection.appendChild(promptSpan);
          loginSection.appendChild(loginLink);
        }
      }

      if (detailsButton) {
        detailsButton.addEventListener("click", async (e) => {
          e.preventDefault();

          if (userData.isLoggedIn) {
            await browser.runtime.sendMessage({
              type: "sendDataToDB"
            })
          }

          const response = await browser.runtime.sendMessage({
            type: "getFullDetails",
          });

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

          browser.tabs.create({ url: url });
        });
      }
    } catch {
    }

    // 3. Gestion Pays/URL
    if (countryResponse && countryResponse.country && countryResponse.url) {
      const urlElement = document.getElementById("site-url");
      const countryElement = document.getElementById("site-country");

      if (countryElement && urlElement) {
        countryElement.textContent = t("country_consumption_intro", { countryName: countryResponse.country });
        urlElement.textContent = countryResponse.url;
      }
    }

    // 4. Equivalents
    try {
      await updateEquivalents();
    } catch {
    }
  }
});


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
