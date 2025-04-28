# This script installs the specified version of the action from GitHub releases.
set -e

echo "GitHub Action Repository :: $ACTION_REPOSITORY (v${VERSION})"
echo "Runner :: $RUNNER_OS ($RUNNER_ARCH)"

gh release download \
    "v${VERSION}" \
    --repo "https://github.com/$ACTION_REPOSITORY" \
    --pattern "${BINARY_NAME}-${RUNNER_OS}-${RUNNER_ARCH}.tar.gz" --clobber \
    --output "/tmp/${RUNNER_OS}-${RUNNER_ARCH}.tar.gz"

tar -xzf "/tmp/${RUNNER_OS}-${RUNNER_ARCH}.tar.gz" -C "/tmp"

if [ -f "/tmp/${BINARY_NAME}" ]; then
    mv "/tmp/${BINARY_NAME}" /usr/local/bin/
    chmod +x "/usr/local/bin/${BINARY_NAME}"
else
    echo "Error: Action not found in the downloaded archive."
    exit 1
fi

echo "GitHub Action installed successfully."

