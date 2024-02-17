<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';
	import Settings from '$lib/Settings.svelte';
	import { onMount } from 'svelte';
	import { cantine } from '$lib/store';
	import { queryCantineHistory } from '$lib/WebComunicationLayer';
	let settings: boolean = false;
	let allergeens: string[] = [
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
	let results: Dish[] = [];
	async function queryHistory(e: Event) {
		let res = await queryCantineHistory($cantine, (e.target as HTMLInputElement).value);
		switch (res._t) {
			case 'success':
				results = res.data;
				break;
			case 'failure':
				console.log(res.error);
				break;
		}
	}
	onMount(async () => {
		let res = await queryCantineHistory($cantine, '');
		switch (res._t) {
			case 'success':
				results = res.data;
				break;
			case 'failure':
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
					{#each allergeens as allergen}
						<div class="2xl:w-1/5 xl:w-1/4 w-1/2">
							<input
								type="checkbox"
								id={'alergen_' + allergeens.indexOf(allergen).toString(10)}
								value={(allergeens.indexOf(allergen) + 1).toString(10)}
							/>
							<label class="text-white" for={'alergen_' + allergeens.indexOf(allergen).toString(10)}
								>{allergen}</label
							>
						</div>
					{/each}
				</div>
				<h2 class="ms-2 text-white text-lg mb-2">Pokrmy</h2>
				<div class="flex flex-1 flex-row h-96 mb-2">
					<div
						class="w-1/2 rounded-md h-96 overflow-y-scroll bg-slate-800 border-2 border-white ms-2 me-8 mb-2 px-3 pb-3"
					>
						<div class="sticky top-0 bg-slate-800 py-3">
							<div
								class="border-separate border-2 border-white flex flex-row w-full rounded-full mx-auto p-2 h-10 text-center sticky top-0"
							>
								<input
									class="dark:dark-mode-autofill p-1 bg-slate-800 text-white border-none w-full focus-within:border-none focus-within:ring-0 focus-within:outline-none"
									type="text"
									on:input={queryHistory}
								/>
								<svg
									class="text-gray-800 dark:text-white block h-8 w-8 me-2 my-auto"
									aria-hidden="true"
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
									style="height: 20px; width: 20px;"
								>
									<path
										class="my-auto block"
										stroke="currentColor"
										stroke-linecap="round"
										stroke-width="2"
										d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"
									/>
								</svg>
							</div>
						</div>
						{#key results}
							{#each results as result}
								<div class="flex flex-row border-2 border-white rounded-md p-2 mt-2">
									<p class="text-white">{result.name}</p>
								</div>
							{/each}
						{/key}
					</div>

					<!-- <Settings addClass="border-2 border-white ms-8 me-2 " /> -->
				</div>
			</div>
		{/if}
	{/key}
</div>
