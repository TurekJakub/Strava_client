<script lang="ts">
  import { Alert} from 'flowbite-svelte';
  export let message: string = '';
</script>
{#if message != ''}
  <div class="bg-red-300 text-white p-3 rounded-md align-middle justify-start flex flex-row sticky bottom-0 w-3/4">
    <p class="align-middle text-red-700 ms-2">{message}</p>
    <button class="ms-auto me-2 text-red-700" on:click={() => message = ''}>X</button>
  </div>
{/if}