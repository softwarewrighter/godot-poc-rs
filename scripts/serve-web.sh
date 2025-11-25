#!/usr/bin/env bash
#
# serve-web.sh - Serve the web export locally for testing
#
# This script starts a local web server with proper COOP/COEP headers
# required for SharedArrayBuffer support (needed for Godot threading).
#
# Usage:
#   ./scripts/serve-web.sh          # Start server on port 8080
#   ./scripts/serve-web.sh 3000     # Start server on port 3000

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
EXPORT_DIR="$PROJECT_ROOT/export/web"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Check if export exists
check_export() {
    if [[ ! -f "$EXPORT_DIR/index.html" ]]; then
        error "Web export not found at $EXPORT_DIR/index.html"
        echo ""
        echo "Run ./scripts/export-to-web.sh first to create the web export."
        exit 1
    fi
}

# Create Python server script with COOP/COEP headers
create_server_script() {
    cat > "$EXPORT_DIR/serve.py" << 'PYTHON_SCRIPT'
#!/usr/bin/env python3
"""
Simple HTTP server with COOP/COEP headers for Godot web exports.

These headers are required for SharedArrayBuffer support, which is
needed for Godot's threading features in web exports.
"""

import http.server
import socketserver
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8080

class CORSRequestHandler(http.server.SimpleHTTPRequestHandler):
    """HTTP handler that adds required headers for Godot web exports."""

    def end_headers(self):
        # Required for SharedArrayBuffer (threading support)
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Resource-Policy', 'cross-origin')

        # Cache control for development
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')

        super().end_headers()

    def log_message(self, format, *args):
        """Custom log format."""
        print(f"[{self.log_date_time_string()}] {format % args}")

def main():
    handler = CORSRequestHandler

    with socketserver.TCPServer(("", PORT), handler) as httpd:
        print(f"\n{'='*60}")
        print(f"  Godot Web Export Server")
        print(f"{'='*60}")
        print(f"\n  URL: http://localhost:{PORT}")
        print(f"\n  Headers enabled:")
        print(f"    - Cross-Origin-Opener-Policy: same-origin")
        print(f"    - Cross-Origin-Embedder-Policy: require-corp")
        print(f"\n  Press Ctrl+C to stop\n")
        print(f"{'='*60}\n")

        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n\nServer stopped.")
            sys.exit(0)

if __name__ == "__main__":
    main()
PYTHON_SCRIPT
}

# Serve using Python (built-in, available everywhere)
serve_python() {
    local port="${1:-8080}"

    create_server_script

    info "Starting Python HTTP server on port $port..."
    info ""
    echo -e "  ${CYAN}Open in browser:${NC} http://localhost:$port"
    info ""
    info "Press Ctrl+C to stop"
    info ""

    cd "$EXPORT_DIR"
    python3 serve.py "$port"
}

# Serve using Rust's miniserve (if available, better performance)
serve_miniserve() {
    local port="${1:-8080}"

    info "Starting miniserve on port $port..."
    info ""
    echo -e "  ${CYAN}Open in browser:${NC} http://localhost:$port"
    info ""
    info "Press Ctrl+C to stop"
    info ""

    cd "$EXPORT_DIR"
    miniserve . \
        --port "$port" \
        --header "Cross-Origin-Opener-Policy: same-origin" \
        --header "Cross-Origin-Embedder-Policy: require-corp" \
        --header "Cross-Origin-Resource-Policy: cross-origin"
}

# Main
main() {
    local port="${1:-8080}"

    # Show help
    if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
        echo "Usage: $0 [port]"
        echo ""
        echo "Starts a local web server for the Godot web export."
        echo "Default port is 8080."
        echo ""
        echo "The server includes COOP/COEP headers required for"
        echo "SharedArrayBuffer support (Godot threading)."
        echo ""
        echo "Options:"
        echo "  port    Port number (default: 8080)"
        echo ""
        echo "Examples:"
        echo "  $0          # Start on port 8080"
        echo "  $0 3000     # Start on port 3000"
        exit 0
    fi

    check_export

    # Prefer miniserve if available (Rust-based, faster)
    if command -v miniserve &> /dev/null; then
        serve_miniserve "$port"
    # Fall back to Python (always available)
    elif command -v python3 &> /dev/null; then
        serve_python "$port"
    else
        error "No suitable server found. Install Python 3 or miniserve (cargo install miniserve)."
    fi
}

main "$@"
