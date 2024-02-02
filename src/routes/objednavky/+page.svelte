<script lang="ts">
	import Menu from '$lib/Menu.svelte';
	import Navbar from '$lib/Navbar.svelte';
	import { onMount } from 'svelte';
	import {getUserMenu} from '$lib/WebComunicationLayer';
	let menuData: MenuData = [[], {}];
	onMount(async () => {
		menuData = await (getUserMenu() as unknown as Promise<MenuData>);
	});
</script>

<Navbar />

{#key menuData}
	<Menu {menuData} />
{/key}
