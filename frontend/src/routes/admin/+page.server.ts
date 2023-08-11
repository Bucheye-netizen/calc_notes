import { PUBLIC_BACKEND_URL } from '$env/static/public';
import { redirect } from '@sveltejs/kit';

export const load = (async ({ fetch }) : Promise<Values<Note>> => {
    const [res, data] = await Promise.all([
        fetch(
            PUBLIC_BACKEND_URL + "/auth/status", 
            {
                method: "GET",
                mode: "cors",
                credentials: "include",
            }
        ),
        await fetch (
            PUBLIC_BACKEND_URL + "/data/notes/get", 
            {
                method: "GET",
                mode: "cors",
                credentials: "include",
            }
        )
    ])

    let [role, notes] = await Promise.all([
        res.json(),
        data.json(),
    ]);

    if (role <= 1) {
        throw redirect(307, "/admin/account/login")
    }

    return { "values": notes };
});

