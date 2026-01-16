import {fail, redirect} from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BACKEND_URL } from '$lib/config';
import {setSessionCookie, invalidateCache} from "$lib/server/session.ts";

export const load: PageServerLoad = async ({ fetch, request }) => {
    try {
        const response = await fetch(`${BACKEND_URL}/get-my-organization`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Cookie': request.headers.get('cookie') || ''
            },
            credentials: 'include'
        });

        if (response.ok) {
            const result = await response.json();
            if (result.success && result.organisation) {
                return {
                    organisation: result.organisation
                };
            }
        }
    } catch {}

    return {
        organisation: null
    };
};


export const actions = {
    supprimer: async ({ request, fetch, cookies }) => {
        try {
            const response = await fetch(`${BACKEND_URL}/delete_account`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
            });

            const result = await response.json();

            if (result.success) {
                throw redirect(303, '/logout?account_deleted=true');
            } else {
                return fail(400, { message: result.message || 'Erreur lors de la suppression' });
            }
        } catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { message: 'Erreur serveur' });
        }
    },
    modifier: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const prenom = data.get('prenom')?.toString();
        const nom = data.get('nom')?.toString();
        const email = data.get('email')?.toString();
        const password = data.get('password')?.toString();

        if (!prenom || !nom || !email) {
            return fail(400, { actionType: 'update_info', message: "Prénom, nom et email requis" });
        }

        const payload: Record<string, string> = { prenom, nom, email };

        if (password && password.length > 0) {
            if (password.length < 8) {
                return fail(400, { actionType: 'update_info', message: "Le mot de passe doit contenir au moins 8 caractères" });
            }
            payload.password = password;
        }

        try {
            const url = `${BACKEND_URL}/update_account`;
            const res = await fetch(url, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify(payload)
            });

            if (!res.ok) {
                const errorText = await res.text();
                return fail(res.status, {
                    actionType: 'update_info',
                    message: `Erreur ${res.status}: ${errorText || 'Erreur lors de la communication avec le serveur'}`
                });
            }

            const result = await res.json();

            if (result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }

                try {
                    const sessionValue = (result.token ?? result.session ?? result.sessionValue) as string | undefined | null;
                    if (sessionValue) {
                        const fakeRes = new Response(null, {
                            headers: { 'set-cookie': `greenscoreweb_sessions=${sessionValue}; Path=/; HttpOnly; SameSite=Lax; Max-Age=3600` }
                        });
                        setSessionCookie(cookies, fakeRes);
                    } else {
                        setSessionCookie(cookies, res);
                    }
                } catch (cookieError) {}
                return {
                    actionType: 'update_info',
                    success: true,
                    message: 'Vos informations ont été mises à jour avec succès'
                };
            }
            return fail(400, { actionType: 'update_info', message: result.message || 'Erreur lors de la mise à jour' });

        } catch (err) {
            return fail(500, {
                actionType: 'update_info',
                message: `Erreur serveur: ${err instanceof Error ? err.message : 'Erreur inconnue'}`
            });
        }
    },

    join_orga: async({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { actionType: 'join_orga', message: "Le code de l'organisation est requis" });
        }

        try {
            const payload = { code: codeOrganisation };

            const response = await fetch(`${BACKEND_URL}/join_organization`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify(payload)
            });

            if (!response.ok) {
                const errorText = await response.text();
                let errorMessage = errorText;
                try {
                    const jsonError = JSON.parse(errorText);
                    errorMessage = jsonError.message || errorText;
                } catch {}

                return fail(response.status, {
                    actionType: 'join_orga',
                    message: errorMessage || "Code invalide"
                });
            }

            const result = await response.json();

            if (result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }

                try {
                    const sessionValue = (result.token ?? result.session ?? result.sessionValue) as string | undefined | null;
                    if (sessionValue) {
                        const fakeRes = new Response(null, {
                            headers: { 'set-cookie': `greenscoreweb_sessions=${sessionValue}; Path=/; HttpOnly; SameSite=Lax; Max-Age=3600` }
                        });
                        setSessionCookie(cookies, fakeRes);
                    } else {
                        setSessionCookie(cookies, response);
                    }
                } catch (cookieError) {}

                return {
                    actionType: 'join_orga',
                    success: true,
                    message: "Vous avez rejoint l'organisation avec succès."
                };
            } else {
                return fail(400, { actionType: 'join_orga', message: result.message || "Erreur lors de l'opération" });
            }

        } catch (error) {
            return fail(500, { actionType: 'join_orga', message: "Erreur serveur lors de la connexion à l'organisation" });
        }
    },

    leave_orga: async({ request, fetch, cookies }) => {
        try {
            const response = await fetch(`${BACKEND_URL}/leave_organization`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                }
            });

            if (!response.ok) {
                const errorText = await response.text();
                return fail(response.status, {
                    actionType: 'leave_orga',
                    message: `Erreur: ${errorText}`
                });
            }

            const result = await response.json();

            if (result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }

                try {
                    setSessionCookie(cookies, response);
                } catch (cookieError) {}

                return {
                    actionType: 'leave_orga',
                    success: true,
                    message: "Vous avez quitté l'organisation."
                };
            } else {
                return fail(400, { actionType: 'leave_orga', message: result.message || "Erreur" });
            }

        } catch (error) {
            return fail(500, { actionType: 'leave_orga', message: "Erreur serveur" });
        }
    },

    change_orga: async({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { actionType: 'change_orga', message: "Le code de l'organisation est requis" });
        }

        try {
            const payload = { code: codeOrganisation };

            const response = await fetch(`${BACKEND_URL}/join_organization`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify(payload)
            });

            if (!response.ok) {
                const errorText = await response.text();
                let errorMessage = errorText;
                try {
                    const jsonError = JSON.parse(errorText);
                    errorMessage = jsonError.message || errorText;
                } catch {}

                return fail(response.status, {
                    actionType: 'change_orga',
                    message: errorMessage || "Code invalide"
                });
            }

            const result = await response.json();

            if (result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }

                try {
                    const sessionValue = (result.token ?? result.session ?? result.sessionValue) as string | undefined | null;
                    if (sessionValue) {
                        const fakeRes = new Response(null, {
                            headers: { 'set-cookie': `greenscoreweb_sessions=${sessionValue}; Path=/; HttpOnly; SameSite=Lax; Max-Age=3600` }
                        });
                        setSessionCookie(cookies, fakeRes);
                    } else {
                        setSessionCookie(cookies, response);
                    }
                } catch (cookieError) {}

                return {
                    actionType: 'change_orga',
                    success: true,
                    message: "Vous avez changé d'organisation avec succès."
                };
            } else {
                return fail(400, { actionType: 'change_orga', message: result.message || "Erreur lors de l'opération" });
            }

        } catch (error) {
            return fail(500, { actionType: 'change_orga', message: "Erreur serveur lors du changement d'organisation" });
        }
    }

} satisfies Actions;
