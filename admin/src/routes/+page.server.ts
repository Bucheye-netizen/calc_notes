import { PUBLIC_BACKEND_URL } from '$env/static/public';
import { redirect } from '@sveltejs/kit';

export const load = (async ({ fetch }) => {
    const res = await fetch(
        PUBLIC_BACKEND_URL + "/api/auth/status", 
        {
            method: "GET",
            mode: "cors",
            credentials: "include",
        }
    ); 

    let role: number = await res.json();

    console.log("Current role: " + role);
    if (role != 2) {
        throw redirect(307, "/account/login")
    }
});

