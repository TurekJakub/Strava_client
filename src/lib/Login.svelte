<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Error from './Error.svelte';
	import { error } from '@sveltejs/kit';
	import { goto } from '$app/navigation';
	let username: string;
	let cantine: number;
	let stayLogged: boolean;
	let show_password: boolean = false;
	let message: string = '';
	let err: boolean = false;
	$: type = show_password ? 'text' : 'password';
	let value: string = '';
	async function submit(e: Event) {
		await invoke('login', {
			username: username,
			password: value,
			cantine: cantine,
			stayLogged: true
		}).then(
			() => {
				goto('/objednavky');
			},
			(error) => {
				message = error as string;
				err = true;
				console.log(err);
			}
		);
	}
	function onPasswordInput(e: Event) {
		value = (e.target as HTMLInputElement).value;
	}
	function showPassword() {
		show_password = !show_password;
	}
</script>
<div class=flex-col>	
	<div id="menu" class="dark:bg-slate-800 h-1/3 px-5 pt-1 rounded-md" style="width: 300px; height: 335px;">
		<h2 class="dark:text-white my-3 w-full md:text-4xl text-2xl text-center">Přihlášení</h2>
		<form on:submit|preventDefault={submit} class="bg-slate-800 flex flex-col h-fit">
			<label class="text-white" for="cantine">Číslo jídelny:</label>
			<input
			class="dark:dark-mode-autofill bg-slate-800 text-white border-2 px-1 border-white rounded-md focus:outline-2 focus:outline-violet-700 focus:outline focus:border-none"
			type="text"
			name="cantine"
			id="cantine"
			bind:value={cantine}
			required
			/>
			<label class="text-white mt-2" for="username">Uživatelské jméno:</label>
			<input
			class="dark:dark-mode-autofill bg-slate-800 text-white border-2 px-1 border-white rounded-md focus:outline-2 focus:outline-violet-700 focus:outline focus:border-none"
			type="text"
			name="username"
			id="username"
			bind:value={username}
			required
			/>
			<label class="text-white mt-2" for="password">Heslo:</label>
			<div
			class=" flex flex-row border-2 border-white rounded-md px-1 focus-within:outline-2 focus-within:outline-violet-700 focus-within:outline focus-within:border-none"
			>
			<input
			{type}
			{value}
			class="dark:dark-mode-autofill bg-slate-800 text-white flex-grow focus:outline-none"
			name="password"
			id="password"
			on:input={onPasswordInput}
			required
			/>
			<button class="text-white me-0" type="button" on:click={showPassword}
			>{show_password ? 'Hide' : 'Show'}</button
			>
		</div>
		<div class="flex-row mt-2">
			<input type="checkbox" name="stay_logged" id="stayLogged" bind:value={stayLogged} />
			<label class="text-white ms-2" for="stayLoggeda">Zůstat přihlášen</label>
		</div>
		<input
		class="bg-violet-700 mt-5 rounded-md"
		type="submit"
		value="Přihlásit"
		style="height: 30px;"
		/>
	</form>
</div>
{#if err}
<Error {message} />
{/if}
</div>
