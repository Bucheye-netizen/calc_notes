import type { PageLoad } from './$types';
import { PUBLIC_BACKEND_URL} from '$env/static/public'


class Note {
    title: string;
    author: string; 
    source: string;
    dependencies: string[] = []; 
}

export const load = (async ({ params }) : Promise<Note> => {
    console.log("Searching for note " + params.title);
    const response = await fetch(
        PUBLIC_BACKEND_URL + "/api/data/notes/get/" + params.title,
        {
            method: "GET",
            mode: 'cors'
        }
    );

    if (!response.ok) { 
        throw response.status
    }

    return await response.json();
}) satisfies PageLoad;
