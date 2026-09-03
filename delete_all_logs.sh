#!/bin/bash
#
# Acessar a página de tokens clássicos
# Vá para: https://github.com/settings/tokens
#
# NÃO clique em "Generate new token (fine-grained)"
#
# Clique em "Generate new token (classic)" (no final da página)
#
# Note: delete-runs
# Expiration: No expiration (ou 90 dias)
#
# Selecione os escopos:
# ☑️ repo (TUDO - vai marcar todos automaticamente)
#    ☑️ repo:status
#    ☑️ repo_deployment  
#    ☑️ public_repo
#    ☑️ repo:invite
#    ☑️ security_events
# ☑️ workflow  <-- ESSENCIAL!
# ☑️ admin:repo_hook
# ☑️ delete_repo (opcional)
#

# Função para detectar repositório do .git
detect_repo() {
    # Verifica se está em um repositório git
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        echo "❌ Erro: Você não está em um repositório git."
        echo "Execute este script dentro de um repositório git."
        exit 1
    fi
    
    # Pega a URL do remote
    local url=$(git config --get remote.origin.url 2>/dev/null)
    
    # Se não tiver origin, tenta o primeiro remote
    if [ -z "$url" ]; then
        local first_remote=$(git remote 2>/dev/null | head -n1)
        if [ -n "$first_remote" ]; then
            url=$(git config --get "remote.$first_remote.url")
        else
            echo "❌ Erro: Nenhum remote configurado neste repositório."
            echo "Configure um remote com: git remote add origin <url>"
            exit 1
        fi
    fi
    
    # Extrai usuario/repositorio
    local repo=""
    if [[ "$url" =~ git@[^:]+:(.+).git$ ]]; then
        repo="${BASH_REMATCH[1]}"
    elif [[ "$url" =~ https?://[^/]+/(.+).git$ ]]; then
        repo="${BASH_REMATCH[1]}"
    elif [[ "$url" =~ git@[^:]+:(.+)$ ]]; then
        repo="${BASH_REMATCH[1]}"
    elif [[ "$url" =~ https?://[^/]+/(.+)$ ]]; then
        repo="${BASH_REMATCH[1]}"
    elif [[ "$url" =~ ssh://[^/]+/(.+).git$ ]]; then
        repo="${BASH_REMATCH[1]}"
    else
        repo=$(echo "$url" | sed 's/.*github\.com[:/]//;s/\.git$//')
    fi
    
    repo=$(echo "$repo" | sed 's/\/$//')
    
    if [ -z "$repo" ]; then
        echo "❌ Erro: Não foi possível extrair o nome do repositório da URL: $url"
        exit 1
    fi
    
    echo "$repo"
}

# Detecta o repositório atual
REPO=$(detect_repo)

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║     🔥 DELETADOR MASSIVO DE RUNS                        ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "📁 Repositório: $REPO"
echo ""

# Verifica se o token foi informado como argumento ou variável de ambiente
if [ -n "$1" ]; then
    TOKEN="$1"
elif [ -n "$GITHUB_TOKEN" ]; then
    TOKEN="$GITHUB_TOKEN"
else
    echo "🔑 Informe seu GitHub Token (Classic):"
    echo "   (Crie em: https://github.com/settings/tokens)"
    echo "   Escopos necessários: repo, workflow"
    echo ""
    read -s -p "Token: " TOKEN
    echo ""
    echo ""
fi

if [ -z "$TOKEN" ]; then
    echo "❌ Token não informado!"
    exit 1
fi

# Testa o token
echo "🔍 Testando token..."
RESPONSE=$(curl -s -H "Authorization: token $TOKEN" \
    "https://api.github.com/repos/$REPO" 2>/dev/null)

if echo "$RESPONSE" | grep -q "Bad credentials"; then
    echo "❌ Token inválido! Certifique-se de que é um token CLASSIC."
    echo "   Escopos necessários: repo, workflow"
    exit 1
fi

echo "✅ Token válido!"
echo ""

# Busca TODOS os runs
echo "🔍 Buscando todos os runs..."
echo ""

PAGE=1
ALL_IDS=()

while true; do
    echo "  📄 Buscando página $PAGE..."
    
    RESPONSE=$(curl -s -H "Authorization: token $TOKEN" \
        "https://api.github.com/repos/$REPO/actions/runs?per_page=100&page=$PAGE" 2>/dev/null)
    
    # Verifica se a resposta é válida
    if echo "$RESPONSE" | grep -q "API rate limit"; then
        echo "  ⏳ Rate limit atingido. Aguardando 60 segundos..."
        sleep 60
        continue
    fi
    
    IDS=$(echo "$RESPONSE" | jq -r '.workflow_runs[].id' 2>/dev/null)
    
    if [ -z "$IDS" ] || [ "$IDS" = "null" ] || [ "$IDS" = "" ]; then
        break
    fi
    
    COUNT=0
    while IFS= read -r id; do
        if [ -n "$id" ] && [ "$id" != "null" ]; then
            ALL_IDS+=("$id")
            COUNT=$((COUNT + 1))
        fi
    done <<< "$IDS"
    
    echo "  ✅ Encontrados $COUNT runs nesta página"
    
    # Verifica se tem mais páginas
    TOTAL_COUNT=$(echo "$RESPONSE" | jq -r '.total_count' 2>/dev/null)
    if [ -n "$TOTAL_COUNT" ] && [ ${#ALL_IDS[@]} -ge "$TOTAL_COUNT" ]; then
        break
    fi
    
    PAGE=$((PAGE + 1))
    
    # Limite de segurança (evita loop infinito)
    if [ $PAGE -gt 50 ]; then
        break
    fi
done

TOTAL=${#ALL_IDS[@]}

echo ""
echo "📊 TOTAL encontrados: $TOTAL runs"
echo ""

if [ "$TOTAL" -eq 0 ]; then
    echo "✅ Nenhum run encontrado!"
    exit 0
fi

echo "📋 Primeiros 5 runs:"
for i in {0..4}; do
    if [ $i -lt $TOTAL ]; then
        echo "  Run ${ALL_IDS[$i]}"
    fi
done
echo ""

read -p "⚠️  Deletar TODOS os $TOTAL runs? (digite 'SIM' para confirmar): " CONFIRMACAO

if [ "$CONFIRMACAO" != "SIM" ]; then
    echo "❌ Cancelado."
    exit 0
fi

echo ""
echo "🚀 Deletando em lotes de 20..."
echo ""

SUCESSOS=0
FALHAS=0
JA_DELETADOS=0
CONTADOR=0

for id in "${ALL_IDS[@]}"; do
    CONTADOR=$((CONTADOR + 1))
    PERCENT=$((CONTADOR * 100 / TOTAL))
    
    echo -n "[$PERCENT%] $CONTADOR/$TOTAL - Deletando $id ... "
    
    STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE \
        -H "Authorization: token $TOKEN" \
        "https://api.github.com/repos/$REPO/actions/runs/$id" 2>/dev/null)
    
    if [ "$STATUS" = "204" ]; then
        echo "✅"
        SUCESSOS=$((SUCESSOS + 1))
    elif [ "$STATUS" = "404" ]; then
        echo "⏭️  (já deletado)"
        JA_DELETADOS=$((JA_DELETADOS + 1))
    else
        echo "❌ (HTTP $STATUS)"
        FALHAS=$((FALHAS + 1))
    fi
    
    # Pausa para não sobrecarregar
    sleep 0.05
done

echo ""
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                     📊 RESUMO FINAL                      ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "  ✅ Deletados com sucesso: $SUCESSOS"
if [ $JA_DELETADOS -gt 0 ]; then
    echo "  ⏭️  Já estavam deletados: $JA_DELETADOS"
fi
if [ $FALHAS -gt 0 ]; then
    echo "  ❌ Falhas: $FALHAS"
fi
echo ""

# Verifica se ainda há runs
echo "🔍 Verificando runs restantes..."
REMAINING=$(curl -s -H "Authorization: token $TOKEN" \
    "https://api.github.com/repos/$REPO/actions/runs?per_page=1" 2>/dev/null | \
    jq -r '.total_count' 2>/dev/null)

if [ -n "$REMAINING" ] && [ "$REMAINING" -gt 0 ]; then
    echo "⚠️  Ainda há $REMAINING runs (pode ser cache do GitHub)"
    echo "   Aguarde 1 minuto e execute novamente"
else
    echo "🎉 MISSÃO CUMPRIDA! Todos os runs foram deletados!"
fi

echo ""
echo "💡 Dica: Para usar em outros repositórios, execute dentro da pasta do projeto"
echo "   Ou passe o token como argumento: $0 SEU_TOKEN"
