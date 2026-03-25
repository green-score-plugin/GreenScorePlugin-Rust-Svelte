import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BACKEND_URL } from '$lib/config';
import { setSessionCookie, invalidateCache } from '$lib/server/session.ts';

export const load: PageServerLoad = async ({ fetch, request, locals, url }) => {
    const headers = {
        'Content-Type': 'application/json',
        'Cookie': request.headers.get('cookie') || ''
    };

    let members = [];
    let organisation = null;
    let accountEquivalents = [];

    const orgIdParam = url.searchParams.get('orgId');
    let targetOrgId = orgIdParam ? parseInt(orgIdParam) : null;

    const userOrgs = locals.user?.organisation;

    if (!targetOrgId && userOrgs && userOrgs.length > 0) {
        targetOrgId = userOrgs[0].id;
    }

    const targetOrg = userOrgs?.find(o => o.id === targetOrgId);

    if (targetOrg) {
        if (targetOrg.est_admin) {
            const res = await fetch(`${BACKEND_URL}/account/organization/members`, {
                method: 'POST',
                headers,
                body: JSON.stringify({ organisation_id: targetOrgId })
            });
            if (res.ok) {
                try {
                    const data = await res.json();
                    if (data.success) members = data.members;
                } catch {}
            }
        }
        organisation = targetOrg;
    }

    const equivRes = await fetch(`${BACKEND_URL}/account/equivalents`, { method: 'GET', headers, credentials: 'include' });
    if (equivRes.ok) {
        try {
            const result = await equivRes.json();
            if (result.success && result.equivalents) {
                accountEquivalents = result.equivalents;
            }
        } catch {}
    }

    return { members, organisation, accountEquivalents };
};

export const actions = {
    supprimer: async ({ request, fetch }) => {
        try {
            const response = await fetch(`${BACKEND_URL}/account/delete`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                }
            });

            const result = await response.json();

            if (result.success) {
                redirect(303, '/logout?account_deleted=true');
            }

            return fail(400, { message: result.message ?? 'errors.delete_error' });
        } catch {
            return fail(500, { message: 'errors.server_error' });
        }
    },

    supprimer_membre: async ({ request, fetch }) => {
        const data = await request.formData();
        const memberId = parseInt(data.get('deleteMemberId')?.toString() || '0', 10);
        const organisationId = parseInt(data.get('organisationId')?.toString() || '0', 10);

        try {
            const response = await fetch(`${BACKEND_URL}/account/organization/members/remove`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                body: JSON.stringify({ userId: memberId, organisationId })
            });

            const result = await response.json();

            if (result.success) {
                return { success: true, message: 'success.member_deleted' };
            }

            return fail(400, { message: result.message ?? 'errors.member_delete_error' });
        } catch {
            return fail(500, { message: 'errors.server_error' });
        }
    },

    modifier: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const prenom = data.get('prenom')?.toString();
        const nom = data.get('nom')?.toString();
        const email = data.get('email')?.toString();
        const password = data.get('password')?.toString();

        if (!prenom || !nom || !email) {
            return fail(400, { actionType: 'update_info', message: 'errors.validation_fields_required' });
        }

        const payload: Record<string, string> = { prenom, nom, email };

        if (password?.trim()) {
            const passwordRegex = /^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$/;
            if (!passwordRegex.test(password)) {
                return fail(400, {
                    actionType: 'update_info',
                    message: 'errors.validation_password_complexity'
                });
            }
            payload.password = password;
        }

        try {
            const res = await fetch(`${BACKEND_URL}/account/update`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify(payload)
            });

            if (!res.ok) {
                return fail(res.status, {
                    actionType: 'update_info',
                    message: 'errors.communication_error',
                    params: { details: await res.text() }
                });
            }

            const result = await res.json();

            if (result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }

                try {
                    const sessionValue = (result.token ?? result.session ?? result.sessionValue) as
                        | string
                        | undefined
                        | null;

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
            } else {
                return fail(400, {
                    actionType: 'update_info',
                    message: result.message ?? 'errors.update_error'
                });
            }
        } catch (err) {
            return fail(500, {
                actionType: 'update_info',
                message: 'errors.server_error',
                params: { details: err instanceof Error ? err.message : 'errors.unknown_error' }
            });
        }
    },

    join_orga: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { actionType: 'join_orga', message: "errors.validation_code_required" });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/account/join-organization`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify({ code: codeOrganisation })
            });

            if (!response.ok) {
                return fail(response.status, {
                    actionType: 'join_orga',
                    message: 'errors.validation_code_invalid'
                });
            }

            const result = await response.json();

            if (result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }

                const sessionValue = result.token ?? result.session ?? result.sessionValue;

                if (sessionValue) {
                    await setSessionCookie(cookies, sessionValue);
                }

                if (result.organisation_id) {
                     redirect(303, `?tab=organisation&orgId=${result.organisation_id}`);
                }

                return {
                    actionType: 'join_orga',
                    success: true,
                    message: 'success.join_organization'
                };
            } else {
                return fail(400, { actionType: 'join_orga', message: result.message ?? 'errors.operation_error' });
            }
        } catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { actionType: 'join_orga', message: 'errors.org_connection_error' });
        }
    },

    modifier_orga: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const organisationName = data.get('organisationName')?.toString();
        const siret = data.get('siret')?.toString();
        const id = data.get('id') ? parseInt(data.get('id')!.toString()) : null;

        if (!organisationName) {
            return fail(400, { actionType: 'update_orga', message: 'errors.validation_org_name_required' });
        }

        if (siret && !/^\d{14}$/.test(siret)) {
            return fail(400, { actionType: 'update_orga', message: 'errors.validation_siret_format' });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/account/organization/update`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify({ id, name: organisationName, siret })
            });

            if (!response.ok) {
                return fail(response.status, { actionType: 'update_orga', message: 'errors.communication_error' });
            }

            const result = await response.json();

            if (!result.success) {
                return fail(400, { actionType: 'update_orga', message: result.message ?? 'errors.update_error' });
            }

            const token = cookies.get('greenscoreweb_sessions');
            if (token) invalidateCache(token);

            const sessionValue = result.token ?? result.session ?? result.sessionValue;
            if (sessionValue) await setSessionCookie(cookies, sessionValue);

            return { actionType: 'update_orga', success: true, message: 'success.org_updated' };
        } catch (err) {
            return fail(500, {
                actionType: 'update_orga',
                message: 'errors.server_error',
                params: { details: err instanceof Error ? err.message : 'errors.unknown_error' }
            });
        }
    },

    leave_orga: async ({ fetch, request, cookies }) => {
        const data = await request.formData();
        const organisationId = parseInt(data.get('organisationId')?.toString() || '0', 10);

        try {
            const response = await fetch(`${BACKEND_URL}/account/leave-organization`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                body: JSON.stringify({ organisationId })
            });


            if (!response.ok) {
                return fail(response.status, { actionType: 'leave_orga', message: 'errors.org_leave_error' });
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

                redirect(303, '?tab=organisation&action=new');
            } else {
                return fail(400, { actionType: 'leave_orga', message: result.message ?? 'errors.operation_error' });
            }
        } catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { actionType: 'leave_orga', message: 'errors.server_error' });
        }
    },

    change_orga: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { actionType: 'change_orga', message: "errors.validation_code_required" });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/account/join-organization`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify({ code: codeOrganisation })
            });

            if (!response.ok) {
                return fail(response.status, { actionType: 'change_orga', message: 'errors.validation_code_invalid' });
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
                return fail(400, { actionType: 'change_orga', message: result.message ?? 'errors.operation_error' });
            }
        } catch {
            return fail(500, { actionType: 'change_orga', message: 'errors.org_change_error' });
        }
    },

    modification_equivalents: async ({ request, fetch }) => {
        const data = await request.formData();
        const equivalents = data.getAll('equivalents').map(v => v.toString());

        try {
            const response = await fetch(`${BACKEND_URL}/account/equivalents`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify({ equivalents })
            });

            const result = await response.json();

            if (!result.success) {
                return fail(400, { actionType: 'modification_equivalents', message: 'errors.update_error' });
            }

            return { actionType: 'modification_equivalents', success: true, message: 'account.equivalent.message.success' };
        } catch {
            return fail(500, { actionType: 'modification_equivalents', message: 'errors.server_error' });
        }
    },

    create_organization: async ({ request, fetch, cookies })=> {        const data = await request.formData();
        const organisationName = data.get('organisationName');
        const siret = data.get('siret')?.toString();

        if(!organisationName) {
            return fail(400, { message: "errors.champs" })
        }

        if (siret && !/^\d{14}$/.test(siret)) {
            return fail(400, { message: 'errors.validation_siret_format' });
        }

        try{
            let body = JSON.stringify({
                organization_name: organisationName,
                siret : siret || null,
            });
            const response = await fetch(`${BACKEND_URL}/account/organization/create`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: body
            });

            const result = await response.clone().json();


            if(result.success) {
                const currentToken = cookies.get('greenscoreweb_sessions');
                if (currentToken) {
                    invalidateCache(currentToken);
                }


                const sessionValue = result.token ?? result.session ?? result.sessionValue;
                if (sessionValue) {
                    await setSessionCookie(cookies, sessionValue);
                } else {
                    await setSessionCookie(cookies, response);
                }
                
                const code = result.account?.code || result.user_full?.organisation?.[0]?.code;
                redirect(303,`/inscription-organisation/${code}`);
            }

            return fail(400, { message: result.message || 'Erreur de connexion' });
        }catch (error) {
            if (error && typeof error === 'object' && ('status' in error || 'location' in error)) {
                throw error;
            }
            return fail(500, { message: 'errors.server_error' });
        }
    }
} satisfies Actions;
