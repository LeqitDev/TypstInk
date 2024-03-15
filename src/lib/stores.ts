import { invoke } from "@tauri-apps/api/core";
import { writable, get } from "svelte/store";

export const projectStructure = writable<App.ProjectStructure>({
    name: "Project",
    root_path: "\\",
    root_hash: "",
    files: []
});

export const currentFile = writable<{ file: App.ProjectEntry, content: string } | undefined>();

export const openedFiles = writable<App.ProjectEntry[]>([]);

export const undefinedFile = writable<string>("");

export async function openFile(file: App.ProjectEntry) {
    let alreadyOpened = false;
    openedFiles.update((files) => {
        if (files.includes(file)) {
            alreadyOpened = true;
            return files;
        }
        return [...files, file];
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

export function closeFile(file: App.ProjectEntry) {
    const cur = get(currentFile);
    let openedFilesVal = get(openedFiles);
    const index = openedFilesVal.findIndex((f) => f === file);
    openedFiles.update((files) => files.filter((f) => f !== file));
    openedFilesVal = get(openedFiles);
    if (cur && cur.file.path === file.path) {
        if (openedFilesVal.length === 0) {
            openUndefined();
        } else {
            const nextFile = openedFilesVal[index] ?? openedFilesVal[index - 1];
            openFile(nextFile);
        }
    }
}

export function openUndefined() {
    currentFile.set({file: { name: "", path: "", hash: "", parent_hash: "", is_file: true}, content: get(undefinedFile)});
}