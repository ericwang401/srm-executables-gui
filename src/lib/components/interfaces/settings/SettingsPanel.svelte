<script lang="ts">
    import * as Sheet from '$lib/components/ui/sheet'
    import CogButton from '$lib/components/ui/CogButton.svelte'
    import { Button } from '$lib/components/ui/button'
    import { Icon, InformationCircle } from 'svelte-hero-icons'
    import { Input } from '$lib/components/ui/input'
    import { Label } from '$lib/components/ui/label'
    import * as Tooltip from '$lib/components/ui/tooltip'
    import { Switch } from '$lib/components/ui/switch'


    let toleranceMultiplier = 2.0
    let shouldRemoveNACalculations = true

</script>

<Sheet.Root>
    <Sheet.Trigger>
        <CogButton />
    </Sheet.Trigger>
    <Sheet.Content side="right">
        <Sheet.Header>
            <Sheet.Title>Advanced Settings</Sheet.Title>
            <Sheet.Description>
                For power users 🤠
            </Sheet.Description>
        </Sheet.Header>
        <div class="flex flex-col space-y-4 py-6">
            <div class="flex flex-col space-y-1.5">
                <div class="flex items-center space-x-2">
                    <Switch id="should-remove-na-calculations" bind:checked={shouldRemoveNACalculations} />
                    <Label for="should-remove-na-calculations">Avoid N/A Calculations</Label>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <Icon src={InformationCircle} mini class='w-4 h-4' />
                        </Tooltip.Trigger>
                        <Tooltip.Content class='max-w-md'>
                            <p>Removes samples with no detected peaks from the input for a given peptide to avoid "-nan(ind)"
                                calculations. This doesn't get rid of all "-nan(ind)" calculations but helps mitigate them.</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </div>
            </div>
            <div class="flex flex-col space-y-1.5">
                <Label class="flex items-center gap-2" for="tolerance-multiplier">Tolerance Multiplier
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <Icon src={InformationCircle} mini class='w-4 h-4' />
                        </Tooltip.Trigger>
                        <Tooltip.Content class='max-w-md'>
                            <p>SRM heuristically detects data pertaining to different charges for a given peptide. It uses
                                standard deviation times this multipler to determine which mass-charge ratio don't belong in
                                the
                                same group. Adjust to what works, but 2.0 is recommended.</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Label>
                <Input id="tolerance-multiplier" type="number" step=".1" bind:value={toleranceMultiplier} />
            </div>
        </div>
        <Sheet.Footer>
            <Sheet.Close asChild let:builder>
                <Button builders={[builder]} type="submit">Save changes</Button>
            </Sheet.Close>
        </Sheet.Footer>
    </Sheet.Content>
</Sheet.Root>
