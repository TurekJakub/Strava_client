import { goto } from '$app/navigation';
import type Menu from './Menu.svelte';

async function login(username: string, value: string, cantine: number, stayLogged: boolean){
    const user  = {"jmeno": username, "heslo": value, "cislo": cantine, "zustatPrihlasen": false,"lang": "CZ"};
    console.log(user);
    fetch('http://localhost:8080/login', {
        method: 'POST',
        credentials: "include",
        headers: {
          //  'csrf-token': 'nocheck',
          
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify(user)
    })
   .then(
        () => {
            goto('/objednavky');
            localStorage.setItem("username", username);
            return null;
        },
        (error: any) => {				
            return error as string;
        }
    );
}
async function getUserMenu(){
    fetch('http://localhost:8080/user_menu', {
        method: 'GET',
        credentials: "include",
        headers: {
           // 'csrf-token': 'nocheck',
            'Content-Type': 'application/json;charset=UTF-8'
        }
    }).then(
        (response) => {
          response.json().then((data) => {
            return JSON.parse(data.MenuData) as Menu;
          })
        },
        (error: any) => {				
            return error as string;
        }

    )
}
export { login , getUserMenu};