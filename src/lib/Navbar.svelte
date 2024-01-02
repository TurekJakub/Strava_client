<script lang="ts">
  import { slide } from 'svelte/transition'
  import { invoke } from '@tauri-apps/api/tauri'
  let  expanded: boolean = false
 function expand() {
   expanded = !expanded;
 }
 async function saveOrders() {
   await invoke('save_orders');
 }
</script>
<nav class="dark:bg-slate-800 w-full flex flex-row align-middle" style="height: 50px;">
 <h1 class="dark:text-white  text-2xl text-center">Strava-klient</h1>
 <a class="dark:text-white align-middle h-fit" href="/objednavky">Objednávky</a>
 <a class="dark:text-white" href="/">Nastavení</a>
 <button type="button" class="dark:bg-violet-700 rounded-md" id="menu-button" on:click={saveOrders} style="width: 75px;">Odeslat</button>
 <div class="justify-end ms-auto me-10">
    <button type="button" class="dark:bg-slate-800 dark:text-white" id="menu-button" on:click={expand} style="width: 75px;">
      Options     
    </button>
    {#if expanded}
      <div class="dark:bg-slate-800 mt-5 rounded flex flex-col border-white border dark:text-white" transition:slide style="width: 75px;">
        <a href="/login">Přihlášení</a>
        <a href="/logout">Odhlášení</a>
      </div>
    {/if}
 </div>

 
  
  
</nav>