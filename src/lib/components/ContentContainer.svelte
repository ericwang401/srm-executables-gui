<script lang="ts">
    import type { HTMLAttributes } from 'svelte/elements'
    import { cn } from '$lib/utils'

    interface $$props extends HTMLAttributes<HTMLDivElement> {}
</script>

<div {...$$props} class={cn('mx-auto max-w-7xl px-6 lg:px-8', $$props.class)}>
    <slot />
</div>
