<script lang='ts'>
    export let showNavigation = false;

    let routes = [
        {
            name: 'Play',
            path: '/play'
        },
        {
            name: 'About',
            path: '/about'
        },
        {
            name: 'Stats',
            path: '/stats'
        },
        {
            name: 'Leaderboard',
            path: '/leaderboard'
        }
    ]
</script>

<section style={showNavigation ? 'opacity: 1; pointer-events: all;' : 'opacity: 0; pointer-events: none;'}>
    <div class="routes">
        {#each routes as route}
                <a href={route.path} on:click={() => showNavigation = false}>
                    {route.name}
                </a>
        {/each}
    </div>

    <div class="decorator">
        <svg xmlns="http://www.w3.org/2000/svg">
            <line x1="80%" y1="0" x2="50%" y2="20%" stroke="var(--background)" stroke-width="1" />
            <line x1="100%" y1="5%" x2="65%" y2="60%" stroke="var(--background)" stroke-width="1" />
            <line x1="100%" y1="85%" x2="30%" y2="40%" stroke="var(--background)" stroke-width="1" />
            <line x1="0" y1="20%" x2="40%" y2="60%" stroke="var(--background)" stroke-width="1" />
            <line x1="0" y1="85%" x2="70%" y2="70%" stroke="var(--background)" stroke-width="1" />
        </svg>
    </div>
      
</section>

<style lang='scss'>
    section{
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: var(--shade);
        z-index: 100;
        padding: 32px;
        
        transition: all 0.1s ease-in-out;
    }
    
    .routes{
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .decorator{
        position: absolute;
        bottom: 0;
        right: 0;
        width: 100vw;
        height: 100vh;
        z-index: -1;
        svg{
            width: 100%;
            height: 100%;
        }
    }

    a{
        width: fit-content;
        color: var(--background);
        text-transform: uppercase;
        cursor: pointer;
        text-decoration: none;
        &:hover{
            text-decoration: underline;
        }
    }
</style>