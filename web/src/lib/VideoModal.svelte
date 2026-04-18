<script lang="ts">
  import { page } from "$app/state";

  let domain = window.location.hostname;

  let id = $derived(page.url.searchParams.get("v"));
</script>

{#if id}
  <button class="backdrop" onclick={() => page.url.searchParams.delete("v")}>
    <video controls autoplay>
      <source
        src={`https://media.${domain}/video/${id}.mp4`}
        type="video/mp4"
      />
      <track kind="captions" />
    </video>
  </button>
{/if}

<style>
  a {
    z-index: 3;
    position: fixed;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 24px;
    padding: 24px;
    background-color: #000000bf;
  }

  video {
    max-width: 96%;
    max-height: 96%;
  }
</style>
