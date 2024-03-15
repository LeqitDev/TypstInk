<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { projectStructure } from '$lib/stores';
    import { invoke } from '@tauri-apps/api/core';
    import Menubar from '../components/MenuBar.svelte';
	import Sideview from '../components/Sideview.svelte';
    import * as Resizable from "$lib/components/ui/resizable";
	import FileView from '../components/FileView.svelte';

    interface Tree {
        entry: App.ProjectEntry;
        children: Tree[];
    }

    invoke('get_project_structure', { path: "P:/Programmieren/Rust/mailo" }).then(async (res) => {
        projectStructure.set(res as App.ProjectStructure);
        console.log($projectStructure);
        // Fill the tree
        // project_tree = { entry: { name: "Mailo", path: root, is_file: false }, children: await get_children(root) };
    });
</script>

<div class="flex flex-col h-screen w-screen">
    <Menubar />
    <div class="flex-grow overflow-hidden">
        <Resizable.PaneGroup direction="horizontal">
            <Resizable.Pane minSize={10} defaultSize={16} maxSize={25}>
                <div class="h-full flex flex-col">
                    <Sideview />
                </div>
            </Resizable.Pane>
            <Resizable.Handle />
            <Resizable.Pane>
                <div class="h-full flex flex-col">
                    <FileView />
                </div>
            </Resizable.Pane>
        </Resizable.PaneGroup>
    </div>
</div>