<script lang="ts">
	import Menu from '$lib/Menu.svelte';
	import Navbar from '$lib/Navbar.svelte';
	import Error from '$lib/Error.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {getUserMenu} from '$lib/WebComunicationLayer';

	export let menuData: MenuData = {}
	let days: string[] = [];
	onMount(async () => {
		// TODO: move data fetching to login component
	   let data = await getUserMenu();
	   switch (data._t) {
		   case 'success':
			   menuData = data.data;
			   days = Object.keys(menuData);
			   console.log(menuData);
			   break;
		   case 'failure':
			   goto('/login');
			   break;
	   }
	});
</script>

<Navbar/>

{#key menuData}
	<Menu  menuData={menuData } days={days} />
{/key}


