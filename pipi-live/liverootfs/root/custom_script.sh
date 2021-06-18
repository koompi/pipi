#!/bin/bash
USER=koompi
PASSWORD=koompi

useradd -m -G users,wheel,audio,video -s /bin/bash $USER
passwd -d $USER &>/dev/null
passwd -d root &>/dev/null

echo "root:root" | chpasswd -c SHA512
echo "$USER:$PASSWORD" | chpasswd -c SHA512

echo "koompilive" > /etc/hostname

for sv in dhcpcd sysklogd gpm dbus lxdm networkmanager; do
	ln -svf /etc/sv/$sv /var/service
done
