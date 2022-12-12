rm -r dist docs
trunk build --release
cp -r dist docs
echo "joshuacarrasco.com" | tee > docs/CNAME
cp docs/index.html docs/404.html
