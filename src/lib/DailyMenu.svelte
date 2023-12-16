<script lang="ts">
  export let date: string;
  export let menu: Map<string, DishInfo>;

</script>

<div id="daily_menu">
  <h2>{date}</h2>
  
  {#each  Object.entries(menu) as [name, dish]}
    <div>{name} {dish.ordered}</div>
  {/each}
</div>
