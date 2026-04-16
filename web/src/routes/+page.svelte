<script lang="ts">
  import { videos } from "$lib/stores";
  import VideoCard from "$lib/VideoCard.svelte";
</script>

<main>
  <div class="videos">
    {#each videos as video}
      <VideoCard {...video} />
    {/each}
  </div>
</main>

<style>
  main {
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    padding: 24px;
  }

  .videos {
    max-width: max(60%, 1440px);
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(max(25%, 256px), 1fr));
    overflow: scroll;
    gap: 32px;
    padding-top: 230px;
  }
</style>
