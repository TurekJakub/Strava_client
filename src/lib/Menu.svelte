<script lang="ts">
	import DailyMenu from '$lib/DailyMenu.svelte';
	export let menuData: MenuData;
</script>

<div class="flex flex-col items-center w-full" id="menu">
	{#each menuData[0] as date}	
		<DailyMenu {date} menu={ menuData[1][date] || new Map()} />
	{/each}
</div>
