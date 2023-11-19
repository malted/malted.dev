#!/usr/bin/env bash

# USAGE: serversetup.sh <NEW_USERNAME> <IP>

DESIRED_REMOTE_USERNAME=$1
EXISTING_REMOTE_USER=root # Assumes you already have a key for for root on your machine.
REMOTE_IP=$2
KEY_NAME=id_ed25519_$(echo $REMOTE_IP | tr . -)
LOCAL_KEY_LOCATION=$HOME/.ssh/$KEY_NAME

ssh-keygen -t ed25519 -a 128 -b 4096 -f $LOCAL_KEY_LOCATION -q -N "" -C "$DESIRED_REMOTE_USERNAME's key for $REMOTE_IP generated on $(date) on $(hostname -s)"
ssh-add $LOCAL_KEY_LOCATION

PUB_KEY=$(cat $LOCAL_KEY_LOCATION.pub)

REMOTE_COMMANDS="
sed -i '/^#PubkeyAuthentication\|PubkeyAuthentication/s/^.*$/PubkeyAuthentication yes/' /etc/ssh/sshd_config &&
sed -i '/^#PermitRootLogin\|PermitRootLogin/s/^.*$/PermitRootLogin no/' /etc/ssh/sshd_config &&
useradd -m -d /home/$DESIRED_REMOTE_USERNAME -s /bin/bash $DESIRED_REMOTE_USERNAME &&
mkdir -p /home/$DESIRED_REMOTE_USERNAME/.ssh &&
touch /home/$DESIRED_REMOTE_USERNAME/.ssh/authorized_keys &&
echo \"$PUB_KEY\" >> /home/$DESIRED_REMOTE_USERNAME/.ssh/authorized_keys &&
chown -R $DESIRED_REMOTE_USERNAME:$DESIRED_REMOTE_USERNAME /home/$DESIRED_REMOTE_USERNAME/.ssh &&
chmod 700 /home/$DESIRED_REMOTE_USERNAME/.ssh &&
chmod 600 /home/$DESIRED_REMOTE_USERNAME/.ssh/authorized_keys &&
echo '$DESIRED_REMOTE_USERNAME ALL=NOPASSWD: ALL' | EDITOR='tee -a' visudo &&
systemctl restart ssh
"

ssh $EXISTING_REMOTE_USER@$REMOTE_IP $REMOTE_COMMANDS

printf "Host %s\n\tUser %s\n\tIdentityFile %s\n\n" $REMOTE_IP $DESIRED_REMOTE_USERNAME $LOCAL_KEY_LOCATION >> $HOME/.ssh/config

ssh $DESIRED_REMOTE_USERNAME@$REMOTE_IP
