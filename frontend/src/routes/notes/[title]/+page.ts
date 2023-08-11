import type { PageLoad } from './$types';
import { PUBLIC_BACKEND_URL} from '$env/static/public'

export const load = (async ({ fetch, params }) : Promise<Note> => {
    const response = await fetch(
        PUBLIC_BACKEND_URL + "/data/notes/get/" + params.title,
        {
            method: "GET",
            mode: "cors",
            credentials: "include",
        }
    );
    return await response.json();
}) satisfies PageLoad;
