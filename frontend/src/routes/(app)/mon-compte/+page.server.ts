import {fail, redirect} from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/config';
import {setSessionCookie, invalidateCache} from "$lib/server/session.ts";

export const actions = {
    modifier: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const prenom = data.get('prenom')?.toString();
        const nom = data.get('nom')?.toString();
        const email = data.get('email')?.toString();
        const password = data.get('password')?.toString();

        if (!prenom || !nom || !email) {
            return fail(400, { message: "Prénom, nom et email requis" });
        }

        const payload: Record<string, string> = { prenom, nom, email };

        if (password && password.length > 0) {
            if (password.length < 8) {
                return fail(400, { message: "Le mot de passe doit contenir au moins 8 caractères" });
            }
            payload.password = password;
        }

        try {
            const url = `${BACKEND_URL}/update_account`;
            const res = await fetch(url, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/json' },
                credentials: 'include',
                body: JSON.stringify(payload)
            });

            if (!res.ok) {
                const errorText = await res.text();
                return fail(res.status, {
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
                } catch (cookieError) {
                    console.error('Erreur cookie:', cookieError);
                }
                return {
                    success: true,
                    message: 'Vos informations ont été mises à jour avec succès'
                };
            }
            return fail(400, { message: result.message || 'Erreur lors de la mise à jour' });

        } catch (err) {
            return fail(500, {
                message: `Erreur serveur: ${err instanceof Error ? err.message : 'Erreur inconnue'}`
            });
        }
    },

    supprimer: async ({ request, fetch, cookies }) => {
        try {
            const response = await fetch(`${BACKEND_URL}/delete_account`, {
                method: 'DELETE',
                headers: {
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include'
            });

            if (!response.ok) {
                const text = await response.text();
                return fail(response.status, { message: `Erreur lors de la suppression: ${text}` });
            }

            cookies.delete('greenscoreweb_sessions', { path: '/' });

        } catch (error) {
            console.error("Erreur suppression:", error);
            return fail(500, {message: 'Erreur serveur'});
        }

        throw redirect(303, '/logout');
    },

    join_orga: async({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { message: "Le code de l'organisation est requis" });
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
                        // Fallback : Si le token n'est pas dans le JSON, on propage les headers de la réponse (Set-Cookie)
                        setSessionCookie(cookies, response);
                    }
                } catch (cookieError) {
                    console.error('Erreur lors du rafraîchissement du cookie:', cookieError);
                }

                return {
                    success: true,
                    message: "Vous avez rejoint l'organisation avec succès."
                };
            } else {
                return fail(400, { message: result.message || "Erreur lors de l'opération" });
            }

        } catch (error) {
            console.error("Erreur join_orga:", error);
            return fail(500, { message: "Erreur serveur lors de la connexion à l'organisation" });
        }
    }



} satisfies Actions;
