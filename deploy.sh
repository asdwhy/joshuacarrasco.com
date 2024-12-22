#!/usr/bin/env sh

rm -r public
hugo
rsync -avz --delete public/ jecs-ovh:~/webroot/joshuacarrasco.com
rsync -avz --delete public/ jecs-ovh:~/webroot/jecs.me
