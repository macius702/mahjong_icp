pushd src/mahjong_icp_frontend
flutter build web
sed -i 's|<base href="/ED-Mahjong/">|<base href="">|g' build/web/index.html
popd
dfx deploy