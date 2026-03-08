<script lang="ts">
    import { domain, theme } from "$lib/stores";

    const links = [
        { href: "/", text: "Home" },
        { href: "/videos", text: "Videos" },
        { href: "/clips", text: "Clips" },
        { href: "/shorts", text: "Shorts" },
        { href: "/favorites", text: "Favorites" },
        { href: "/settings", text: "Settings" },
        {
            href: `https://auth.${$domain}/with/google?next=https://video.${$domain}`,
            text: "Profile",
        },
    ];
</script>

<div class="navbar">
    <a href="/" id="title">
        <img
            src={`https://media.${$domain}/icons/logo.png`}
            alt="ico"
            style="width: 48px; height: 48px;"
        />
        <h2>Video</h2>
    </a>
    {#each links as { href, text }}
        <a
            {href}
            id={`${text.toLowerCase()}`}
            style={`grid-area: ${text.toLowerCase()};`}
        >
            <img
                src={`https://media.${$domain}/icons/${$theme}/${text.toLowerCase()}.svg`}
                alt="ico"
                id={`${text.toLowerCase()}_icon`}
            />
            <p id={`${text.toLowerCase()}_text`}>{text}</p>
        </a>
    {/each}
</div>

<style>
    a {
        width: 100%;
        height: 100%;
        display: grid;
        box-sizing: border-box;
        @media (orientation: portrait) {
            grid-template-rows: 1fr min-content 2px min-content 1fr;
            grid-template-areas:
                "."
                "icon"
                "."
                "text"
                ".";
        }
        @media (orientation: landscape) {
            grid-template-columns: 1fr 1fr 1fr;
            grid-template-areas: "icon text text";
        }
        color: inherit;
        align-items: center;
        text-decoration: none;
    }

    img {
        grid-area: icon;
        width: 24px;
        height: 24px;
        justify-self: center;
    }

    p {
        grid-area: text;
        line-height: 0px;
        @media (orientation: portrait) {
            font-size: 0.5rem;
            justify-self: center;
        }
        @media (orientation: landscape) {
            justify-self: left;
        }
        pointer-events: none;
    }

    .navbar {
        display: grid;
        @media (orientation: portrait) {
            height: 64px;
            grid-template-columns: 1fr repeat(7, 4fr) 1fr;
            grid-template-areas: ". home videos shorts clips favorites settings profile .";
        }
        @media (orientation: landscape) {
            width: 192px;
            grid-template-columns: repeat(3, 1fr);
            grid-template-rows: 96px repeat(5, 40px) auto 64px;
            grid-template-areas:
                "title title title"
                "home home home"
                "videos videos videos"
                "shorts shorts shorts"
                "clips clips clips"
                "favorites favorites favorites"
                ". . ."
                "settings . profile";
        }
        justify-items: center;
        align-items: center;
        box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.5);
    }

    #title {
        @media (orientation: portrait) {
            display: none;
        }
        @media (orientation: landscape) {
            grid-area: title;
            display: flex;
            flex-direction: row;
            justify-content: center;
            align-items: center;
            gap: 4px;
        }
    }

    #settings,
    #profile {
        @media (orientation: landscape) {
            grid-template-columns: 1fr;
            grid-template-areas: "icon";
            justify-content: center;
            padding: 0px;
        }
    }

    #settings_text,
    #profile_text {
        @media (orientation: landscape) {
            display: none;
        }
    }
</style>
