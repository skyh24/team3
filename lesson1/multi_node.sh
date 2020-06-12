
./target/release/node-template \
  --base-path /tmp/alice \
  --chain=local \
  --alice \
  --port 30333 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator \
  --name AlicesNode 

./target/release/node-template \
  --base-path /tmp/bob \
  --chain=local \
  --bob \
  --port 30334 \
  --telemetry-url 'ws://telemetry.polkadot.io:1024 0' \
  --validator \
  --name BobsNode \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWNTF4DyJ9ax5JCwhoMip3XCykgzR6ckgDPgL3Nu5oAGBe