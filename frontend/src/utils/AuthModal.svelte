<script lang='ts'>
	import { createEventDispatcher } from "svelte";
	import Register from "./Register.svelte";
	import Login from "./Login.svelte";
    import * as jose from 'jose'
	import { user } from "../stores/user.store";



    const dispatch = createEventDispatcher();
    let modal : "login" | "register" = "login";

    const closeModal = (e: any) => {
        if(e.key === "Escape") dispatch("close");
    }

    const handleLoggedIn = (e: any) => {
        const info = jose.decodeJwt(e.detail.token);
        user.set(info.sub);
        dispatch("close");
        window.location.reload();
    }

</script>

<svelte:document on:keydown={closeModal}/>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="backdrop" on:click={() => dispatch("close")}>
    <div class="modal" on:click|stopPropagation={() => {}}>
        {#if modal == "login"}
            <Login on:close={() => dispatch("close")} on:register={() => modal = "register"} on:loginToken={handleLoggedIn} />
        {:else}
            <Register on:close={() => dispatch("close")} on:login={() => modal = "login"}  on:loginToken={handleLoggedIn} />
        {/if}
    </div>
</div>


<style lang='scss'>
    .backdrop{
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        backdrop-filter: blur(8px);
        background-color: rgba(0,0,0,0.5);
        z-index: 2;
    }

    .modal{
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%,-50%);
        background-color: var(--background);
        border-radius: 8px;
        padding: 16px;
        width: 90vmin;
        max-width: 500px;
        height: 225px;

        z-index: 3;
    }
</style>
