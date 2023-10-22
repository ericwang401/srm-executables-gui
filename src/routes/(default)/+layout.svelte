<script>
    import ContentContainer from '$lib/components/ContentContainer.svelte'
</script>

<div class="h-2 w-full bg-gradient-to-r from-indigo-600 to-fuchsia-400" />

<ContentContainer>
    <div class="space-y-1.5 pb-6 pt-8 lg:pt-12">
        <h1 class="font-heading text-2xl font-semibold md:text-5xl lg:text-7xl">SRM Executables</h1>
        <p class="max-w-2xl text-muted-foreground">
            A utility that calculates the synthesis rates of specific proteins. Created by <a
                class="font-medium underline underline-offset-4"
                href="https://github.com/ericwang401"
                target="_blank"
            >
                Eric Wang
            </a>
        </p>
    </div>

    <slot />
</ContentContainer>
