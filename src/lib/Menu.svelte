<script lang="ts">
	import DailyMenu from '$lib/DailyMenu.svelte';
	import Alert from '$lib/Alert.svelte';
	import { A } from 'flowbite-svelte';

	export let menuData: Menu;
	export let days: string[];
	export let account: number;
	
	let error:string = '';
</script>

<div class="flex flex-col items-center w-full" id="menu">
	{#each days as date}
		<DailyMenu bind:account bind:error {date} menu={menuData[date] || new Map()} />
	{/each}
	{#key error}
		<Alert bind:message={error} />
	{/key}
</div>
