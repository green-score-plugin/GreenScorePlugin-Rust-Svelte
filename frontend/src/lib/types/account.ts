export interface User {
    id: number;
    id_organisation?: number;
    id_service?: number;
    email: string;
    prenom: string;
    nom: string;
    total_carbon_footprint: number;
}

export interface Organisation {
    id: number;
    nom: string;
    code: string;
    siret?: string;
    est_admin: boolean; 
}

export interface Service {
    id: number;
    id_organisation: number;
    nom: string;
}

export interface UserFull {
    user: User;
    organisation: Organisation[];
    service?: Service;
}
