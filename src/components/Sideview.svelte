<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { projectStructure } from "$lib/stores";
	import { AppWindow } from "lucide-svelte";
	import { onMount } from "svelte";
	import ProjectStructureView from "./ProjectStructureView.svelte";
	import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";

    $: rootEntries = $projectStructure.files.filter((entry) => entry.parent_hash === $projectStructure.root_hash);
</script>


<div class="py-2 border-b">
    {#if $projectStructure.name}
        <h1 class="text-lg font-semibold text-center">{$projectStructure.name}</h1>
    {/if}
</div>
{#if $projectStructure.files}
    <ScrollArea class="flex-1">
        <ProjectStructureView project_tree={rootEntries} depth={0} />
    </ScrollArea>
{/if}