import { env } from '$env/dynamic/private';
export const BACKEND_URL = env.BACKEND_URL ?? 'http://localhost:3000';
export const ELECTRICITY_MAP_API_KEY = env.ELECTRICITY_MAP_API_KEY ?? '';
