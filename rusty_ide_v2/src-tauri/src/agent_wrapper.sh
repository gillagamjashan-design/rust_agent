#!/bin/bash

#############################################################################
# Rusty IDE Agent Wrapper
#
# This script provides a file-based communication bridge between Rusty IDE
# and the external agent process. It watches for request.json, processes
# requests with the agent, and writes response.json.
#
# Usage:
#   ./agent_wrapper.sh [--daemon] [--agent-path PATH]
#
# Options:
#   --daemon        Run in daemon mode (background, continuous monitoring)
#   --agent-path    Path to the agent executable (default: uses rust_agent)
#   --timeout       Timeout for agent processing in seconds (default: 120)
#   --help          Show this help message
#
# Environment:
#   RUSTY_AGENT_DIR    Directory for request/response files (default: ~/.rusty/agent)
#   RUST_AGENT_PATH    Path to rust_agent executable
#############################################################################

set -euo pipefail

# Configuration
AGENT_DIR="${RUSTY_AGENT_DIR:-$HOME/.rusty/agent}"
REQUEST_FILE="$AGENT_DIR/request.json"
RESPONSE_FILE="$AGENT_DIR/response.json"
PROCESSING_FLAG="$AGENT_DIR/.processing"
LOG_FILE="$AGENT_DIR/wrapper.log"

# Default settings
DAEMON_MODE=false
AGENT_TIMEOUT=120
AGENT_PATH="${RUST_AGENT_PATH:-rust_agent}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

#############################################################################
# Logging functions
#############################################################################

log() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$LOG_FILE"
}

log_info() {
    log "INFO" "$@"
}

log_error() {
    log "ERROR" "$@"
}

log_debug() {
    log "DEBUG" "$@"
}

#############################################################################
# Utility functions
#############################################################################

# Check if required commands are available
check_dependencies() {
    local missing_deps=()

    if ! command -v jq &> /dev/null; then
        missing_deps+=("jq")
    fi

    if ! command -v inotifywait &> /dev/null; then
        missing_deps+=("inotify-tools")
    fi

    if [ ${#missing_deps[@]} -gt 0 ]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        log_info "Install with: sudo apt-get install ${missing_deps[*]}"
        exit 1
    fi
}

# Check if agent is available
check_agent() {
    if ! command -v "$AGENT_PATH" &> /dev/null; then
        log_error "Agent not found: $AGENT_PATH"
        log_info "Please install rust_agent or set RUST_AGENT_PATH"
        return 1
    fi

    log_info "Using agent: $AGENT_PATH"
    return 0
}

# Create necessary directories
setup_environment() {
    if [ ! -d "$AGENT_DIR" ]; then
        log_info "Creating agent directory: $AGENT_DIR"
        mkdir -p "$AGENT_DIR"
    fi

    # Clean up any stale processing flags
    if [ -f "$PROCESSING_FLAG" ]; then
        log_debug "Removing stale processing flag"
        rm -f "$PROCESSING_FLAG"
    fi
}

#############################################################################
# Request processing
#############################################################################

# Process a single request
process_request() {
    local request_file="$1"

    log_info "Processing request: $request_file"

    # Create processing flag
    touch "$PROCESSING_FLAG"

    # Extract data from request
    local query=$(jq -r '.query' "$request_file")
    local workspace=$(jq -r '.workspace_path' "$request_file")
    local current_file=$(jq -r '.current_file // ""' "$request_file")

    log_debug "Query: $query"
    log_debug "Workspace: $workspace"

    # Prepare agent command
    local agent_cmd="$AGENT_PATH"
    local agent_args=()

    # Add workspace context if available
    if [ -n "$workspace" ] && [ "$workspace" != "null" ]; then
        cd "$workspace" 2>/dev/null || log_error "Failed to change to workspace: $workspace"
    fi

    # Build the agent prompt
    local prompt="$query"

    if [ -n "$current_file" ] && [ "$current_file" != "null" ]; then
        local current_code=$(jq -r '.current_code // ""' "$request_file")
        prompt="Context: Working on file $current_file\n\nCurrent code:\n$current_code\n\nTask: $query"
    fi

    # Create temporary file for agent output
    local temp_output=$(mktemp)

    # Run agent with timeout
    log_info "Running agent (timeout: ${AGENT_TIMEOUT}s)..."

    local agent_exit_code=0
    if timeout "$AGENT_TIMEOUT" "$agent_cmd" -p "$prompt" > "$temp_output" 2>&1; then
        log_info "Agent completed successfully"
    else
        agent_exit_code=$?
        if [ $agent_exit_code -eq 124 ]; then
            log_error "Agent timed out after ${AGENT_TIMEOUT}s"
            echo "Error: Agent processing timed out" > "$temp_output"
        else
            log_error "Agent failed with exit code: $agent_exit_code"
        fi
    fi

    # Parse agent output and create response
    create_response "$temp_output" "$agent_exit_code"

    # Clean up
    rm -f "$temp_output"
    rm -f "$PROCESSING_FLAG"

    log_info "Request processing complete"
}

# Create response JSON from agent output
create_response() {
    local output_file="$1"
    local exit_code="$2"

    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%S.%3NZ")
    local response_text=$(cat "$output_file")

    # Try to extract code blocks from the response
    local code_suggestions="[]"

    # Simple code block detection (```language ... ```)
    if grep -q '```' "$output_file"; then
        log_debug "Detected code blocks in response"

        # Extract code blocks (this is a simplified parser)
        local temp_suggestions=$(mktemp)
        echo "[]" > "$temp_suggestions"

        # This is a basic implementation - a more robust parser would be better
        while IFS= read -r line; do
            if [[ "$line" =~ ^\`\`\`([a-z]+)$ ]]; then
                local lang="${BASH_REMATCH[1]}"
                local code=""
                local in_code=true

                while $in_code && IFS= read -r code_line; do
                    if [[ "$code_line" =~ ^\`\`\`$ ]]; then
                        in_code=false

                        # Add suggestion to array
                        local suggestion=$(jq -n \
                            --arg file "${current_file:-unknown}" \
                            --arg code "$code" \
                            --arg lang "$lang" \
                            --arg desc "Code suggestion" \
                            '{file: $file, code: $code, language: $lang, description: $desc}')

                        jq --argjson item "$suggestion" '. += [$item]' "$temp_suggestions" > "${temp_suggestions}.tmp"
                        mv "${temp_suggestions}.tmp" "$temp_suggestions"
                    else
                        code="${code}${code_line}\n"
                    fi
                done
            fi
        done < "$output_file"

        code_suggestions=$(cat "$temp_suggestions")
        rm -f "$temp_suggestions"
    fi

    # Determine if changes should be auto-applied
    local apply_changes=false
    if [ "$exit_code" -eq 0 ] && [ "$code_suggestions" != "[]" ]; then
        apply_changes=true
    fi

    # Create response JSON
    jq -n \
        --arg timestamp "$timestamp" \
        --arg response "$response_text" \
        --argjson suggestions "$code_suggestions" \
        --argjson apply "$apply_changes" \
        '{
            timestamp: $timestamp,
            response_text: $response,
            code_suggestions: $suggestions,
            apply_changes: $apply
        }' > "$RESPONSE_FILE"

    log_info "Response written to: $RESPONSE_FILE"
}

#############################################################################
# Monitoring modes
#############################################################################

# Process a single request and exit
process_once() {
    if [ ! -f "$REQUEST_FILE" ]; then
        log_error "No request file found: $REQUEST_FILE"
        exit 1
    fi

    process_request "$REQUEST_FILE"

    # Clean up request file
    rm -f "$REQUEST_FILE"
}

# Run in daemon mode (continuous monitoring)
run_daemon() {
    log_info "Starting agent wrapper in daemon mode"
    log_info "Monitoring directory: $AGENT_DIR"
    log_info "Press Ctrl+C to stop"

    # Set up signal handlers
    trap 'log_info "Shutting down..."; rm -f "$PROCESSING_FLAG"; exit 0' INT TERM

    while true; do
        # Wait for request file to be created or modified
        if [ -f "$REQUEST_FILE" ]; then
            # Check if we're not already processing
            if [ ! -f "$PROCESSING_FLAG" ]; then
                process_request "$REQUEST_FILE"

                # Clean up request file after processing
                rm -f "$REQUEST_FILE"
            else
                log_debug "Skipping request - already processing"
            fi
        fi

        # Use inotifywait for efficient file monitoring
        inotifywait -q -e create -e modify --timeout 1 "$AGENT_DIR" 2>/dev/null || true

        # Brief sleep to avoid tight loop
        sleep 0.1
    done
}

#############################################################################
# Main
#############################################################################

show_help() {
    cat << EOF
Rusty IDE Agent Wrapper

Usage:
    $0 [OPTIONS]

Options:
    --daemon            Run in daemon mode (continuous monitoring)
    --agent-path PATH   Path to agent executable (default: rust_agent)
    --timeout SECONDS   Timeout for agent processing (default: 120)
    --help              Show this help message

Environment Variables:
    RUSTY_AGENT_DIR     Directory for request/response files (default: ~/.rusty/agent)
    RUST_AGENT_PATH     Path to rust_agent executable

Examples:
    # Process a single request
    $0

    # Run as daemon
    $0 --daemon

    # Use custom agent path
    $0 --daemon --agent-path /usr/local/bin/my_agent

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --daemon)
                DAEMON_MODE=true
                shift
                ;;
            --agent-path)
                AGENT_PATH="$2"
                shift 2
                ;;
            --timeout)
                AGENT_TIMEOUT="$2"
                shift 2
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

main() {
    parse_args "$@"

    log_info "=== Rusty IDE Agent Wrapper ==="
    log_info "Agent directory: $AGENT_DIR"
    log_info "Log file: $LOG_FILE"

    # Setup
    check_dependencies
    setup_environment

    # Check agent availability
    if ! check_agent; then
        exit 1
    fi

    # Run appropriate mode
    if [ "$DAEMON_MODE" = true ]; then
        run_daemon
    else
        process_once
    fi
}

# Run main function with all arguments
main "$@"
