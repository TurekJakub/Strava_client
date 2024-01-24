<script lang="ts">
	import { slide } from 'svelte/transition';
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let expanded: boolean = false;
	let username: string = "Nepřihlášen";
	async function saveOrders() {
		await invoke('save_orders');
	}
	async function logout() {
		await invoke('logout');
	}
</script>

<nav
	class="dark:bg-slate-800 w-full flex flex-row align-middle justify-center"
	style="height: 50px;"
>
	<div class="md:w-3/4 w-full flex flex-row justify-start">
		<h1 class="dark:text-white text-2xl text-center mt-auto mb-auto ms-2">Strava-klient</h1>
		<a
			class="dark:text-white text-center mt-auto mb-auto ms-16"
			style="display: block;"
			href="/objednavky">Objednávky</a
		>
		<a class="dark:text-white text-center mt-auto mb-auto ms-2 me-auto" href="/">Nastavení</a>

		<button class="dark:text-white me-2 ms-auto text-center bg-slate-900 rounded" id="user_button"
			>{username}</button
		>
		<Dropdown class="bg-slate-800 rounded-md" triggeredBy="#user_button">
			<DropdownItem><button on:click={logout}>Odhlásit</button></DropdownItem>
		</Dropdown>
	</div>
</nav>
