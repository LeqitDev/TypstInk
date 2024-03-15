<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { openFile, projectStructure } from "$lib/stores";
	import { sort_entrys } from "$lib/utils.js";
	import { invoke } from "@tauri-apps/api/core";
	import { FileCode2Icon, FolderIcon, FolderOpenIcon } from "lucide-svelte";
	import { onMount } from "svelte";

    export let project_tree: App.ProjectEntry[];
    export let depth = 0;

    function updateChild(child: App.ProjectEntry) {
        if ($projectStructure.files.find((entry) => entry.parent_hash === child.hash)) return;
        invoke('get_project_structure', { path: child.path }).then((res) => {
            const child_ps = res as App.ProjectStructure;
            if (child_ps.files) {
                projectStructure.update((ps) => {
                    child_ps.files.forEach((entry) => {
                        if (ps.files.find((e) => e.hash === entry.hash)) return;
                        ps.files.push(entry);
                    });
                    return ps;
                })
            }
        })
    }

    function getChildren(child: App.ProjectEntry) {
        let children: App.ProjectEntry[] = [];
        if (!$projectStructure.files) return children;
        $projectStructure.files.forEach((entry) => {
            if (entry.parent_hash === child.hash) {
                children.push(entry);
            }
        });
        return children;
    }
</script>
{#if project_tree }
    {#each project_tree as child }
        {#if child}
            <div style={`padding-left: ${0.25 * depth*2}rem`}>
                <Button 
                    variant={child.is_file ? "ghost" : "link"} 
                    size="sm" class={`w-full justify-start`} 
                    on:click={async () => {
                        if (!child.is_file) {
                            if (!child.opened) {
                                updateChild(child);
                            }
                            child.opened = !child.opened;
                        } else {
                            openFile(child);
                        }
                    }}>
                        {#if !child.is_file}{#if child.opened}<FolderOpenIcon class="w-4 h-4 mx-2" />{:else}<FolderIcon class="w-4 h-4 mx-2" />{/if}{:else}<FileCode2Icon class="w-4 h-4 mx-2" />{/if} {child.name}
                </Button>
            </div>
            {#if child.opened}
                <svelte:self project_tree={getChildren(child)} depth={depth + 1} />
            {/if}
        {/if}
    {/each}
{/if}