import { invoke } from '@tauri-apps/api/tauri';
import { goto } from '$app/navigation';

async function login(username: string, value: string, cantine: number, stayLogged: boolean){
    await invoke('login', {
        username: username,
        password: value,
        cantine: cantine,
        stayLogged: stayLogged
    }).then(
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
export { login };