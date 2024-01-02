<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	export let date: string;
	export let menu: DailyMenu;
	async function selectDish(e : Event) {
		let name = (e.target as HTMLInputElement).value;
		let keys = Object.keys(menu);
		for (let i = 0; i < keys.length; i++) {
			console.log(keys[i] !== name);
			if (keys[i] !== name) {
				menu[keys[i]].order_state = false;
			} 
		}
		console.log(menu[name].order_state);
		await invoke("order_dish", { dishId:menu[name].id, ordered:menu[name].order_state});
	}
</script>

<div class="bg-slate-800 rounded-md my-5 border-white border-1 md:w-3/4 w-full p-5" id="daily_menu">
	<h2 class="text-white text-2xl">{date}</h2>
	{#each Object.entries(menu) as [name, dish]}
		<div class="flex-row flex mt-2">
			<input
				class="accent-violet-700 me-5"
				style="width: 20px;"
				type="checkbox"
				bind:value={name}
				bind:checked={dish.order_state}
				on:change|preventDefault={selectDish}
			/>
			<div class="bg-slate-800 text-white text-lg">{name}</div>
		</div>
	{/each}
</div>
