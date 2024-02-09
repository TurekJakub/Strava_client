<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<button on:click|preventDefault={click}>Click me</button>
<script lang="ts">
    import {goto} from '$app/navigation';
    function click() {
        goto('/objednavky');
    }
</script>