pub mod controllers {
    pub mod account_controller;
    pub mod auth_controller;
    pub mod home_controller;
    pub mod lpc_controller;
    pub mod mo_controller;
    pub mod my_data_controller;
    pub mod plugin_controller;
}

pub mod models {
    pub mod user;
    pub mod service;
    pub mod organisation;
    pub mod advice;
    pub mod monitored_website;
    pub mod equivalent;
}

pub mod dto {
    pub mod user_full;
    pub mod lpc_dto;
}

pub mod repository {
    pub mod user_repository;
    pub mod organisation_repository;
    pub mod service_repository;
    pub mod advice_repository;
    pub mod monitored_website_repository;
    pub mod equivalent_repository;
}

pub mod service {
    pub mod auth_service;
    pub mod advice_service;
    pub mod monitored_website_service;
    pub mod equivalent_service;
}

pub mod green_score;
pub mod router;
pub mod session_store;
pub mod cleanup;
