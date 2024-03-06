<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';
	import { onMount } from 'svelte';
	import { cantine } from '$lib/store';
	import { queryCantineHistory, querySettings, fetchSettings } from '$lib/WebComunicationLayer';
	import { goto } from '$app/navigation';
	import Alert from '$lib/Alert.svelte';
	import BlackListMenu from '$lib/BlackListMenu.svelte';
	let error: string = '';
	let settings: boolean = false;
	let historyResults: MenuDish[] = [];
	let settingsResults: MenuDish[] = [];
	let allergens: string[] = [
		'Lepek',
		'Korýši',
		'Vejce',
		'Ryby',
		'Arašídy',
		'Sójové boby',
		'Mléko',
		'Skořápkové plody',
		'Celer',
		'Hořčice',
		'Sezamová semena',
		'Oxid siřičitý a siřičitany',
		'Vlčí bob',
		'Měkkýši'
	];
	
	let allergensGroup: string[] = [];
	async function queryHistory(e: Event) {
		let res = await queryCantineHistory($cantine, (e.target as HTMLInputElement).value);
		switch (res._t) {
			case 'success':
				historyResults = res.data;
				break;
			case 'failure':
				console.log(res.error);
				break;
		}
	}
	onMount(async () => {
		let settingsRes = await fetchSettings();
		switch (settingsRes._t) {
			case 'success':
				settingsResults = settingsRes.data.whitelistedDishes;
				allergensGroup = settingsRes.data.blacklistedAllergens;
				break;
			case 'failure':
				error = settingsRes.error;
				break;
			case 'unauthorized':
				goto('/');
				break;
		}
		let historyRes = await queryCantineHistory($cantine, '');
		switch (historyRes._t) {
			case 'success':
				historyResults = historyRes.data;
				break;
			case 'failure':
				error = historyRes.error;
				break;
		}
	});
</script>

<Navbar />
<!--<nav class="block md:hidden z-50 sticky top-0 bg-slate-800 h-6 w-full border-1 border-white"></nav>-->
<div class="w-full md:w-3/4 flex flex-row justify-center py-2 mx-auto">
	<div
		id="navigation"
		class="ms-2 hidden w-52 rounded-md h-28 bg-slate-800 me-8 md:flex flex-col justify-start"
	>
		<button
			class="text-white text-start mt-2 mx-2"
			on:click={() => {
				settings = true;
			}}>Nastavení</button
		>
		<button
			class="text-white text-start mt-2 mx-2"
			on:click={() => {
				settings = false;
			}}>Automatického odhlašování</button
		>
	</div>
	{#key settings}
		{#if settings}
			<div class="rounded-md h-full bg-slate-800" style="width: calc(100% + 64px);"></div>
		{:else}
			<div class="rounded-md h-full bg-slate-800" style="width: calc(100% + 64px);">
				<h2 class="ms-2 text-white text-lg">Alergeny</h2>
				<div class="flex flex-col md:flex-row md:flex-wrap p-2">
					{#each allergens as allergen}
						<div class="2xl:w-1/5 xl:w-1/4 w-1/2">
							<input
								type="checkbox"
								id={'alergen_' + allergens.indexOf(allergen).toString(10)}
								value={(allergens.indexOf(allergen) + 1).toString(10)}
								bind:group={allergensGroup}
							/>
							<label class="text-white" for={'alergen_' + allergens.indexOf(allergen).toString(10)}
								>{allergen}</label
							>
						</div>
					{/each}
				</div>
				<h2 class="ms-2 text-white text-lg mb-2">Pokrmy</h2>
				<BlackListMenu bind:sourceList={historyResults} bind:targetList={settingsResults} />
			</div>
		{/if}
	{/key}
	{#key error}
		<Alert message={error} />
	{/key}
</div>
