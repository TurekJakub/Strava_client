<script lang="ts">
	import Menu from '$lib/Menu.svelte';
	import Navbar from '$lib/Navbar.svelte';
	import Error from '$lib/Error.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {getUserMenu} from '$lib/WebComunicationLayer';

	let menuData: MenuData = {}
	let days: string[] = [];
	let account: number  = parseFloat(sessionStorage.getItem('account')||'0');
	onMount(async () => {
	   let data = await getUserMenu();
	   console.log(data);
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

<Navbar bind:accountValue={account} />

{#key menuData}
	<Menu bind:account menuData={menuData } days={days} />
{/key}


