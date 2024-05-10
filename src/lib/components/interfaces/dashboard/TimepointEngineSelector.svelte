<script lang="ts">
    import * as Tabs from '$lib/components/ui/tabs'
    import { engineType } from '$lib/stores/settings'

</script>

<Tabs.Root bind:value={$engineType}>
    <Tabs.List>
        <Tabs.Trigger value="single">Single-Timepoint</Tabs.Trigger>
        <Tabs.Trigger value="multi">Multi-Timepoint</Tabs.Trigger>
    </Tabs.List>
</Tabs.Root>