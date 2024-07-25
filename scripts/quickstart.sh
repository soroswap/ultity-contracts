#!/bin/bash

previewHash=$(jq -r '.previewHash' configs.json)
quickstartHash=$(jq -r '.quickstartHash' configs.json)
protocolVersion=$(jq -r '.protocolVersion' configs.json)

previewVersion=$(echo "$previewHash" | cut -d'@' -f1)
echo $previewVersion

set -e

case "$1" in
standalone)
    echo "Using standalone network"
    ARGS="--local --enable-soroban-diagnostic-events"
    ;;
futurenet)
    echo "Using Futurenet network"
    ARGS="--futurenet"
    ;;
testnet)
    echo "Using Testnet network"
    ARGS="--testnet"
    ;;
*)
    echo "Usage: $0 standalone|futurenet|testnet"
    exit 1
    ;;
esac

shift

echo "1. Creating docker soroban network"
(docker network inspect soroban-network -f '{{.Id}}' 2>/dev/null) \
  || docker network create soroban-network

echo "  "
echo "  "

previewContainerName="soroban-preview-${previewVersion}"
echo "2. Searching for a previous ${previewContainerName} container"
containerID=$(docker ps --filter=`name=${previewContainerName}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing ${previewContainerName}  container."
    docker rm --force ${previewContainerName}
    echo "Finished removing ${previewContainerName} container."
else
    echo "No previous ${previewContainerName} container was found"
fi
echo "  "
echo "  "

stellarContainerName="stellar"
echo "3. Searching for a previous ${stellarContainerName} container"
containerID=$(docker ps --filter=`name=${stellarContainerName}` --all --quiet)
if [[ ${containerID} ]]; then
    echo "Start removing ${stellarContainerName} container."
    docker rm --force ${stellarContainerName}
    echo "Finished removing ${stellarContainerName} container."
else
    echo "No previous ${stellarContainerName} container was found"
fi
echo "  "
echo "  "

echo "4. Run a ${previewContainerName} container"

currentDir=$(pwd)
docker run -dti \
  --volume ${currentDir}:/workspace \
  --name ${previewContainerName} \
  -p 8002:8000 \
  --ipc=host \
  --network soroban-network \
  esteblock/soroban-preview:${previewHash}

echo "  "
echo "  "

echo "5. Run a ${stellarContainerName} quickstart container"
# Run the ${stellarContainerName} quickstart image
docker run --rm -ti \
  --name ${stellarContainerName} \
  --network soroban-network \
  -p 8000:8000 \
  stellar/quickstart:${quickstartHash} \
  $ARGS \
  --enable-soroban-rpc \
  --protocol-version ${protocolVersion} \
  --enable-soroban-diagnostic-events \
  "$@" # Pass through args from the CLI
