<script lang="ts">
	import Menu from '$lib/Menu.svelte';
	import Navbar from '$lib/Navbar.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let menuData: MenuData = [[], {}];
	onMount(async () => {
		await invoke('get_menu_data', {}).then(
			(data) => {
				menuData = data as MenuData;
				console.log((data as [string[], MenuData])[0]);
			},
			(error) => {
				console.log(error);
			}
		);
	});
</script>

<Navbar />

{#key menuData}
	<Menu {menuData} />
{/key}
