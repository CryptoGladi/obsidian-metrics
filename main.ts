import { Plugin, TAbstractFile, Notice } from 'obsidian';

// @ts-ignore
import rustPlugin from "./pkg/wasm_module_bg.wasm";
import * as plugin from "./pkg/wasm_module";

export default class MyPlugin extends Plugin {
	async onload() {
		const buffer = Uint8Array.from(atob(rustPlugin), c => c.charCodeAt(0))
		await plugin.default(Promise.resolve(buffer));

		this.addCommand({
			id: 'get-metrics-to-json',
			name: 'Get metrics to json',
			callback: async () => {
				console.time("getMetrics");
				new Notice('Getting metrics...');

				let files = this.app.vault.getMarkdownFiles();

				// @ts-ignore
				let basePath = this.app.vault.adapter.basePath;

				const promises = files.map(async (file) => {
					let text = await this.app.vault.cachedRead(file);
					let path = `${basePath}/${file.path}`;

					return new plugin.NoteInfo(text, path);
				});

				let textFromFiles = await Promise.all(promises);
				let json_metrics = plugin.get_json_metrics(textFromFiles, basePath);

				let file_metrics = this.app.vault.getRoot().children.find((d) => {
					return d.path == "metrics.json";
				});

				if (file_metrics instanceof TAbstractFile)
					this.app.vault.delete(file_metrics);

				this.app.vault.create("metrics.json", json_metrics);

				console.timeEnd("getMetrics");
				new Notice('Done getting metrics!');
			}
		});
	}
}