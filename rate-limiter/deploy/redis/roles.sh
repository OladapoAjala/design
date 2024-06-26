#!/bin/bash
# Usage: ./roles.sh

kubectl version

urls=$(kubectl get pods -l app=redis -o jsonpath='{range.items[*]}{.status.podIP} ')
command="kubectl exec -it redis-0 -- redis-cli --cluster create "

for url in $urls
do
    command+=$url":6379 "
done

echo "Executing command: " $command
$command