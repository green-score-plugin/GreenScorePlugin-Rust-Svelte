import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BACKEND_URL } from '$lib/config';
import { setSessionCookie, invalidateCache } from '$lib/server/session.ts';

export const load: PageServerLoad = async ({ fetch, request, locals }) => {
    const headers = {
        'Content-Type': 'application/json',
        'Cookie': request.headers.get('cookie') || ''
    };

    let members = [];
    let organisation = null;
    let accountEquivalents = [];

    if (locals.user?.role === 'organisation') {
        const res = await fetch(`${BACKEND_URL}/get_organisation_members`, { method: 'POST', headers });
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

    const equivRes = await fetch(`${BACKEND_URL}/account/get_account_all_equivalents`, { method: 'GET', headers, credentials: 'include' });
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
            const response = await fetch(`${BACKEND_URL}/delete_account`, {
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
        const memberId = data.get('deleteMemberId');

        try {
            const response = await fetch(`${BACKEND_URL}/remove_organisation_member`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                body: JSON.stringify({ userId: memberId })
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
            const res = await fetch(`${BACKEND_URL}/update_account`, {
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

            if (!result.success) {
                return fail(400, { actionType: 'update_info', message: result.message ?? 'errors.update_error' });
            }

            const token = cookies.get('greenscoreweb_sessions');
            if (token) invalidateCache(token);

            const sessionValue = result.token ?? result.session ?? result.sessionValue;
            if (sessionValue) await setSessionCookie(cookies, sessionValue);

            return { actionType: 'update_info', success: true, message: 'success.info_updated' };
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
            return fail(400, { actionType: 'join_orga', message: 'errors.validation_code_required' });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/join_organization`, {
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

            if (!result.success) {
                return fail(400, { actionType: 'join_orga', message: result.message ?? 'errors.operation_error' });
            }

            const token = cookies.get('greenscoreweb_sessions');
            if (token) invalidateCache(token);

            const sessionValue = result.token ?? result.session ?? result.sessionValue;
            if (sessionValue) await setSessionCookie(cookies, sessionValue);

            return { actionType: 'join_orga', success: true, message: 'success.org_joined' };
        } catch {
            return fail(500, { actionType: 'join_orga', message: 'errors.org_connection_error' });
        }
    },

    modifier_orga: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const organisationName = data.get('organisationName')?.toString();
        const siret = data.get('siret')?.toString();

        if (!organisationName) {
            return fail(400, { actionType: 'update_orga', message: 'errors.validation_org_name_required' });
        }

        if (siret && !/^\d{14}$/.test(siret)) {
            return fail(400, { actionType: 'update_orga', message: 'errors.validation_siret_format' });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/update_organisation`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                },
                credentials: 'include',
                body: JSON.stringify({ name: organisationName, siret })
            });

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
        try {
            const response = await fetch(`${BACKEND_URL}/leave_organization`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Cookie': request.headers.get('cookie') || ''
                }
            });

            if (!response.ok) {
                return fail(response.status, { actionType: 'leave_orga', message: 'errors.org_leave_error' });
            }

            const result = await response.json();

            if (!result.success) {
                return fail(400, { actionType: 'leave_orga', message: result.message ?? 'errors.operation_error' });
            }

            const token = cookies.get('greenscoreweb_sessions');
            if (token) invalidateCache(token);

            await setSessionCookie(cookies, response);

            return { actionType: 'leave_orga', success: true, message: 'success.org_left' };
        } catch {
            return fail(500, { actionType: 'leave_orga', message: 'errors.server_error' });
        }
    },

    change_orga: async ({ request, fetch, cookies }) => {
        const data = await request.formData();
        const codeOrganisation = data.get('codeOrganisation')?.toString().trim();

        if (!codeOrganisation) {
            return fail(400, { actionType: 'change_orga', message: 'errors.validation_code_required' });
        }

        try {
            const response = await fetch(`${BACKEND_URL}/join_organization`, {
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

            if (!result.success) {
                return fail(400, { actionType: 'change_orga', message: result.message ?? 'errors.operation_error' });
            }

            const token = cookies.get('greenscoreweb_sessions');
            if (token) invalidateCache(token);

            const sessionValue = result.token ?? result.session ?? result.sessionValue;
            if (sessionValue) await setSessionCookie(cookies, sessionValue);

            return { actionType: 'change_orga', success: true, message: 'success.org_changed' };
        } catch {
            return fail(500, { actionType: 'change_orga', message: 'errors.org_change_error' });
        }
    },

    modification_equivalents: async ({ request, fetch }) => {
        const data = await request.formData();
        const equivalents = data.getAll('equivalents').map(v => v.toString());

        try {
            const response = await fetch(`${BACKEND_URL}/account/update_account_equivalents`, {
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

            return { actionType: 'modification_equivalents', success: true, message: 'equivalent.message.success' };
        } catch {
            return fail(500, { actionType: 'modification_equivalents', message: 'errors.server_error' });
        }
    }
} satisfies Actions;
