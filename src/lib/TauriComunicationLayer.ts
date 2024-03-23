import { invoke } from '@tauri-apps/api/tauri';
import { goto } from '$app/navigation';

async function login(username: string, value: string, cantine: number){
    let res =await invoke('login', {
        username: username,
        password: value,
        cantine: cantine,
    }).then(
        () => {
            goto('/objednavky');
            localStorage.setItem("username", username);            
        },
       
    );
}
export { login };