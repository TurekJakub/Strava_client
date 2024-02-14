<script>
  import Navbar from '$lib/Navbar.svelte';
  import Settings from '$lib/Settings.svelte';
</script>
<Navbar/>
<div class="w-3/4 h-full flex flex-row justify-center py-2 mx-auto " style="height: calc(100vh - 80px);">
    <Settings addClass="me-8"/>
    <Settings addClass="ms-8"/>
</div>