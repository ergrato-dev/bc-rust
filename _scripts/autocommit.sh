#!/bin/bash
# ═══════════════════════════════════════════════════════════════
# 🦀 BOOTCAMP RUST - AUTO-COMMIT SCRIPT
# ═══════════════════════════════════════════════════════════════
# Runs every 5 minutes via cron
# Uses Conventional Commits format with What/For/Impact
# ═══════════════════════════════════════════════════════════════

set -e

# Configuration
REPO_DIR="/home/epti/Documents/epti-dev/bc-channel/bc-rust"
LOG_FILE="$REPO_DIR/_scripts/autocommit.log"
MAX_LOG_LINES=500

# Colors for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Timestamp
timestamp() {
    date "+%Y-%m-%d %H:%M:%S"
}

# Log function
log() {
    echo -e "[$(timestamp)] $1" | tee -a "$LOG_FILE"
}

# Rotate log if too large
rotate_log() {
    if [ -f "$LOG_FILE" ] && [ $(wc -l < "$LOG_FILE") -gt $MAX_LOG_LINES ]; then
        tail -n 200 "$LOG_FILE" > "$LOG_FILE.tmp"
        mv "$LOG_FILE.tmp" "$LOG_FILE"
        log "📋 Log rotated"
    fi
}

# Detect change type based on files modified
detect_commit_type() {
    local files="$1"
    
    if echo "$files" | grep -qE "\.md$"; then
        echo "docs"
    elif echo "$files" | grep -qE "\.rs$"; then
        if echo "$files" | grep -qE "test|spec"; then
            echo "test"
        else
            echo "feat"
        fi
    elif echo "$files" | grep -qE "Cargo\.(toml|lock)$"; then
        echo "build"
    elif echo "$files" | grep -qE "\.(json|yaml|yml|toml)$"; then
        echo "chore"
    elif echo "$files" | grep -qE "Dockerfile|docker-compose"; then
        echo "build"
    elif echo "$files" | grep -qE "\.svg$"; then
        echo "style"
    elif echo "$files" | grep -qE "\.sh$"; then
        echo "chore"
    else
        echo "chore"
    fi
}

# Detect scope based on files modified
detect_scope() {
    local files="$1"
    
    if echo "$files" | grep -qE "semana-[0-9]+"; then
        # Extract week number
        local week=$(echo "$files" | grep -oE "semana-[0-9]+" | head -1)
        echo "$week"
    elif echo "$files" | grep -qE "_docs/"; then
        echo "docs"
    elif echo "$files" | grep -qE "_assets/"; then
        echo "assets"
    elif echo "$files" | grep -qE "_scripts/"; then
        echo "scripts"
    elif echo "$files" | grep -qE "\.vscode/"; then
        echo "vscode"
    elif echo "$files" | grep -qE "\.devcontainer/"; then
        echo "docker"
    elif echo "$files" | grep -qE "\.github/"; then
        echo "github"
    else
        echo "bootcamp"
    fi
}

# Generate commit description (What?)
generate_what() {
    local files="$1"
    local file_count=$(echo "$files" | wc -l)
    
    if [ "$file_count" -eq 1 ]; then
        local filename=$(basename "$files")
        echo "update $filename"
    elif [ "$file_count" -le 3 ]; then
        echo "update $(echo "$files" | xargs -I {} basename {} | tr '\n' ', ' | sed 's/,$//')"
    else
        echo "update $file_count files"
    fi
}

# Generate commit purpose (For?)
generate_for() {
    local type="$1"
    local scope="$2"
    
    case "$type" in
        "docs")
            echo "improve documentation and learning materials"
            ;;
        "feat")
            echo "add new functionality for $scope"
            ;;
        "test")
            echo "ensure code quality and correctness"
            ;;
        "build")
            echo "improve build configuration and dependencies"
            ;;
        "style")
            echo "enhance visual presentation"
            ;;
        "chore")
            echo "maintain project structure"
            ;;
        *)
            echo "continue bootcamp development"
            ;;
    esac
}

# Generate impact description
generate_impact() {
    local type="$1"
    local file_count="$2"
    
    case "$type" in
        "docs")
            echo "📚 Better learning experience for students"
            ;;
        "feat")
            echo "🚀 New capabilities available in the bootcamp"
            ;;
        "test")
            echo "✅ Increased code reliability"
            ;;
        "build")
            echo "🔧 Improved development environment"
            ;;
        "style")
            echo "🎨 Enhanced visual materials"
            ;;
        *)
            echo "🦀 Bootcamp progress: $file_count file(s) updated"
            ;;
    esac
}

# Main function
main() {
    rotate_log
    log "🔄 Starting auto-commit check..."
    
    cd "$REPO_DIR" || {
        log "${RED}❌ Error: Cannot access repository directory${NC}"
        exit 1
    }
    
    # Check if there are changes (including untracked files)
    local has_changes=false
    local untracked=$(git ls-files --others --exclude-standard)
    
    if ! git diff --quiet || ! git diff --cached --quiet || [ -n "$untracked" ]; then
        has_changes=true
    fi
    
    if [ "$has_changes" = false ]; then
        log "✨ No changes detected"
        exit 0
    fi
    
    # Get list of changed files (including untracked)
    local changed_files=$(git diff --name-only)
    local staged_files=$(git diff --cached --name-only)
    local untracked_files=$(git ls-files --others --exclude-standard)
    local all_files=$(echo -e "$changed_files\n$staged_files\n$untracked_files" | sort -u | grep -v '^$')
    
    if [ -z "$all_files" ]; then
        log "✨ No files to commit"
        exit 0
    fi
    
    local file_count=$(echo "$all_files" | wc -l)
    
    # Detect commit metadata
    local commit_type=$(detect_commit_type "$all_files")
    local scope=$(detect_scope "$all_files")
    local what=$(generate_what "$all_files")
    local for_reason=$(generate_for "$commit_type" "$scope")
    local impact=$(generate_impact "$commit_type" "$file_count")
    
    # Build commit message (Conventional Commits format)
    local commit_title="$commit_type($scope): $what"
    local commit_body="What: $what
For: $for_reason
Impact: $impact

Auto-committed by bootcamp script
Files changed: $file_count"
    
    # Stage all changes
    git add -A
    
    # Commit with message
    git commit -m "$commit_title" -m "$commit_body"
    
    log "${GREEN}✅ Committed: $commit_title${NC}"
    log "   📁 Files: $file_count"
    log "   📝 Type: $commit_type | Scope: $scope"
    
    # Optional: Push to remote (uncomment if needed)
    # git push origin main 2>/dev/null && log "🚀 Pushed to remote" || log "⚠️ Push failed or no remote"
    
    log "─────────────────────────────────────────"
}

# Run main function
main "$@"
