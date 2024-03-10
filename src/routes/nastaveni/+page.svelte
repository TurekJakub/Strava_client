<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';
	import { onMount } from 'svelte';
	import { cantine } from '$lib/store';
	import { queryCantineHistory, fetchSettings, querySettings } from '$lib/WebComunicationLayer';
	import { goto } from '$app/navigation';
	import Alert from '$lib/Alert.svelte';
	import BlackListMenu from '$lib/BlackListMenu.svelte';

	let error: string = '';
	let blackListSource: Dish[] = [];
	let blackListTarget: Dish[] = [];
	let whiteListSource: Dish[] = [];
	let whiteListTarget: Dish[] = [];
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
	async function handleWhiteListSoureceQuery(e: CustomEvent) {
		let res = await queryCantineHistory($cantine, e.detail.detail);
		switch (res._t) {
			case 'success':
				whiteListSource = res.data;
				break;
			case 'failure':
				console.log(res.error);
				break;
		}
	}
	async function handleBlackListSoureceQuery(e: CustomEvent) {
		let res = await queryCantineHistory($cantine, e.detail.detail);
		switch (res._t) {
			case 'success':
				blackListSource = res.data;
				break;
			case 'failure':
				console.log(res.error);
				break;
		}
	}
	async function handleQueryBlackList(e: CustomEvent) {
		let res = await querySettings(e.detail.detail, 'blacklist');
		switch (res._t) {
			case 'success':
				blackListTarget = res.data;
				break;
			case 'failure':
				console.log(res.error);
				break;
		}
	}
	async function handleQueryWhiteList(e: CustomEvent) {
		let res = await querySettings(e.detail.detail, 'whitelist');
		switch (res._t) {
			case 'success':
				whiteListTarget = res.data;
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
				console.log(settingsRes.data);
				blackListTarget = settingsRes.data.blacklistedDishes;
				whiteListTarget = settingsRes.data.whitelistedDishes;
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
				blackListSource = historyRes.data;
				whiteListSource = historyRes.data;
				break;
			case 'failure':
				error = historyRes.error;
				break;
		}
	});
</script>

<Navbar />
<div class="w-full md:w-3/4 flex flex-row justify-center py-2 mx-auto">
	<div class="rounded-md h-full bg-slate-800" style="width: calc(100%);">
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
		<h2 class="ms-2 text-white text-lg mb-2">Automaticky odhlašované pokrmy</h2>
		<BlackListMenu
			bind:sourceList={blackListSource}
			bind:targetList={blackListTarget}
			on:querySource={handleBlackListSoureceQuery}
			on:queryTarget={handleQueryBlackList}
		/>
		<h3 class="ms-2  text-white text-lg mb-2">Preferované pokrmy</h3>
		<BlackListMenu
			bind:sourceList={whiteListSource}
			bind:targetList={whiteListTarget}
			on:querySource={handleWhiteListSoureceQuery}
			on:queryTarget={handleQueryWhiteList}
		/>
	</div>
	{#key error}
		<Alert message={error} />
	{/key}
</div>
