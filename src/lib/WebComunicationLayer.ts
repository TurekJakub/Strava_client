import { goto } from '$app/navigation';
import { json } from '@sveltejs/kit';
import { Card } from 'flowbite-svelte';

const login = async (
	username: string,
	value: string,
	cantine: number,
	stayLogged: boolean
): Promise<Result<string,string>> => {
	const user = {
		jmeno: username,
		heslo: value,
		cislo: cantine,
		zustatPrihlasen: stayLogged,
		lang: 'CZ'
	};

	let res = await fetch('http://localhost:8080/login', {
		method: 'POST',
		credentials: 'include',
		headers: {
			//  'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		},
		body: JSON.stringify(user)
	})
    if (res.status === 200) {
        let c =  await res.json();
        return { _t: "success", data: (c as LoginResponse).user };
    }
    else {
        let c =  await res.json();
        return { _t: "failure", error: (c as ErrorResponse).message };
    }
		
};
const getUserMenu = async (): Promise<Result<Menu,string>> => {
	let res = await fetch('http://localhost:8080/user_menu', {
		method: 'GET',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		}
	})
    if (res.status === 200) {
        let menu =  await res.json();
        return { _t: "success", data: (menu as MenuResponse).menu};
    }else {
        let err =  await res.json();
        return { _t: "failure", error: (err as ErrorResponse).message };
    }    
    
}
const orderDish = async (dishId: string, status: boolean): Promise<Result<void,string>> => {
	let res = await fetch('http://localhost:8080/order_dish', {
		method: 'POST',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		},
		body: JSON.stringify({ id: dishId, status: status })
	})
	if (res.status !== 200) {
		let c =  await res.json();
		return { _t: "failure", error: (c as ErrorResponse).message };
	}
	return { _t: "success", data: undefined };
}
export { login, getUserMenu, orderDish };
