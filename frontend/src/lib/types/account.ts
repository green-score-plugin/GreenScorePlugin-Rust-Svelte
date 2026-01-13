export interface User {
    id: number;
    email: string;
    nom: string;
    prenom: string;
}

export interface Organisation {
    id: number;
    nom: string;
    siret: string;
    code: string;
    admin_id: number;
}

export type Account =
    | ({ role: 'user' } & User)
    | ({ role: 'organisation' } & Organisation);