import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";

export const projectStructure = writable<App.ProjectStructure>({
    name: "Project",
    root_path: "\\",
    root_hash: "",
    files: []
});

export const currentFile = writable<{ file: App.ProjectEntry, content: string } | undefined>();

export const openedFiles = writable<string[]>([]);

export async function openFile(file: App.ProjectEntry) {
    let alreadyOpened = false;
    openedFiles.update((files) => {
        if (files.includes(file.path)) {
            alreadyOpened = true;
            return files;
        }
        return [...files, file.path];
    });
    if (alreadyOpened) {
        const cur = get(currentFile);
        if (cur && cur.file.path === file.path) {
            return;
        }
    }
    invoke("read_file", { path: file.path }).then((content) => {
        currentFile.set({ file, content: content as string });
    });
}