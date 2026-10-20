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
suap-core = { git = "https://github.com/suap-companion/suap-kit", tag = "v0.1.0" }
suap-rag  = { git = "https://github.com/suap-companion/suap-kit", tag = "v0.1.0" }
suap-sync = { git = "https://github.com/suap-companion/suap-kit", tag = "v0.1.0" }
```

## Desenvolver

Regras em [AGENTS.md](AGENTS.md). Validação completa (igual ao CI): `sh scripts/validar.sh`.

## Notas

- `suap-core` ainda grava em diretórios com o nome `chamados` (constante `APPLICATION`) e `suap-sync` usa o serviço de chaveiro `chamados-sync`: mantido para não mudar o comportamento do chamados. Parametrizar é trabalho futuro.
- `suap-rag`: a busca é por palavras (FTS5/BM25), sem sinônimos.
