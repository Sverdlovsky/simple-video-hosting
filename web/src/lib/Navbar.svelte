<script lang="ts">
  import { Video, videos } from "$lib/stores";
  import { page } from "$app/state";

  let domain: string = window.location.hostname;
  let title: string = domain.split(".")[0];
  let search: string = $state("");
  let kind: string = $state("");

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
    border-bottom: 1px solid #222222;
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
    color: white;
  }

  .title img {
    width: 48px;
    height: 48px;
  }

  .title p {
    font-size: 24px;
    line-height: 0px;
  }

  .actions {
    display: flex;
    flex-direction: row;
  }

  .add {
    display: flex;
    flex-direction: row;
    gap: 4px;
    align-items: center;
    background-color: white;
    border: none;
    border-radius: 20px;
    padding: 4px 24px 4px 16px;
    margin: 4px 8px;
  }

  .add img {
    width: 16px;
    height: 16px;
    color: black;
  }

  .add p {
    font-size: 16px;
    color: black;
    line-height: 0px;
  }

  .sign {
    display: flex;
    flex-direction: row;
    gap: 4px;
    align-items: center;
    background-color: #222222;
    border-radius: 20px;
    padding: 4px 24px 4px 16px;
    margin: 4px 8px;
  }

  .sign img {
    width: 16px;
    height: 16px;
    color: #444444;
  }

  .sign p {
    font-size: 16px;
    color: #444444;
    line-height: 0px;
  }

  .search {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    background-color: black;
    border: 1px solid #222222;
    border-radius: 25px;
    flex-direction: row;
    align-items: center;
  }

  .search img {
    width: 20px;
    height: 20px;
    color: #444444;
    padding: 14px;
  }

  .search input {
    width: 100%;
    height: 46px;
    background-color: transparent;
    border: none;
    outline: none;
    font-size: 16px;
    color: white;
  }

  .categories {
    display: flex;
    flex-direction: row;
    justify-content: start;
    overflow: scroll;
    gap: 8px;
  }

  .categories button {
    border: none;
    border-radius: 20px;
    padding: 4px 24px 4px 16px;
    margin: 4px 8px;
    background-color: #222222;
    color: #444444;
    text-decoration: none;
  }

  .categories button:active {
    background-color: white;
    color: black;
  }
</style>
