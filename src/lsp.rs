use tower_lsp_server::jsonrpc::Result as LspResult;
use tower_lsp_server::{Client, LanguageServer, LspService, Server, lsp_types::*};

// https://github.com/tower-lsp-community/tower-lsp-server/blob/main/README.md
#[derive(Debug)]
struct Backend {
	client: Client,
}

impl Backend {
	fn new(client: Client) -> Self {
		Backend { client }
	}
}

impl LanguageServer for Backend {
	async fn initialize(&self, _: InitializeParams) -> LspResult<InitializeResult> {
		Ok(InitializeResult::default())
	}

	async fn initialized(&self, _: InitializedParams) {
		self.client
			.log_message(MessageType::INFO, "server initialized!")
			.await;
	}

	// async fn diagnostic(
	// 	&self,
	// 	params: DocumentDiagnosticParams,
	// ) -> LspResult<DocumentDiagnosticReportResult> {
	// }

	async fn shutdown(&self) -> LspResult<()> {
		Ok(())
	}
}

pub async fn main() -> anyhow::Result<()> {
	let stdin = tokio::io::stdin();
	let stdout = tokio::io::stdout();

	let (service, socket) = LspService::new(|client| Backend::new(client));
	Server::new(stdin, stdout, socket).serve(service).await;

	Ok(())
}
