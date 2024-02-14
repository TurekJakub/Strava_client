<script lang="ts">
	import Error from './Error.svelte';
	import { login } from '$lib/WebComunicationLayer';
	import { goto } from '$app/navigation';
	let username: string;
	let cantine: number;
	let stayLogged: boolean = false;
	let show_password: boolean = false;
	let message: string = '';
	$: type = show_password ? 'text' : 'password';
	let value: string = '';

	async function submit() {
		let res = await login(username, value, cantine, stayLogged);
		console.log(res);
		switch (res._t) {
			case 'success':
				sessionStorage.setItem('username', res.data.username);
				localStorage.setItem('account', res.data.account.toString(10));
				goto('/objednavky');
				break;
			case 'failure':
				message = res.error;
				break;
		}
	}

	function onPasswordInput(e: Event) {
		value = (e.target as HTMLInputElement).value;
	}

	function showPassword() {
		show_password = !show_password;
	}
</script>

<div class="flex-col">
	<div
		id="menu"
		class="dark:bg-slate-800 h-1/3 px-5 pt-1 rounded-md"
		style="width: 300px; height: 335px;"
	>
		<h2 class="dark:text-white my-3 w-full md:text-4xl text-2xl text-center">Přihlášení</h2>
		<form on:submit|preventDefault={submit} class="bg-slate-800 flex flex-col h-fit">
			<label class="text-white" for="cantine">Číslo jídelny:</label>
			<input
				class="menu-item dark:dark-mode-autofill bg-slate-800 text-white border-2 px-1 border-white rounded-md focus:outline-2 focus:outline-offset-0 focus:outline-violet-700 focus:outline focus:border-none focus:ring-0 focus:resize-none"
				type="text"
				name="cantine"
				id="cantine"
				bind:value={cantine}
				required
			/>
			<label class="text-white mt-2" for="username">Uživatelské jméno:</label>
			<input
				class="menu-item dark:dark-mode-autofill bg-slate-800 text-white border-2 px-1 border-white rounded-md focus:outline-2 focus:outline-offset-0 focus:outline-violet-700 focus:outline focus:border-none focus:ring-0"
				type="text"
				name="username"
				id="username"
				bind:value={username}
				required
			/>
			<label class="text-white mt-2" for="password">Heslo:</label>
			<div
				class=" flex flex-row border-2 border-white rounded-md px-1 focus-within:outline-2 focus-within:outline-violet-700 focus-within:outline focus-within:border-none menu-item focus:outline-none"
			>
				<input
					{type}
					{value}
					class="dark:dark-mode-autofill bg-slate-800 text-white border-none flex-grow focus-within:border-none focus-within:ring-0 focus-within:outline-none"
					name="password"
					id="password"
					on:input={onPasswordInput}
					required
				/>
				<button
					class="text-white me-0 select-none active:shadow-none"
					type="button"
					on:click={showPassword}
					tabindex="-1"
				>
					<svg
						class="w-6 h-6 text-gray-800 dark:text-white"
						aria-hidden="true"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<path
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d={show_password
								? 'M4 14c-.5-.6-.9-1.3-1-2 0-1 4-6 9-6m7.6 3.8A5 5 0 0 1 21 12c0 1-3 6-9 6h-1m-6 1L19 5m-4 7a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z'
								: 'M21 12c0 1.2-4 6-9 6s-9-4.8-9-6c0-1.2 4-6 9-6s9 4.8 9 6Z M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z'}
						/>
					</svg>
				</button>
			</div>
			<div class="flex-row mt-2">
				<input
					class="non-expand focus:border-none focus:ring-0 focus:outline-offset-0 focus:outline-violet-700 rounded-sm"
					type="checkbox"
					name="stay_logged"
					id="stayLogged"
					bind:value={stayLogged}
				/>
				<label class="text-white ms-2" for="stayLoggeda">Zůstat přihlášen</label>
			</div>
			<input
				class="bg-violet-700 mt-5 rounded-md menu-item focus:ring-0 focus:border-none focus:outline-white focus:outline-1 outline-none"
				type="submit"
				value="Přihlásit"
			/>
		</form>
	</div>
	{#if message != ''}
		<Error {message} />
	{/if}
</div>
