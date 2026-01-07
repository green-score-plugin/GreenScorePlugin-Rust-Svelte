import type { RequestHandler, RequestEvent } from './$types';
import { BACKEND_URL } from '$lib/config';


async function proxyToBackend(path: string | undefined, url: URL, request: Request): Promise<Response> {
    const safePath = path ?? '';
    const targetUrl = `${BACKEND_URL}/${safePath}${url.search}`;

    console.log(`[Proxy] ${request.method} /api/${safePath} -> ${targetUrl}`);

    const headers: Record<string, string> = {};
    request.headers.forEach((value, key) => {
        if (key.toLowerCase() !== 'host' && key.toLowerCase() !== 'connection') {
            headers[key] = value;
        }
    });

    try {
        const response = await fetch(targetUrl, {
            method: request.method,
            headers,
            body: ['GET', 'HEAD'].includes(request.method)
                ? undefined
                : await request.arrayBuffer()
        });

        const responseHeaders: Record<string, string> = {};
        response.headers.forEach((value, key) => {
            responseHeaders[key] = value;
        });

        return new Response(response.body, {
            status: response.status,
            statusText: response.statusText,
            headers: responseHeaders
        });

    } catch (error: any) {
        console.error('[Proxy] Error:', error);
        return new Response(JSON.stringify({
            error: 'Backend unavailable',
            details: error instanceof Error ? error.message : String(error)
        }), {
            status: 502,
            headers: { 'Content-Type': 'application/json' }
        });
    }
}


const handleRequest: RequestHandler = ({ params, url, request }: RequestEvent) => {
    return proxyToBackend(params.path, url, request);
};

export const GET: RequestHandler = handleRequest;
export const POST: RequestHandler = handleRequest;
export const PUT: RequestHandler = handleRequest;
export const PATCH: RequestHandler = handleRequest;
export const DELETE: RequestHandler = handleRequest;