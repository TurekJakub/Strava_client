import { invoke } from '@tauri-apps/api/tauri';
import { goto } from '$app/navigation';
import { json } from '@sveltejs/kit';

const login = async (username: string, value: string, cantine: number) : Promise<Result<User,string>> =>{
    let res = await invoke('login', {
        username: username,
        password: value,
        cantine: cantine,
    })
    .then(
       (response) => {
        console.log(response);
           return {_t:'succes', data: response as User};
       }
    ).catch(
        (error) => {
             return error as Result<string, string>;
        }
        
    );
    return res; 
    
}
export { login };