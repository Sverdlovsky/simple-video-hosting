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

    let search = $state("");
    let videos = $state<Video[]>([]);

    $effect(() => {
        const url = new URL(`https://video.${$domain}/api/videos`);
        url.search = page.url.searchParams.toString();

        const s = search.trim();
        if (s) {
            url.searchParams.set("search", s);
        }

        url.searchParams.set("kind", "clip");
        url.searchParams.set("random", "true");

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
    <div class="search">
        <div class="search_bar">
            <img
                src={`https://media.${$domain}/icons/${$theme}/search.svg`}
                alt="src"
                class="search_icon"
            />
            <input
                type="text"
                bind:value={search}
                placeholder="Search..."
                class="search_input"
            />
        </div>
        <div class="search_filter">
            <img
                src={`https://media.${$domain}/icons/${$theme}/filter.svg`}
                alt="flt"
                class="filter_icon"
            />
        </div>
    </div>
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

    input {
        background-color: transparent;
        color: var(--neu8);
        font-size: 16px;
    }

    .search {
        display: flex;
        flex-direction: row;
        align-items: center;
    }

    .search_bar {
        height: 40px;
        width: 100%;
        display: flex;
        flex-direction: row;
        align-items: center;
        background-color: var(--sec1);
        border: 0.5px solid var(--sec4);
        border-radius: 24px;
        gap: 16px;
        padding: 0px 16px;
    }

    .search_icon {
        width: 16px;
        height: 16px;
    }

    .search_input {
        border: none;
        outline: none;
        font-size: 0.75rem;
        width: 100%;
    }

    .search_filter {
        width: 56px;
        height: 42px;
        display: flex;
        justify-content: center;
        align-content: center;
    }

    .filter_icon {
        width: 24px;
        height: 100%;
    }

    .videos {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(max(25%, 256px), 1fr));
        gap: 32px;
    }
</style>
