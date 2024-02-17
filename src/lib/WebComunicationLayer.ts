import { goto } from '$app/navigation';
import { json } from '@sveltejs/kit';
import { Card } from 'flowbite-svelte';

const login = async (
	username: string,
	value: string,
	cantine: number,
	stayLogged: boolean
): Promise<Result<User, string>> => {
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
	});
	if (res.status === 200) {
		let data = await res.json();
		return { _t: 'success', data: (data as LoginResponse).user };
	} else {
		let error = await res.json();
		return { _t: 'failure', error: (error as ErrorResponse).message };
	}
};
const getUserMenu = async (): Promise<Result<Menu, string>> => {
	let res = await fetch('http://localhost:8080/user_menu', {
		method: 'GET',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		}
	});
	if (res.status === 200) {
		let menu = await res.json();
		return { _t: 'success', data: (menu as MenuResponse).menu };
	} else {
		let err = await res.json();
		return { _t: 'failure', error: (err as ErrorResponse).message };
	}
};
const orderDish = async (dishId: string, status: boolean): Promise<Result<number, string>> => {
	let res = await fetch('http://localhost:8080/order_dish', {
		method: 'POST',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		},
		body: JSON.stringify({ id: dishId, status: status })
	});
	if (res.status !== 200) {
		let error = await res.json();
		return { _t: 'failure', error: (error as ErrorResponse).message };
	}
	let account = ((await res.json()) as OrderDishResponse).account;
	return { _t: 'success', data: account };
};
const saveOrder = async (): Promise<Result<void, SaveFailureResponse>> => {
	let res = await fetch('http://localhost:8080/save_orders', {
		method: 'POST',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		}
	});
	if (res.status !== 200) {
		let error = await res.json();
		return { _t: 'failure', error: error as SaveFailureResponse };
	}
	return { _t: 'success', data: undefined };
};
const logout = async (): Promise<void> => {
	await fetch('http://localhost:8080/logout', {
		method: 'POST',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck',
			'Content-Type': 'application/json;charset=UTF-8'
		}
	});
};
const queryCantineHistory = async (
	cantineId: string,
	query: string
): Promise<Result<[Dish], string>> => {
	let url = `http://localhost:8080/cantine_history?cantine_id=${encodeURIComponent(
		cantineId
	)}&query=${encodeURIComponent(query)}`;
	let res = await fetch(url, {
		method: 'GET',
		credentials: 'include',
		headers: {
			// 'csrf-token': 'nocheck
		}
	});
	if (res.status !== 200) {
		let error = await res.json();
		return { _t: 'failure', error: (error as ErrorResponse).message };
	}
	type Response = { result: [Dish]};
	let data = await res.json();
	return { _t: 'success', data: (data as Response).result };
};
export { login, getUserMenu, orderDish, saveOrder, logout, queryCantineHistory };
