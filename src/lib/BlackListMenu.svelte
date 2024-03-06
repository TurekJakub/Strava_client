<script lang="ts">
	import BlackList from './BlackList.svelte';
	import { createEventDispatcher } from 'svelte';

	export let sourceList: MenuDish[] = [];
	export let targetList: MenuDish[] = [];
	let draggedItem: MenuDish;

	const dispatch = createEventDispatcher();

	function onDropToSourceList(e: Event) {
		console.log(e);
		if (draggedItem !== undefined) {
			sourceList = [...sourceList, draggedItem];
			for ( let i = 0; i < targetList.length; i++ ) {
				if (targetList[i].name === draggedItem.name && targetList[i].allergens === draggedItem.allergens) {
					targetList.splice(i, 1);					
				}
			}
			targetList = targetList;
		}
	}
	function onDropToTargetList(e: Event) {
		if (draggedItem !== undefined) {
			targetList = [...targetList, draggedItem];
			for ( let i = 0; i < sourceList.length; i++ ) {
				if (sourceList[i].name === draggedItem.name && sourceList[i].allergens === draggedItem.allergens) {
					sourceList.splice(i, 1);					
				}
			}
			sourceList = sourceList;
		}
	}
</script>

<div class="flex flex-1 flex-row h-96 mb-2">
	<!-- Compare this snippet from src/lib/BlackList.svelte:
		<div
		class="w-1/2 rounded-md h-96 overflow-y-scroll scrollbar-none border-2 border-white ms-2 me-8 mb-2 px-3 pb-3"
		on:drop={onDropToSourceList}
		on:dragover|preventDefault
		role="cell"
		tabindex="-1"
		>
		<div class="sticky top-0 bg-slate-800 py-3">
			<div
			class="border-separate border-2 border-white flex flex-row w-full rounded-full mx-auto p-2 h-10 text-center sticky top-0"
			>
			<input
			class="dark:dark-mode-autofill p-1 bg-slate-800 text-white border-none w-full focus-within:border-none focus-within:ring-0 focus-within:outline-none"
			type="text"
			/>
			<svg
			class="text-gray-800 dark:text-white block h-8 w-8 me-2 my-auto"
			aria-hidden="true"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			style="height: 20px; width: 20px;"
			>
			<path
			class="my-auto block"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			d="m21 21-3.5-3.5M17 10a7 7 0 1 1-14 0 7 7 0 0 1 14 0Z"
			/>
		</svg>
	</div>
</div>

{#each sourceList as result}
<div class="flex flex-row border-2 border-white rounded-md p-2 mt-2">
	<p class="text-white">{result.name}</p>
	<p class="text-white ms-2">{result.allergens}</p>
</div>
{/each}
</div>
-->
	<BlackList
		bind:draggedItem
		bind:list={sourceList}
		on:drop={(e) => {
			onDropToSourceList(e);
			dispatch('drop');
		}}
	/>
	<BlackList
		bind:draggedItem
		bind:list={targetList}
		on:drop={(e) => {
			onDropToTargetList(e);
			dispatch('drop');
		}}
	/>
</div>
