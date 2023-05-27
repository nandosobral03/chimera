<script lang='ts'>
    import {user} from "../stores/user.store";
	import AuthModal from "./AuthModal.svelte";
    let showLogin = false;
    const openLogin = () => {
        showLogin = true;
    }   

    
    const logout = () => {
        localStorage.removeItem('token');
        localStorage.removeItem('guest_id');
        user.set(undefined);
        if(window.location.pathname == "/profile") window.location.href = "/play";
        else window.location.reload();
    }
</script>


<div class="user-info">
    {#if $user == undefined}
        <button on:click={() => openLogin()}>Login</button>
    {:else}
        <div>
            <span>
                Logged in as <span
                  class="username"
                  on:click|once = { () => window.location.href = `/profile` } 
                  on:keydown|preventDefault|stopPropagation = { (e) => e.key == "Enter" ? window.location.href = `/profile` : null }>
                    {$user}
                </span>
            </span>
            <button on:click={() => logout()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--! Font Awesome Pro 6.4.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. --><path d="M502.6 278.6c12.5-12.5 12.5-32.8 0-45.3l-128-128c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L402.7 224 192 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l210.7 0-73.4 73.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l128-128zM160 96c17.7 0 32-14.3 32-32s-14.3-32-32-32L96 32C43 32 0 75 0 128L0 384c0 53 43 96 96 96l64 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-64 0c-17.7 0-32-14.3-32-32l0-256c0-17.7 14.3-32 32-32l64 0z"/></svg>
            </button>
        </div>
    {/if}
</div>
{#if showLogin}
    <AuthModal  on:close={() => showLogin = false}/>
{/if}

<style lang='scss'>
    .user-info{
        position: absolute;
        bottom: 16px;
        right: 16px;
        background-color: var(--shade);
        border-radius: 8px;
        z-index: 1;
        border: 1px solid var(--background);
        div{
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 8px 16px;
            gap: 16px;
            button{
                padding: 0px;
            }
            span{
                display: flex;
                align-items: center;
                gap: 8px;
            }
            .username{
                cursor: pointer;
                opacity: 1;
                &:hover{
                    filter: brightness(1.1);
                }
            }
        }
        span{
            padding: 8px 0;
            display: block;
            font-size: 16px;
            color: var(--background);
        }
        button{
            appearance: none;
            border: none;
            background-color: var(--shade);
            border-radius: 8px;
            padding: 8px 16px;
            cursor: pointer;
            font-size: 16px;
            color: var(--background);
            outline: none;
            opacity: 0.8;
            text-transform: uppercase;
            &:hover{
                opacity: 1;
            }
             
            svg{
                width: 16px;
                fill: var(--background);
                margin: auto;
            }
        }
    }
</style>