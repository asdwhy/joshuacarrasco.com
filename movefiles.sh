mv docs/CNAME ./CNAMEtmp;
sudo rm -rf docs/;
mkdir docs
mv dist/jecs/* docs/
rm -rf dist/
mv CNAMEtmp docs/CNAME;
cp docs/index.html docs/404.html;
