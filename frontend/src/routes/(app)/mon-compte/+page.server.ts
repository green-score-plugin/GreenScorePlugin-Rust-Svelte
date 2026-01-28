import {fail, redirect} from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BACKEND_URL } from '$lib/config';
import {setSessionCookie, invalidateCache} from "$lib/server/session.ts";

export const load: PageServerLoad = async ({ fetch, request, locals }) => {
    const headers = {
        'Content-Type': 'application/json',
        'Cookie': request.headers.get('cookie') || ''
    };

    let members = [];
    let organisation = null;

    if (locals.user?.role === 'organisation') {
        const res = await fetch(`${BACKEND_URL}/get_organisation_members`, { method: "POST", headers });
        if (res.ok) {
            try {
                const data = await res.json();
                if (data.success) members = data.members;
            } catch {}
        }
    } else if (locals.user?.role === 'user') {
        const orgRes = await fetch(`${BACKEND_URL}/get-my-organization`, { method: 'GET', headers, credentials: 'include' });
        if (orgRes.ok) {
            try {
                const result = await orgRes.json();
                if (result.success && result.organisation) {
                    organisation = result.organisation;
                }
            } catch {}
        }
    }

    return {
        members,
        organisation
    };
};


export const actions = {
    supprimer: async ({ request, fetch }) => {
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
                 redirect(303, '/logout?account_deleted=true');
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
    supprimer_membre: async ({ request, fetch }) => {
        const data = await request.formData();
        const memberId = data.get('deleteMemberId');

        try {
            const response = await fetch(`${BACKEND_URL}/remove_organisation_member`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                body: JSON.stringify({ "userId" : memberId })
            });

            const result = await response.json();


            if (result.success) {
                return {
                    success: true,
                    message: 'success.member_deleted'
                };
            }
        } catch {
            return fail(500, { message: 'errors.member_delete_error' });
        }
    },
    modifier: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const prenom = data.get('prenom')?.toString();
        const nom = data.get('nom')?.toString();
        const email = data.get('email')?.toString();
        const password = data.get('password')?.toString();

        if (!prenom || !nom || !email) {
            return fail(400, { actionType: 'update_info', message: "errors.validation_fields_required" });
        }

        const payload: Record<string, string> = { prenom, nom, email };

        if (password && password.trim() !== '') {
             const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$/;
             if (!passwordRegex.test(password)) {
                 return fail(400, {
                     actionType: 'update_info',
                     message: "errors.validation_password_complexity"
                 });
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
                    message: `Erreur ${res.status}: ${errorText || 'errors.communication_error'}`
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
                        await setSessionCookie(cookies, sessionValue);
                    } else {
                        await setSessionCookie(cookies, res);
                    }
                } catch (cookieError) {
                    console.error('Erreur cookie:', cookieError);
                }
                return {
                    actionType: 'update_info',
                    success: true,
                    message: 'success.info_updated'
                };
            }
            return fail(400, { actionType: 'update_info', message: result.message || 'errors.update_error' });

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
            return fail(400, { actionType: 'join_orga', message: "errors.validation_code_required" });
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
                    message: errorMessage || "errors.validation_code_invalid"
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
                        await setSessionCookie(cookies, sessionValue);
                    } else {
                        await setSessionCookie(cookies, response);
                    }
                } catch (cookieError) {
                    console.error('Erreur lors du rafraîchissement du cookie:', cookieError);
                }

                return {
                    actionType: 'join_orga',
                    success: true,
                    message: "success.org_joined"
                };
            }
            if (!result.success) {
                return fail(400, { actionType: 'join_orga', message: result.message || "errors.operation_error" });
            }

        } catch {
            return fail(500, { actionType: 'join_orga', message: "errors.org_connection_error" });
        }
    },

    modifier_orga: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const organisationName = data.get('organisationName')?.toString();
        const siret = data.get('siret')?.toString();

        if (!organisationName) {
            return fail(400, { actionType: 'update_orga', message: "errors.validation_org_name_required" });
        }

        if(siret && siret.length > 0 && !/^\d{14}$/.test(siret)) {
            return fail(400, { actionType: 'update_orga', message: "errors.validation_siret_format" });
        }

        try{
            let result = await fetch(`${BACKEND_URL}/update_organisation`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify({
                    name: organisationName,
                    siret: siret
                })
            });

            const resJson = await result.json();

            if (resJson.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }
                try {
                    const sessionValue = (resJson.token ?? resJson.session ?? resJson.sessionValue) as string | undefined | null;
                    if (sessionValue) {
                        await setSessionCookie(cookies, sessionValue);
                    } else {
                        await setSessionCookie(cookies, result);
                    }

                } catch (cookieError) {
                    console.error('Erreur lors de la mise à jour du cookie:', cookieError);
                }

                return {
                    actionType: 'update_orga',
                    success: true,
                    message: "success.org_updated"
                };
            } else {
                return fail(400, { actionType: 'update_orga', message: resJson.message || 'errors.org_update_error' });
            }

        } catch (err) {
            return fail(500, {
                actionType: 'update_orga',
                message: `Erreur serveur: ${err instanceof Error ? err.message : 'errors.unknown_error'}`
            });
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
                    await setSessionCookie(cookies, response);
                } catch (cookieError) {}

                return {
                    actionType: 'leave_orga',
                    success: true,
                    message: "success.org_left"
                };
            } else {
                return fail(400, { actionType: 'leave_orga', message: result.message || "errors.org_leave_error" });
            }

        } catch {
            return fail(500, { actionType: 'leave_orga', message: "errors.server_error" });
        }
    },
    change_orga: async({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { actionType: 'change_orga', message: "errors.validation_code_required" });
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
                    message: errorMessage || "errors.validation_code_invalid"
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
                        await setSessionCookie(cookies, fakeRes);
                    } else {
                        await setSessionCookie(cookies, response);
                    }
                } catch (cookieError) {}

                return {
                    actionType: 'change_orga',
                    success: true,
                    message: "success.org_changed"
                };
            } else {
                return fail(400, { actionType: 'change_orga', message: result.message || "errors.operation_error" });
            }

        } catch {
            return fail(500, { actionType: 'change_orga', message: "errors.org_change_error" });
        }
    }
} satisfies Actions;