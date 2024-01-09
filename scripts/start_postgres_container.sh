#! /bin/bash

read -p "Enter container name: " CONT_NAME

read -rep $'Start or stop container:\n1-start\n2-stop\n' METHOD

if [[ $METHOD = "1" ]]; then 
    docker start ${CONT_NAME}
    # echo "start"
elif [[ $METHOD = "2" ]]; then
    docker stop ${CONT_NAME}
    # echo "stop"
fi