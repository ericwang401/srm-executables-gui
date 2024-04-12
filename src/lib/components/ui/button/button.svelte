<script lang="ts">
    import { Button as ButtonPrimitive } from 'bits-ui'
    import { cn } from '$lib/utils'
    import { buttonVariants, type Props, type Events } from '.'
    import Spinner from '$lib/components/ui/Spinner.svelte'

    type $$Props = Props
    type $$Events = Events

    let className: $$Props['class'] = undefined
    export let variant: $$Props['variant'] = 'default'
    export let size: $$Props['size'] = 'default'
    export let builders: $$Props['builders'] = []
    export let loading: $$Props['loading'] = false
    export let disabled: $$Props['disabled'] = false
    export { className as class }
</script>

<ButtonPrimitive.Root
    {builders}
    class={cn(buttonVariants({ variant, size, className }))}
    disabled={disabled || loading}
    {...$$restProps}
    on:click
    on:keydown
>
    {#if loading}
        <Spinner class="mr-2 h-4 w-4" />
    {:else}
        <slot name="icon" />
    {/if}
    <slot />
</ButtonPrimitive.Root>
