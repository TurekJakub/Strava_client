<script lang="ts">
	import Navbar from '$lib/Navbar.svelte';
	import Settings from '$lib/Settings.svelte';
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
</script>

<Navbar />
<div class="block md:hidden sticky top-0 bg-slate-800 h-6 w-full z-50 border-1 border-white"></div>
<div
	class="w-full md:w-3/4 h-full flex flex-row justify-center py-2 mx-auto"
	style="height: calc(100vh - 128px);"
>
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
			<div class="me-2 rounded-md h-full bg-slate-800" style="width: calc(100% + 64px);">
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
				<div class="flex flex-1 flex-row h-1/2">
					<Settings addClass="border-2 border-white ms-2 me-8" />
          <Settings addClass="border-2 border-white ms-8 me-2 " />
				</div>
			</div>
		{/if}
	{/key}
</div>
