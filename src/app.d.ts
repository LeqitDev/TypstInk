// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}

		interface ProjectStructure {
			name: string;
			root_path: string;
			root_hash: string;
			files: ProjectEntry[];
		}

		interface ProjectEntry {
			name: string;
			path: string;
			hash: string;
			is_file: boolean;
			parent_hash: string;
			opened?: boolean;
		}
	}
}

export {};
