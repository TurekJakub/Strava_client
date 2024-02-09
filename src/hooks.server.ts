import type { Handle } from '@sveltejs/kit';
import { redirect } from "@sveltejs/kit";
const nonrestricted = ["/login", "/"];
export  const handle: Handle = async ({ event, resolve}) => {
    console.log(event.request.url)
    if (isNonRestricted(event.request.url)) {
        return await resolve(event);
    }
    let status = await fetch('http://localhost:8080/user_status', { 
        method: 'GET',
        credentials: "include",
        headers: {
            'Content-Type': 'application/json;charset=UTF-8'
        }
    }).then(
        (response) => {
          return response.status;
        },       
    );
    if(status === 401){
        throw redirect(302 , '/login');               
    }
  return await resolve(event);
}
const isNonRestricted: (url: string) => boolean = (url) => {
    let params = url.split("/");
    return nonrestricted.includes("/"+params[params.length - 1]);
}