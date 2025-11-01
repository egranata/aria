import { workspace, EventEmitter, ExtensionContext, Uri } from "vscode";

import {
  Disposable,
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;
export async function activate(context: ExtensionContext) {

	const configuredPath = workspace.getConfiguration('aria').get<string>('lsp.serverPath')?.trim();
	const envPath = process.env.ARIA_LSP_PATH?.trim();
	const defaultUri = Uri.joinPath(context.extensionUri, '..', '..', 'target', 'debug', 'lsp');
	const command = configuredPath || envPath || defaultUri.fsPath;
	const run: Executable = {
		command,
		options: {
		env: {
			...process.env,
			RUST_LOG: "debug",
		},
		},
	};
	
  const serverOptions: ServerOptions = {
		run,
		debug: run,
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "aria" }],
		synchronize: {
		fileEvents: workspace.createFileSystemWatcher("**/.clientrc"),
		}
	};

	client = new LanguageClient("aria-language-server", "aria language server", serverOptions, clientOptions);
	client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

export function activateInlayHints(ctx: ExtensionContext) {
  const maybeUpdater = {
    hintsProvider: null as Disposable | null,
    updateHintsEventEmitter: new EventEmitter<void>(),

    async onConfigChange() {
      this.dispose();
      // TODO: reload the lsp connection if path changed  
    },

    dispose() {
      this.hintsProvider?.dispose();
      this.hintsProvider = null;
      this.updateHintsEventEmitter.dispose();
    },
  };

  workspace.onDidChangeConfiguration(maybeUpdater.onConfigChange, maybeUpdater, ctx.subscriptions);

  maybeUpdater.onConfigChange().catch(console.error);
}