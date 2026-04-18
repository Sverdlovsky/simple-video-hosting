<script lang="ts">
  import { Video, videos } from "$lib/stores";
  import { page } from "$app/state";

  let domain: string = window.location.hostname;
  let title: string = domain.split(".")[0];
  let search: string = $state("");
  let kind: string = $state("all");

  $effect(() => {
    const url = new URL(`https://api.${domain}/videos`);
    url.search = page.url.searchParams.toString();

    const s = search.trim();
    if (s) {
      url.searchParams.set("search", s);
    }

    url.searchParams.set("kind", kind);
    url.searchParams.set("random", (kind == "All").toString());

    const debounceTimeout = setTimeout(async () => {
      try {
        const res = await fetch(url);
        if (!res.ok) {
          console.error("Request error");
          videos.set([]);
          return;
        }
        videos.set((await res.json()) as Video[]);
      } catch (err) {
        console.error("Network error", err);
        videos.set([]);
      }
    }, 300);

    return () => clearTimeout(debounceTimeout);
  });
</script>

<div class="navbar">
  <div class="navbox">
    <div class="top">
      <a href="/" class="title">
        <img src="/svg/favicon.svg" alt=">" />
        <p>{title}</p>
      </a>
      <div class="actions">
        <button class="add">
          <img src="/svg/add.svg" alt="+" />
          <p>Add</p>
        </button>
        <a href={`https://auth.${domain}/with/google`} class="sign">
          <img src="/svg/sign.svg" alt="o" />
          <p>Sign in</p>
        </a>
      </div>
    </div>
    <div class="search">
      <img src="/svg/search.svg" alt="Q" />
      <input type="text" bind:value={search} placeholder="Search..." />
    </div>
    <div class="categories">
      <button class:active={kind === "all"} onclick={() => (kind = "all")}>
        All
      </button>
      <button class:active={kind === "full"} onclick={() => (kind = "full")}>
        Fulls
      </button>
      <button class:active={kind === "clip"} onclick={() => (kind = "clip")}>
        Clips
      </button>
      <button class:active={kind === "short"} onclick={() => (kind = "short")}>
        Shorts
      </button>
    </div>
  </div>
</div>

<style>
  .navbar {
    z-index: 2;
    position: fixed;
    width: 100%;
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid var(--color-zinc-900);
    align-items: center;
  }

  .navbox {
    width: calc(min(max(60%, 1440px), 100%) - 48px);
    display: flex;
    flex-direction: column;
    padding: 24px;
    gap: 16px;
  }

  .top {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  .title {
    display: flex;
    flex-direction: row;
    gap: 8px;
    align-items: center;
    text-decoration: none;
    color: var(--color-white);
  }

  .title img {
    width: 48px;
    height: 48px;
  }

  .title p {
    font-size: var(--text-2xl);
    line-height: 0px;
    font-weight: var(--font-weight-medium);
  }

  .actions {
    display: flex;
    flex-direction: row;
  }

  .add {
    height: 40px;
    display: flex;
    flex-direction: row;
    gap: 4px;
    align-items: center;
    background-color: var(--color-zinc-200);
    border: none;
    border-radius: calc(infinity * 1px);
    padding-left: 16px;
    padding-right: 24px;
  }

  .add img {
    width: 16px;
    height: 16px;
    color: var(--color-black);
  }

  .add p {
    color: var(--color-black);
    font-size: var(--text-base);
    font-weight: var(--font-weight-medium);
    line-height: 0px;
  }

  .sign {
    height: 40px;
    display: flex;
    flex-direction: row;
    gap: 4px;
    align-items: center;
    background-color: color-mix(
      in oklab,
      var(--color-zinc-900) 50%,
      transparent
    );
    border-radius: calc(infinity * 1px);
    padding-left: 16px;
    padding-right: 24px;
    text-decoration: none;
  }

  .sign img {
    width: 16px;
    height: 16px;
    color: var(--color-zinc-400);
  }

  .sign p {
    color: var(--color-zinc-400);
    font-size: var(--text-base);
    font-weight: var(--font-weight-medium);
    line-height: 0px;
  }

  .search {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    background-color: color-mix(
      in oklab,
      var(--color-zinc-900) 50%,
      transparent
    );
    border: 1px solid color-mix(in oklab, var(--ring) 50%, transparent);
    border-radius: calc(infinity * 1px);
    align-items: center;
  }

  .search img {
    width: 20px;
    height: 20px;
    color: var(--color-zinc-500);
    padding: 14px;
  }

  .search input {
    width: 100%;
    height: 46px;
    background-color: transparent;
    border: none;
    outline: none;
    font-size: 16px;
    color: var(--color-zinc-200);
  }

  .categories {
    display: flex;
    flex-direction: row;
    justify-content: start;
    overflow: scroll;
    gap: 8px;
  }

  .categories button {
    height: 40px;
    border: none;
    border-radius: calc(infinity * 1px);
    padding: 0px 24px;
    background-color: var(--color-zinc-900);
    color: var(--color-zinc-400);
    text-decoration: none;
    font-size: var(--text-sm);
    font-weight: var(--font-weight-medium);
    line-height: 0px;
  }

  .categories button:active {
    background-color: var(--color-white);
    color: var(--color-black);
  }
</style>
