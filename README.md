# suap-kit

Núcleo compartilhado dos aplicativos do [suap-companion](https://github.com/suap-companion): [chamados](https://github.com/suap-companion/chamados) (Central de Serviços) e baseconhecimento.

Este repositório **nunca contém dados do SUAP**: só código.

| Crate | O que faz |
| --- | --- |
| `suap-core` | Cliente do SUAP: login, sessão (cookies), configuração por perfil, diretórios do app (`AppPaths`). |
| `suap-rag` | Documentos de apoio: importa TXT/Markdown/PDF, divide em trechos e indexa num SQLite local (FTS5/BM25), retomável por página. |
| `suap-sync` | Blocos da sincronização cifrada: criptografia (XChaCha20-Poly1305), guarda da chave (keyring, arquivo, variável), backends (pasta, S3/R2 com SigV4) e trava de instância única. O documento sincronizado e sua mescla ficam em cada aplicativo. |

`suap-sync` oferece a feature `test-support`, que expõe `fake_s3` (um S3 em memória) para os testes de quem usa a crate.

## Usar

Por *git dependency* fixada em tag (o repo não é publicado no crates.io):

```toml
suap-core = { git = "https://github.com/suap-companion/suap-kit", tag = "v0.2.0" }
suap-rag  = { git = "https://github.com/suap-companion/suap-kit", tag = "v0.2.0" }
suap-sync = { git = "https://github.com/suap-companion/suap-kit", tag = "v0.2.0" }
```

## Desenvolver

Regras em [AGENTS.md](AGENTS.md). Validação completa (igual ao CI): `sh scripts/validar.sh`.

## Notas

- **Nome do aplicativo:** chame `suap_core::set_app_name("meu-app")` uma vez, no início do programa. O nome decide o diretório de dados, o serviço do chaveiro (`meu-app-sync`), as variáveis de ambiente (`MEU_APP_SYNC_KEY`, `MEU_APP_SYNC_PASSPHRASE`, `MEU_APP_S3_ACCESS_KEY_ID`, `MEU_APP_S3_SECRET_ACCESS_KEY`) e o comando citado nas mensagens (`meu-app sync credentials set`). Sem a chamada, vale `chamados` (compatibilidade com o primeiro aplicativo). A configuração (`~/.config/suap`) é compartilhada entre os aplicativos.
- `suap-rag`: a busca é por palavras (FTS5/BM25), sem sinônimos.
