<script lang="ts">
	import DailyMenu from '$lib/DailyMenu.svelte';
	export let menuData: Map<string, Map<string, DishInfo>>;
	//let keys: string[] = Array.from(menuData.keys());
	console.log(menuData);
	let keys: string[] = []
</script>

<div id="menu">
	{#each  Object.entries(menuData) as [date, menu]}
		<DailyMenu {date} menu={menu || new Map()} />
	{/each}
</div>
