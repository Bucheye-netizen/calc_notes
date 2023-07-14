import type { PageLoad } from './$types';
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

class Note {
    title: string;
    author: string; 
    source: string;
    dependencies: string[] = []; 
}

export const load = (async ({ params }) => {
    console.log("Searching for note " + params.title);
    const response = await fetch(
        BACKEND_URL + "/api/data/notes/get/" + params.title,
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
