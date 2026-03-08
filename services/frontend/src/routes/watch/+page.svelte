<script lang="ts">
    import { onMount } from "svelte";
    import { domain } from "$lib/stores";

    let video: string | null = null;

    onMount(() => {
        const params = new URLSearchParams(window.location.search);
        video = params.get("v");
    });
</script>

<main>
    <video controls autoplay>
        <source
            src={`https://media.${$domain}/video/get/${video}.mp4`}
            type="video/mp4"
        />
        <track kind="captions" />
    </video>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    video {
        width: 100%;
        max-width: 100%;
        height: auto;
    }
</style>
