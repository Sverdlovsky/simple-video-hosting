<script lang="ts">
  const domain: string = window.location.hostname;

  export let id: string;
  export let title: string;
  export let tags: {
    id: string;
    title: string;
    color: number;
    link?: string;
  }[];
  export let users: {
    id: string;
    email: string;
    role: string;
    link?: string;
  }[];
  export let apps: { id: string; title: string; link?: string }[];

  $: if (tags?.length) {
    tags.forEach((tag) => {
      const query = new URLSearchParams(window.location.search);
      const url = new URL(window.location.href);
      url.search = query.toString();
      url.searchParams.set("tag", tag.id);
      tag.link = url.toString();
    });
  }

  $: if (users?.length) {
    users.forEach((user) => {
      const query = new URLSearchParams(window.location.search);
      const url = new URL(window.location.href);
      url.search = query.toString();
      url.searchParams.set("user", user.id);
      user.link = url.toString();
    });
  }

  $: if (apps?.length) {
    apps.forEach((app) => {
      const query = new URLSearchParams(window.location.search);
      const url = new URL(window.location.href);
      url.search = query.toString();
      url.searchParams.set("app", app.id);
      app.link = url.toString();
    });
  }
</script>

<div class="video-card">
  <a href={`/watch?v=${id}`} class="preview_button">
    <img
      src={`https://media.${domain}/video/previews/${id}.jpg`}
      alt="prw"
      class="preview_image"
      loading="lazy"
    />
  </a>
  <div class="tags">
    {#each tags as tag}
      <a
        href={tag.link}
        class="tag"
        style="background-color: #{tag.color.toString(16)}"
      >
        {tag.title}
      </a>
    {/each}
  </div>
  <div class="users">
    {#each (users || []).slice(0, 3) as user, i}
      <a
        href={user.link}
        class="user_button"
        style="z-index: {users.length - i}"
      >
        <img
          src={`https://media.${$domain}/avatars/${user.id}.png`}
          alt={user.email}
          class="user_icon"
          loading="lazy"
        />
      </a>
    {/each}
  </div>
  <p>{title}</p>
  <div class="apps">
    {#each (apps || []).slice(0, 3) as app, i}
      <a href={app.link} class="app_button" style="z-index: {apps.length - i}">
        <img
          src={`https://media.${$domain}/apps/${app.id}.png`}
          alt={app.title}
          class="app_icon"
          loading="lazy"
        />
      </a>
    {/each}
  </div>
</div>

<style>
  p {
    grid-area: title;
    line-height: 0px;
    text-align: center;
  }

  .video-card {
    display: grid;
    grid-template-columns: 1fr 6fr 1fr;
    grid-template-rows: auto auto auto;
    grid-template-areas:
      "preview preview preview"
      "tags tags tags"
      "users title apps";
    gap: 4px;
  }

  .preview_button {
    grid-area: preview;
  }

  .preview_image {
    width: 100%;
    aspect-ratio: 16 / 9;
    border-radius: 32px;
  }

  .tags {
    grid-area: tags;
  }

  .tag {
    padding: 2px 6px;
    border-radius: 6px;
  }

  .users {
    grid-area: users;
    display: flex;
    flex-direction: row;
    justify-content: start;
  }

  .user_button {
    border: none;
    background: none;
    padding: 0;
    cursor: pointer;
    margin-right: -16px;
    transition: margin-right 0.3s ease;
    -webkit-tap-highlight-color: transparent;
  }

  .users:hover .user_button {
    margin: 0;
  }

  .user_icon {
    width: 32px;
    height: 32px;
    border-radius: 16px;
  }

  .apps {
    grid-area: apps;
    display: flex;
    flex-direction: row-reverse;
    justify-content: end;
  }

  .app_button {
    border: none;
    background: none;
    padding: 0;
    cursor: pointer;
    margin-left: -16px;
    transition: margin-right 0.3s ease;
    -webkit-tap-highlight-color: transparent;
  }

  .apps:hover .app_button {
    margin: 0;
  }

  .app_icon {
    height: 32px;
    width: 32px;
    border-radius: 8px;
    margin-left: -16px;
  }
</style>
