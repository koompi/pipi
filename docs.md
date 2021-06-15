# Setup The Tool
1. chmod +x build.sh
2. ./build.sh
3. change store.conf in etc to 0.0.0.0:3690
4. sudo bin-repo create /var/www/core/core.db
5. sudo server -g &
6. to build packages use: pipi -b
7. to add new package in repo: pipi -a *.app
8. to remove package in repo: pipi -r package name(without extension)
9. to update the db use: pipi -u



#  How to kill port (server)
1. ps -ef
2. sudo kill -9 PID
