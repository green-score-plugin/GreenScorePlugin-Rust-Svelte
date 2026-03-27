use rand::RngExt;
use sqlx::MySqlPool;
use sqlx::Error;
use crate::dto::consumption_data_point_dto::ConsumptionDataPoint;
use crate::repository::organisation_repository::OrganisationRepository;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::repository::monitored_website_repository::MonitoredWebsiteRepository;
use crate::service::equivalent_service::EquivalentService;
use crate::dto::mo_infos_dto::MyOrganizationInfos;
use crate::dto::top_polluting_site_dto::TopPollutingSite;
use crate::dto::update_organisation_request_dto::UpdateOrganisationRequest;
use crate::models::organisation::Organisation;
use crate::repository::user_repository::UserRepository;

pub struct OrganisationService;

impl OrganisationService {

    pub async fn find_id_by_siret(pool: &MySqlPool, siret: String) -> Result<Option<i64>, Error> {
        OrganisationRepository::find_id_by_siret(pool, &siret).await
    }
    pub async fn organization_informations(pool: &MySqlPool, orga_id: i64, user_id: i64) -> Result<MyOrganizationInfos, Error>
    {
        let name: String = OrganisationRepository::organization_name(pool, orga_id).await.unwrap_or(None).unwrap_or_else(|| "Organisation inconnue".to_string());
        let average_daily_carbon_footprint: f64 = MonitoredWebsiteService::average_daily_carbon_footprint_for_organization(pool, orga_id).await;
        let total_consumption: f64 = MonitoredWebsiteService::total_organization_consumption(pool, orga_id).await.unwrap_or(None).unwrap_or(0.0);

        let equivalent = EquivalentService::equivalent(pool, Some(user_id), 1, total_consumption).await.ok().and_then(|mut v| v.pop());

        Ok(MyOrganizationInfos {
            name,
            average_daily_carbon_footprint,
            equivalent,
            total_consumption,
        })
    }

    pub async fn get_daily_organization_consumption(pool: &MySqlPool, orga_id: i64) -> Result<Vec<ConsumptionDataPoint>, Error> {
        MonitoredWebsiteRepository::get_daily_organization_consumption(pool, orga_id).await
    }

    pub async fn get_weekly_organization_consumption(pool: &MySqlPool, orga_id: i64) -> Result<Vec<ConsumptionDataPoint>, Error> {
        MonitoredWebsiteRepository::get_weekly_organization_consumption(pool, orga_id).await
    }

    pub async fn get_monthly_organization_consumption(pool: &MySqlPool, orga_id: i64) -> Result<Vec<ConsumptionDataPoint>, Error> {
        MonitoredWebsiteRepository::get_monthly_organization_consumption(pool, orga_id).await
    }

    pub async fn get_top5_polluting_sites_by_organization(pool: &MySqlPool, org_id: i64) -> Result<Vec<TopPollutingSite>, Error> {
        let top_polluting_sites: Vec<TopPollutingSite> = MonitoredWebsiteRepository::get_top5_polluting_sites_by_organization(pool, org_id).await?;

        Ok(top_polluting_sites.into_iter()
            .map(|top_polluting_site: TopPollutingSite| TopPollutingSite {
                url_domain: top_polluting_site.url_domain,
                total_footprint: (top_polluting_site.total_footprint * 100.0).round() / 100.0
            })
            .collect())
    }

    pub async fn update_organisation_details(pool: &MySqlPool, current_org: &Organisation, payload: UpdateOrganisationRequest) -> Result<Organisation, String> {
        let new_name = &payload.name;

        if new_name != &current_org.nom {
            let existing = OrganisationRepository::find_id_by_name(pool, new_name).await.map_err(|e| e.to_string())?;
            if existing.is_some() {
                return Err("errors.org_name_exists".to_string());
            }
        }

        if let Some(ref new_siret) = payload.siret {
            if Some(new_siret) != current_org.siret.as_ref() {
                let existing = OrganisationRepository::find_id_by_siret(pool, new_siret).await.map_err(|e| e.to_string())?;
                if existing.is_some() {
                    return Err("errors.org_siret_exists".to_string());
                }
            }
        }

        OrganisationRepository::update_organisation(pool, current_org.id, new_name, payload.siret.clone()).await.map_err(|e| e.to_string())?;

        Ok(Organisation {
            id: current_org.id,
            nom: new_name.clone(),
            code: current_org.code.clone(),
            siret: payload.siret,
            est_admin: current_org.est_admin,
        })
    }

    pub async fn inscription_orga(
        pool: &MySqlPool,
        organisation_name: &str,
        siret: Option<String>,
        user_id: i64
    ) -> Result<(i64, String), String>
    {
        if OrganisationRepository::find_id_by_siret(pool, organisation_name).await.map_err(|_| "errors.db_error")?.is_some() {
            return Err("errors.org_exists".to_string());
        }

        let code = OrganisationService::generate_organisation_code();

        let siret_string = siret.map(|s| s.to_string());
        let organisation_id = OrganisationRepository::insert_organisation(pool, organisation_name, &code, siret_string)
            .await
            .map_err(|_| "errors.org_insert_error")?;

        let is_admin = true;
        UserRepository::join_organisation(pool, user_id, organisation_id, is_admin)
            .await
            .map_err(|_| "errors.org_join_error")?;

        Ok((organisation_id, code))
    }

    pub async fn delete_organization(pool: &MySqlPool, org_id: i64) -> Result<(), String> {
        OrganisationRepository::delete_organization(pool, org_id).await.map_err(|e| e.to_string())
    }

    pub fn generate_organisation_code() -> String {
        const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        const LENGTH: usize = 8;

        let mut rng = rand::rng();

        (0..LENGTH)
            .map(|_| {
                let idx = rng.random_range(0..CHARACTERS.len());
                CHARACTERS[idx] as char
            })
            .collect()
    }
}