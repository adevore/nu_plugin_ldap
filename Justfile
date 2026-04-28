default:
    just --list

upgrade-nu:
    cargo upgrade --incompatible \
        --package nu-plugin \
        --package nu-protocol

upgrade-nu-dry-run:
    cargo upgrade --dry-run --incompatible \
        --package nu-plugin \
        --package nu-protocol
