FROM alpine:latest

ENTRYPOINT ["sh", "-c", "while true; do echo \"${PREFIX} Log message at $(date)\"; sleep 7; done"]
