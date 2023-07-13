import type { PageLoad } from './$types';


class Note {
    title: string;
    author: string; 
    source: string;
    dependencies: string[] = []; 
}

export const load = (async ({ params }) => {
    console.log("Searching for note " + params.title);
    const response = await fetch(
        "http://localhost:3000",
        {
            method: "GET",
            mode: "same-origin", 
            cache: "default", 
            credentials: "same-origin",
            redirect: "error",
            body: JSON.stringify(
                {
                    "table": "notes",
                    "conds": 
                        [
                            [["title", "=", params.title], ""],
                        ],
                } 
            ),
        }
    );
    let out: Note = await response.json();

    return out;
    
}) satisfies PageLoad;
