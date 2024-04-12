<script lang="ts">
    import * as Tabs from '$lib/components/ui/tabs'

    let engine: 'single' | 'multi' = 'single'
</script>

<Tabs.Root bind:value={engine}>
    <Tabs.List>
        <Tabs.Trigger value="single">Single-Timepoint</Tabs.Trigger>
        <Tabs.Trigger value="multi">Multi-Timepoint</Tabs.Trigger>
    </Tabs.List>
</Tabs.Root>