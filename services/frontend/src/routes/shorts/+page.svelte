<script lang="ts">
    import { domain, theme } from "$lib/stores";
    import { page } from "$app/state";
    import VideoCard from "$lib/VideoCard.svelte";

    type Tag = { id: string; title: string; color: number; link?: string };
    type User = { id: string; email: string; role: string; link?: string };
    type App = { id: string; title: string; link?: string };

    type Video = {
        id: string;
        title: string;
        tags: Tag[];
        users: User[];
        apps: App[];
    };

    let videos = $state<Video[]>([]);

    $effect(() => {
        const url = new URL(`https://video.${$domain}/api/videos`);
        url.search = page.url.searchParams.toString();
        url.searchParams.set("kind", "short");

        const debounceTimeout = setTimeout(async () => {
            try {
                const res = await fetch(url);
                if (!res.ok) {
                    console.error("Request error");
                    videos = [];
                    return;
                }
                videos = (await res.json()) as Video[];
            } catch (err) {
                console.error("Network error", err);
                videos = [];
            }
        }, 300);

        return () => clearTimeout(debounceTimeout);
    });
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
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: auto;
        gap: 24px;
        padding: 24px;
    }

    .videos {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(max(25%, 256px), 1fr));
        gap: 32px;
    }
</style>
