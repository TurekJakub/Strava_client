<script lang="ts">
	import Menu from '$lib/Menu.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	let menuData: Map<string, Map<string, DishInfo>> = new Map<string, Map<string, DishInfo>>();
	onMount(async () => {
		await invoke('get_menu_data', {}).then(
			(data) => {
				menuData = data as Map<string, Map<string, DishInfo>>;
                console.log(menuData);
			},
			(error) => {
				console.log(error);
			}
		);
	});
</script>


{#key menuData}
<Menu {menuData} />
{/key}
