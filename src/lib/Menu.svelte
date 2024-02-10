<script lang="ts">
	import DailyMenu from '$lib/DailyMenu.svelte';
	export let menuData: Menu;
	export let days: string[];
</script>

<div class="flex flex-col items-center w-full" id="menu">
	{#each days as date}	
		<DailyMenu {date} menu={ menuData[date] || new Map()} />
	{/each}
</div>
