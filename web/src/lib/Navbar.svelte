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
        <p>${title}</p>
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
      <button onclick={() => (kind = "all")}> All </button>
      <button onclick={() => (kind = "full")}> Fulls </button>
      <button onclick={() => (kind = "clip")}> Clips </button>
      <button onclick={() => (kind = "short")}> Shorts </button>
    </div>
  </div>
</div>

<style>
  .navbar {
    width: 100%;
    z-index: 2;
    display: flex;
    border-bottom: 1px solid #222222;
  }

  .navbox {
    max-width: max(60%, 1440px);
    display: flex;
    flex-direction: columns;
    margin: 16px;
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
  }

  .title img {
    width: 48px;
    height: 48px;
  }

  .title p {
    font-size: 32;
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
    background-color: white;
    border-radius: 20px;
    padding: 4px 8px;
    margin: 16px 8px;
  }

  .add img {
    width: 16px;
    height: 16px;
    color: black;
  }

  .add p {
    font-size: 16;
    color: black;
    line-height: 0px;
  }

  .sign {
    display: flex;
    flex-direction: row;
    gap: 4px;
    background-color: #222222;
  }

  .sign img {
    width: 16px;
    height: 16px;
    color: gray;
  }

  .sign p {
    font-size: 16;
    color: gray;
    line-height: 0px;
  }

  .search {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    background-color: black;
    border: 1px solid #222222;
    border-radius: 24px;
    flex-direction: row;
    align-items: center;
  }

  .search img {
    width: 20px;
    height: 20px;
    color: gray;
    padding: 14px;
  }

  .search input {
    width: 100%;
    height: 48px;
    background-color: transparent;
    border: none;
    outline: none;
    font-size: 16;
    color: white;
  }

  .categories {
    display: flex;
    flex-direction: row;
    justify-content: start;
    overflow: scroll;
    gap: 8px;
  }
</style>
