# AGENTS.md

Guia para agentes (e pessoas) que trabalham neste repositório: bibliotecas Rust compartilhadas entre `suap-companion/chamados` e `suap-companion/baseconhecimento`. Fale e escreva em **português do Brasil**.

## Mapa

- `crates/suap-core`: SUAP (login, sessão, configuração, `AppPaths`).
- `crates/suap-rag`: índice de documentos (SQLite/FTS5), chunking e extração de texto.
- `crates/suap-sync`: criptografia, chaves, backends (pasta, S3/R2) e trava; sem o documento sincronizado (é de cada app).

## Validação (antes de entregar)

- `sh scripts/validar.sh` (sem Rust local: Docker `rust:latest`): `cargo fmt`, `clippy -D warnings` e `cargo llvm-cov --fail-under-lines 100`. Cobertura de **100% das linhas** é regra (RS-01): uma linha só com `)?;` conta como perdida; closures e ramos nunca executados também.
- Uma só instanciação genérica por binário de teste (genéricos instanciados com tipos diferentes perdem linhas no resumo).

## Regras

- **Privacidade**: nada de conteúdo de chamado/base de conhecimento, senha, cookie ou chave em logs, issues, PRs ou testes.
- **Dependências novas** só com justificativa.
- **Compatibilidade**: os apps fixam o kit por tag. Mudança incompatível → versão maior (0.x: minor) e os apps atualizam a tag em PR próprio.
- **Git**: commits em pt-BR `tipo(escopo): [TAG] Descrição`, com **Motivo**, **Impacto** e **Arquivos**; autoria de Kelson da Costa Medeiros, sem `Co-Authored-By`.
- **Release**: aumente a versão em `Cargo.toml` (`[workspace.package]`), mescle com CI verde e crie a tag `vX.Y.Z` com `gh release create`.
- Mudança em workflow: carregue a skill `github-actions` e consulte as versões das actions.
