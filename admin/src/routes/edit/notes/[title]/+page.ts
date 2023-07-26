import { PUBLIC_BACKEND_URL } from '$env/static/public';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (async ({ fetch, params}) : Promise<Note> => {
    const res = await fetch(
        PUBLIC_BACKEND_URL + "/data/notes/get/" + params.title, 
        {
            method: "GET",
            mode: "cors",
            credentials: "include",
        }
    );

    return await res.json();
}) satisfies PageLoad;


