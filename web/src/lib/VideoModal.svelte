<script lang="ts">
  import { page } from "$app/stores";

  let domain = window.location.hostname;

  $: id = $page.url.searchParams.get("v");
</script>

{#if id}
  <a class="backdrop" href={domain}>
    <video controls autoplay>
      <source
        src={`https://media.${domain}/video/${id}.mp4`}
        type="video/mp4"
      />
      <track kind="captions" />
    </video>
  </a>
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
    max-width: 90%;
    max-height: 90%;
  }
</style>
