# 🎯 Exemplos Práticos: Merge Mode em Ação

## Cenário 1: Setup Modular de Dotfiles

### Estrutura de Arquivos

```
~/.dotfiles/
├── base/
│   ├── .bashrc
│   ├── .zshrc
│   └── .config/
│       └── nvim/
│           └── init.lua
├── dev/
│   ├── .bashrc          # Contém apenas export PATH e DEV tools
│   ├── .zshrc           # Contém apenas export PATH e DEV tools
│   └── .cargo/
│       └── config.toml
├── gui/
│   ├── .bashrc          # Contém apenas GUI-related exports
│   └── .config/
│       └── i3/
│           └── config
└── work/
    ├── .bashrc          # Contém empresa-specific configs
    └── .ssh/
        └── work_config
```

### Execução

```bash
# Passo 1: Setup base (mínimo funcional)
$ ruslink base --target ~ --merge --merge-append -v

# Output:
# Package     : base
# Stow dir    : /home/user/.dotfiles
# Target dir  : /home/user
# Merge Mode  : ENABLED
# Append Exts : [".bashrc", ".bash_profile", ".zshrc", ".profile", ".fishrc"]
# Linked: /home/user/.bashrc → ...
# Linked: /home/user/.zshrc → ...
# ✅ Done!

# Resultado:
~/.bashrc              → ~/.dotfiles/base/.bashrc ✅
~/.zshrc              → ~/.dotfiles/base/.zshrc ✅
~/.config/nvim/...    → ~/.dotfiles/base/.config/nvim/... ✅

# Passo 2: Adicionar dev tools
$ ruslink dev --target ~ --merge --merge-append -v

# Output:
# Merge Mode  : ENABLED
# ✓ Merged content from dev into /home/user/.bashrc
# ✓ Merged content from dev into /home/user/.zshrc
# Linked: /home/user/.cargo/config.toml → ...
# ✅ Done!

# Resultado em ~/.bashrc:
# (conteúdo de base/.bashrc)
#
# === ruslink [dev] ===
# export PATH="$HOME/.cargo/bin:$PATH"
# export RUST_BACKTRACE=1
# === ruslink [dev] (end) ===

# Passo 3: Adicionar GUI
$ ruslink gui --target ~ --merge --merge-append -v

# Output:
# ✓ Merged content from gui into /home/user/.bashrc
# Linked: /home/user/.config/i3/config → ...
# ✅ Done!

# Resultado final em ~/.bashrc:
# (conteúdo de base/.bashrc)
#
# === ruslink [dev] ===
# export PATH="$HOME/.cargo/bin:$PATH"
# === ruslink [dev] (end) ===
#
# === ruslink [gui] ===
# export GTK_SCALE=1.5
# === ruslink [gui] (end) ===

# Passo 4: Verificar histórico de merges
$ ruslink base --show-merge-history

# Output:
# 📋 Merge History (.ruslink-merge-log):
# [2025-05-14 10:23:45] Package: base | File: /home/user/.bashrc
# [2025-05-14 10:23:46] Package: base | File: /home/user/.zshrc
# [2025-05-14 10:24:12] Package: dev | File: /home/user/.bashrc
# [2025-05-14 10:24:13] Package: dev | File: /home/user/.zshrc
# [2025-05-14 10:25:01] Package: gui | File: /home/user/.bashrc
```

---

## Cenário 2: Conflito Detectado e Resolvido

### Estrutura

```
~/.dotfiles/
├── office/
│   └── .bashrc    # Contém Azure CLI setup
└── devops/
    └── .bashrc    # Contém AWS CLI + Kubernetes setup
```

### O que acontece

```bash
# Setup office
$ ruslink office --target ~ --merge --merge-append -y

# ~/.bashrc recebe:
# # === ruslink [office] ===
# export AZURE_SUBSCRIPTION=...
# # === ruslink [office] (end) ===

# Tentar adicionar devops SEM --merge (ERRO)
$ ruslink devops --target ~

# ❌ Error: Conflict: "/home/user/.bashrc" already exists
#    (use --force, --adopt, or --merge-append)

# Solução 1: Adicionar com merge
$ ruslink devops --target ~ --merge --merge-append -y

# ✓ Merged content from devops into /home/user/.bashrc
# ✅ Done!

# Resultado em ~/.bashrc:
# === ruslink [office] ===
# export AZURE_SUBSCRIPTION=...
# === ruslink [office] (end) ===
#
# === ruslink [devops] ===
# export AWS_PROFILE=default
# export KUBECONFIG=~/.kube/config
# === ruslink [devops] (end) ===

# Solução 2: Se preferir mesclar manualmente
$ ruslink devops --target ~ --merge --dry-run -v

# Avalia o que faria, depois você edita manualmente
$ vim ~/.bashrc
# (editar e fazer merge manual)
$ ruslink devops --delete --target ~  # Se decidir não usar
```

---

## Cenário 3: Trabalho + Casa

### Setup

```bash
# Máquina de trabalho
$ ruslink office --target ~ --merge --merge-append -y
$ ruslink work-vpn --target ~ --merge --merge-append -y
$ ruslink work-docker --target ~ --merge --merge-append -y

# Máquina pessoal
$ ruslink personal --target ~ --merge --merge-append -y
$ ruslink gaming --target ~ --merge --merge-append -y

# Máquina servidor
$ ruslink server-base --target ~ --merge --merge-append -y
$ ruslink monitoring --target ~ --merge --merge-append -y
$ ruslink backups --target ~ --merge --merge-append -y
```

### Histórico de merges em cada máquina

```bash
# Na máquina de trabalho:
$ ruslink office --show-merge-history
# [10:00:00] Package: office | File: ~/.bashrc
# [10:00:05] Package: office | File: ~/.zshrc
# [10:01:00] Package: work-vpn | File: ~/.bashrc
# [10:02:00] Package: work-docker | File: ~/.bashrc

# Na máquina pessoal:
$ ruslink personal --show-merge-history
# [14:00:00] Package: personal | File: ~/.bashrc
# [14:01:00] Package: gaming | File: ~/.bashrc

# No servidor:
$ ruslink server-base --show-merge-history
# [00:00:00] Package: server-base | File: ~/.bashrc
# [00:01:00] Package: monitoring | File: ~/.bashrc
# [00:02:00] Package: backups | File: ~/.bashrc
```

---

## Cenário 4: Update e Cleanup

### Atualizando um package

```bash
# Arquivo original antes
$ cat ~/.bashrc
# === ruslink [base] ===
# export FOO=old
# === ruslink [base] (end) ===
#
# === ruslink [dev] ===
# export BAR=dev
# === ruslink [dev] (end) ===

# Se você editar base/.bashrc para export FOO=new
# E rodar ruslink novamente:
$ ruslink base --restow --target ~ --merge --merge-append -y

# Result: ~/.bashrc agora tem:
# === ruslink [base] ===
# export FOO=new   # ← ATUALIZADO
# === ruslink [base] (end) ===
#
# === ruslink [dev] ===
# export BAR=dev
# === ruslink [dev] (end) ===

# Dev section permanece intacto ✅
```

### Remover um package

```bash
# Se você quer remover o package 'gui':
$ ruslink gui --delete --target ~ -y

# ❌ CUIDADO: --delete remove symlinks, NÃO toca em appends
# ~/.bashrc ainda tem:
# === ruslink [gui] ===
# ...
# === ruslink [gui] (end) ===

# Para limpar o append, opções:
# 1. Editar manualmente ~/.bashrc
# 2. Usar --force para sobrescrever:
$ ruslink base --target ~ --force --backup -y
# (Cria ~/.bashrc.bak e reconstrói do zero)
```

---

## Cenário 5: Dry-Run com Merge

### Preview de mudanças

```bash
# Ver o que vai acontecer SEM fazer nada
$ ruslink dev --target ~ --merge --merge-append --dry-run -v

# Output:
# DRY RUN MODE ENABLED
# DRY RUN: would append content from ~/.dotfiles/dev/.bashrc to ~/.bashrc
# DRY RUN: would append content from ~/.dotfiles/dev/.zshrc to ~/.zshrc
# DRY RUN: would link ~/.cargo/config.toml → ...
# Dry run completed. No changes were made.

# Se tudo parecer OK:
$ ruslink dev --target ~ --merge --merge-append -y
```

---

## Cenário 6: Custom Extensions

### Arquivos que você quer fazer merge

```bash
# Você tem packages que mexem em:
# - .gitconfig (git global config)
# - .ssh/config (SSH setup)
# - .config/fish/config.fish (Fish shell)

# Use extensões customizadas:
$ ruslink office --target ~ --merge --merge-append \
  --merge-extensions ".gitconfig,.ssh/config,.config/fish/config.fish" -y

# Resultado:
# .gitconfig terá merge de [office]
# .ssh/config terá merge de [office]
# .config/fish/config.fish terá merge de [office]
```

---

## Comparação: Com e Sem Merge Mode

### SEM Merge Mode ❌

```bash
$ ruslink base --target ~
# OK ✅

$ ruslink dev --target ~
# ❌ ERRO: ~/.bashrc já existe
# Você está preso. Opções ruins:
# - Editar manualmente .dotfiles/dev/.bashrc para não conflitar
# - Usar --force (perde base)
# - Usar --adopt (perde dev)
# - Executar script externo para mesclar
```

### COM Merge Mode ✅

```bash
$ ruslink base --target ~ --merge --merge-append -y
# OK ✅

$ ruslink dev --target ~ --merge --merge-append -y
# OK ✅ (inteligentemente mergeado)

$ ruslink gui --target ~ --merge --merge-append -y
# OK ✅ (três packages convivendo harmoniosamente)
```

---

## Checklist: Antes de Usar Merge Mode

```
☐ Entendi que --merge-append vai adicionar conteúdo, não sobrescrever
☐ Revisei quais extensões serão mergeadas
☐ Fiz backup de ~/.bashrc (ou rodei --dry-run primeiro)
☐ Tenho consciência de que marcadores [package] vão aparecer no arquivo
☐ Para remover um package, vou precisar editar manualmente ou usar --force
☐ Vou acompanhar .ruslink-merge-log para auditoria
```

---

**Conclusão:** Merge Mode torna possível usar ruslink com setups **reais e modernos** de dotfiles! 🎉
