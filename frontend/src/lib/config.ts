import { env } from '$env/dynamic/public';
export const BACKEND_URL = env.PUBLIC_BACKEND_URL ?? 'http://localhost:3000';
