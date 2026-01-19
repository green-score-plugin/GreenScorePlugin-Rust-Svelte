import {BACKEND_URL} from "$lib/config.ts";
import type {RequestHandler} from './$types';


export const fallback: RequestHandler = async ({ params, request }) => {
    const route = params.path;
    const backendUrl = `${BACKEND_URL}/plugin/${route}`;

    const options: RequestInit = {
        method: request.method,
        headers: new Headers(request.headers),
    };


    if (request.method !== 'GET' && request.method !== 'HEAD') {
        options.body = await request.arrayBuffer();
        // @ts-ignore - duplex est nécessaire pour node 18+ mais pas encore dans tous les types
        options.duplex = 'half';
    }

    try {
        return await fetch(backendUrl, options);
    } catch (err) {
        console.error('Erreur Proxy Rust:', err);
        return new Response('Erreur de communication avec le backend', { status: 502 });
    }
};