<script lang="ts">
    import type { HTMLAttributes } from 'svelte/elements'
    import { cn } from '$lib/utils'
    import { Icon, Cog6Tooth } from 'svelte-hero-icons'


    interface $$props extends HTMLAttributes<HTMLButtonElement> {
    }

</script>

<button {...$$props} class={cn('grid items-center p-1 transition hover:rotate-45 focus:outline-none', $$props.class)}>
    <Icon src={Cog6Tooth} mini class="w-5 h-5 text-foreground" />
</button>