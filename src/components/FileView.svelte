<script lang="ts">
	import { onMount } from 'svelte';
	import * as monaco from 'monaco-editor';
	import * as oni from 'vscode-oniguruma';
	import onigurumaWasm from 'vscode-oniguruma/release/onig.wasm?url';
	import * as textmate from 'vscode-textmate';
	import type { Registry, StateStack } from 'vscode-textmate';
	import { INITIAL } from 'vscode-textmate';
	import { closeFile, currentFile, openFile, openedFiles } from '$lib/stores';
	import { XIcon } from 'lucide-svelte';
	import ScrollArea from "$lib/components/ui/scroll-area/scroll-area.svelte";

	let editorPane: HTMLElement;

	const grammarURL =
		'https://raw.githubusercontent.com/typst/typst/main/tools/support/typst.tmLanguage.json';
	const configURL = 'https://raw.githubusercontent.com/typst/typst/main/tools/support/config.json';
    let editor: monaco.editor.IStandaloneCodeEditor;

	let value = "";

    monaco.languages.register({ id: "typst", extensions: ["typ"] });

	/** Wires up monaco-editor with monaco-textmate */
	export function wireTextMateGrammars(
		/** TmGrammar `Registry` this wiring should rely on to provide the grammars. */
		registry: Registry,

		/** `Map` of language ids (string) to TM names (string). */
		languages: Map<string, string>,

		/** The monaco editor instance to wire up. */
		editor: monaco.editor.ICodeEditor | undefined
	) {

		return Promise.all(
			Array.from(languages.keys()).map(async (languageId) => {
                const grammar = await registry.loadGrammar(languages.get(languageId) || '');

				monaco.languages.setTokensProvider(languageId, {
					getInitialState: () => new TokenizerState(INITIAL),
					tokenize: (line: string, state: TokenizerState) => {
						const result = grammar!.tokenizeLine(line, state.ruleStack);

						return {
							endState: new TokenizerState(result.ruleStack),
							tokens: result.tokens.map((token) => {
								const scopes = token.scopes.slice(0);

								/* for (let i = scopes.length - 1; i >= 0; i--) {
									const scope = scopes[i];
									const foreground = tokenTheme._match(scope)._foreground;

									if (foreground !== defaultForeground) {
										return {
											...token,
											scopes: scope
										};
									}
								} */

								return {
									...token,
									scopes: scopes[scopes.length - 1]
								};
							})
						};
					}
				});
			})
		);
	}

	class TokenizerState implements monaco.languages.IState {
		constructor(private _ruleStack: StateStack) {}

		public get ruleStack(): StateStack {
			return this._ruleStack;
		}

		public clone(): TokenizerState {
			return new TokenizerState(this._ruleStack);
		}

		public equals(other: monaco.languages.IState): boolean {
			return (
				other instanceof TokenizerState && (other === this || other.ruleStack === this.ruleStack)
			);
		}
	}

    async function initMonaco() {
        const wasm = await fetch(onigurumaWasm).then((res) => res.arrayBuffer());
        await oni.loadWASM(wasm);

        const registry = new textmate.Registry({
            onigLib: Promise.resolve(oni),
            loadGrammar: async (scopeName) => {
                const grammar = await fetch(grammarURL).then((res) => res.json());
                return grammar;
            }
        });

        const languages = new Map([['typst', 'source.typst']]);

        monaco.languages.register({ id: 'typst', extensions: ['.typ'] });
        monaco.languages.setLanguageConfiguration('typst', 
            fetch(configURL) as unknown as monaco.languages.LanguageConfiguration
        )

        await wireTextMateGrammars(registry, languages, undefined);

        monaco.editor.defineTheme('typst-dark-theme', {
            base: 'vs-dark',
            inherit: true,
            colors: {
                'editor.foreground': '#E2E2E2',
                'editor.background': '#030711',
                'editor.selectionBackground': '#44475A',
                'editor.lineHighlightBackground': '#202936bb',
                'editorCursor.foreground': '#F8F8F2',
                'editorWhitespace.foreground': '#F8F8F2'
            },
            rules: [
                { token: 'heading', foreground: '#FF0000' },
                { token: 'bold', fontStyle: 'bold' },
                { token: 'italic', fontStyle: 'italic' },
                { token: 'raw', foreground: '#00FF00' },
                { token: 'math', foreground: '#2222FF' },
                { token: 'comment', foreground: '#828282' },
                { token: 'string', fontStyle: 'italic bold' }
            ]
        });
    }

    async function init() {
        const EditorWorker = await import("monaco-editor/esm/vs/editor/editor.worker?worker");
		await initMonaco();

        // @ts-ignore
        self.MonacoEnvironment = {
            getWorker: function (_moduleId: any, label: string) {
                return new EditorWorker.default();
            },
        };

        editor = monaco.editor.create(editorPane, {
            value,
            language: 'typst',
            theme: 'typst-dark-theme',
            automaticLayout: true
        });

		editor.onDidChangeModelContent(handleChanges);

		editor.onDidChangeModel((e) => {
			console.log("hi");
		})
    }

	onMount(() => {
        init();

        currentFile.subscribe((value) => {
            if (editor && value && editor.getValue() !== value.content) {
                editor.setValue(value.content);
            }
        });

        return () => {
            if (editor) editor.dispose();
        };
	});

	function handleChanges(event: monaco.editor.IModelContentChangedEvent) {
		if (!editor || !$currentFile || !editor.getModel()) return;
		if (editor.getModel()!.getValue() !== $currentFile.content) {
			openedFiles.update((files) => {
				let index = files.findIndex((file) => file.hash === $currentFile!.file.hash);
				if (index !== -1) {
					files[index].changed = true;
				}
				return files;
			});
		} else {
			openedFiles.update((files) => {
				let index = files.findIndex((file) => file.hash === $currentFile!.file.hash);
				if (index !== -1) {
					files[index].changed = false;
				}
				return files;
			});
		}
	}
</script>

<ScrollArea class="border-b" orientation="horizontal">
	<div class="flex flex-row text-nowrap h-8">
		{#each $openedFiles as file}
			<div class="px-1 border-r flex flex-row h-full items-center">
				{#if file.changed}
					<div class="w-2 h-2 bg-accent-foreground rounded-full mr-1"></div>
				{/if}
				<button class:underline={$currentFile?.file.hash === file.hash} on:click={() => openFile(file)}>{file.name}</button>
				<button class="pl-0.5" on:click={() => closeFile(file)}>
					<XIcon class="w-4 h-4" />
				</button> 
			</div>
		{:else}
			<div class="px-1 border-r flex flex-row h-full">
				<button class="underline">undefined</button>
				<button class="pl-0.5">
					<XIcon class="w-4 h-4" />
				</button> 
			</div>
		{/each}
	</div>
</ScrollArea>
<div class="flex h-full">
	<div class="w-full" bind:this={editorPane}></div>
</div>
