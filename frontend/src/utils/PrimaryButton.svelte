<script lang='ts'>
    export let disabled : boolean = false;
    export let type: "standard" | "error" = "standard";
	import { createEventDispatcher, onMount } from "svelte";

    const dispatch = createEventDispatcher();
</script>
<button 
    class={type}
on:click|stopPropagation={()=>{
    if(!disabled)dispatch("click")
}} {disabled}
    >
    <slot/>
</button>

<style lang='scss'>
   button{
        padding: 8px 16px;
        border-radius: 8px;
        border: none;
        background-color: var(--shade);
        color: var(--background);
        font-weight: bold;
        cursor: pointer;
        text-transform: uppercase;
        &:hover{
            background-color: var(--dark-accent);
            color: var(--background);
        }
        &:disabled{
            opacity: 0.5;
            cursor: not-allowed;
        }
    }

    .error{
        background-color: var(--danger) !important;
        &:hover{
            background-color: var(--danger) !important;
            filter: brightness(1.1);
        }
    }

</style>