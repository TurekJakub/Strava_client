<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { each } from "svelte/internal";
  import DailyMenu from "./DailyMenu.svelte";
  let menuData = [];  
  onMount(async () => {
    menuData =  await invoke("get_menu_data", {});    
    console.log(menuData['Čtvrtek 31. 8.']);  
  });
</script>

<div id="menu">
  {#each Object.entries(menuData) as [date, dishes]}
    <DailyMenu date={date} dishes={dishes} />
   
  {/each}
</div>
